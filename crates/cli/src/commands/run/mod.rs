use std::env;

use colored::Colorize;
use eyre::Result;
use relm4_macros::view;
use rust_i18n::t;
use serde_value::Value;
use tokio::process::Command;

mod list;
mod util;

use custard_util::user_error;
use package_json::PackageJson;
use util::{get_node_modules_bin_dir, scripts_field_not_found};

use crate::consts::{LIFECYCLE_SCRIPTS, SCRIPT_SIGNAL_EXIT_CODE};

pub async fn invoke(
    script_name: Option<String>,
    args: Option<Vec<String>>,
    run_with_lifecycle_cmd: bool,
) -> Result<()> {
    let args = args.unwrap_or_default();
    let package_json = PackageJson::from_package_json_file().await?;

    if let Some(script_name) = script_name {
        if !run_with_lifecycle_cmd && LIFECYCLE_SCRIPTS.contains(&script_name.as_str()) {
            eprintln!(
                "{} {}",
                "tip:".yellow().bold(),
                &t!("run-with-lifecycle-script-command", name = &script_name)
                    .black()
                    .bold()
            );
        }

        if let Some(ref scripts) = package_json.scripts {
            if let Some(script_content) = scripts.get(&script_name) {
                let pre_script_name = format!("pre{script_name}");
                let post_script_name = format!("post{script_name}");

                // Run prescript...
                if let Some(script_content) = scripts.get(&pre_script_name) {
                    run_script(&pre_script_name, script_content, &[], &package_json).await?;
                }

                // ...then the script itself...
                run_script(&script_name, script_content, &args, &package_json).await?;

                // ...and finally the postscript
                if let Some(script_content) = scripts.get(&post_script_name) {
                    run_script(&post_script_name, script_content, &[], &package_json).await?;
                }
            } else {
                user_error(
                    t!("script-not-found", name = &script_name),
                    exitcode::CONFIG,
                );
            }
        } else {
            scripts_field_not_found();
        }
    } else {
        list::scripts().await?;
    }

    Ok(())
}

async fn run_script(
    name: &str,
    content: &str,
    args: &[String],
    package_json: &PackageJson,
) -> Result<()> {
    let bin_dir = get_node_modules_bin_dir()?;
    let content = format!("{content} {}", args.join(" "));

    println!("{} script `{name}`", "Running".green().bold());
    println!("\n{}", format!("> {content}").bold());

    // Get the system specific shell, and the flag to make it quiet
    let shell = if cfg!(windows) { "cmd.exe" } else { "/bin/sh" };
    let quiet_flag = if cfg!(windows) { "/C" } else { "-c" };

    // Get the "real" PATH environment variable.
    let system_path = env::var("PATH")?;

    /*
    This looks like this:
    npm_name => "name",
    npm_version => "1.2.3"
    */
    let flat_btree = serde_value_flatten::to_flatten_maptree("_", Some("npm_"), package_json)?;
    let _flattened_package_json: Vec<(String, String)> = flat_btree
        .iter()
        .map(|(k, v)| (value_to_string(k), value_to_string(v)))
        .collect();

    view! {
        mut command = Command::new(shell) {
            args: [quiet_flag, &content],
            env: args!(
                "PATH",
                format!("{}:{system_path}", bin_dir.to_string_lossy())
            ),
            //envs: args!(flattened_package_json)
        }
    };

    let status = command.status().await?;

    match status.code() {
        Some(code) if !status.success() => {
            println!();
            user_error(t!("script-not-ok", code = &code.to_string()), code);
        }
        None => {
            println!();
            user_error(t!("script-terminated-by-signal"), SCRIPT_SIGNAL_EXIT_CODE);
        }
        _ => {}
    }

    Ok(())
}

fn value_to_string(value: &Value) -> String {
    if let Value::String(s) = value {
        s.clone()
    } else {
        serde_json::to_string(value).unwrap()
    }
}
