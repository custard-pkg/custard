use std::env;

use eyre::Result;
use owo_colors::OwoColorize;
use rust_i18n::t;
use tokio::process::Command;

mod list;
mod util;

use util::{get_node_modules_bin_dir, scripts_field_not_found};

use crate::consts::{LIFECYCLE_SCRIPTS, SCRIPT_SIGNAL_EXIT_CODE};
use crate::package_json::PackageJson;
use crate::util::user_error;

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
            );
        }

        if let Some(scripts) = package_json.scripts {
            if let Some(script_content) = scripts.get(&script_name) {
                let pre_script_name = format!("pre{script_name}");
                let post_script_name = format!("post{script_name}");

                // Run prescript...
                if let Some(script_content) = scripts.get(&pre_script_name) {
                    run_script(&pre_script_name, script_content, &[]).await?;
                }

                // ...then the script itself...
                run_script(&script_name, script_content, &args).await?;

                // ...and finally the postscript
                if let Some(script_content) = scripts.get(&post_script_name) {
                    run_script(&post_script_name, script_content, &[]).await?;
                }
            } else {
                user_error(
                    t!("script-not-found", name = &script_name),
                    exitcode::CONFIG,
                )
            }
        }
    } else {
        list::scripts().await?;
    }

    Ok(())
}

async fn run_script(name: &str, content: &str, args: &[String]) -> Result<()> {
    let bin_dir = get_node_modules_bin_dir().await?;
    let content = format!("{content} {}", args.join(" "));

    println!("{} script `{name}`", "Running".green().bold());
    println!("\n{}", format!("> {content}").bold());

    let mut command = &mut if cfg!(target_os = "windows") {
        Command::new("cmd")
    } else {
        Command::new("sh")
    };
    command = command
        .arg(if cfg!(target_os = "windows") {
            "/C"
        } else {
            "-c"
        })
        .arg(content);
    if let Some(bin_dir) = bin_dir {
        if let Ok(system_path) = env::var("PATH") {
            command = command.env(
                "PATH",
                format!("{}:{system_path}", bin_dir.to_string_lossy()),
            )
        }
    }

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
