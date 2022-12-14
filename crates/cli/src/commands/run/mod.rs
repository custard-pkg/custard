use std::env;

use colored::Colorize;
use eyre::Result;
use relm4_macros::view;
use rust_i18n::t;
use tokio::process::Command;

mod list;
mod util;
//mod json_vars;

use custard_util::user_error;
use package_json::PackageJson;
use util::{get_node_modules_bin_dir, scripts_field_not_found};

use crate::consts::{LIFECYCLE_SCRIPTS, SCRIPT_SIGNAL_EXIT_CODE};

pub async fn invoke(
    script_name: Option<String>,
    args: Option<Vec<String>>,
    script_shell: String,
    if_present: bool,
    ignore_scripts: bool,
    run_with_lifecycle_cmd: bool,
) -> Result<()> {
    let args = args.unwrap_or_default();
    let package_json = PackageJson::from_package_json_file().await?;

    if let Some(script_name) = script_name {
        if !run_with_lifecycle_cmd && LIFECYCLE_SCRIPTS.contains(&script_name.as_str()) {
            eprintln!(
                "{} {}",
                "tip:".yellow().bold(),
                &t!("run.run-with-lifecycle-script-command", name = &script_name)
                    .black()
                    .bold()
            );
        }

        if let Some(ref scripts) = package_json.scripts {
            if let Some(script_content) = scripts.get(&script_name) {
                let pre_script_name = format!("pre{script_name}");
                let post_script_name = format!("post{script_name}");

                if !ignore_scripts {
                    // Run prescript...
                    if let Some(script_content) = scripts.get(&pre_script_name) {
                        run_script(&pre_script_name, script_content, &[], &script_shell).await?;
                    }
                }

                // ...then the script itself...
                run_script(&script_name, script_content, &args, &script_shell).await?;

                // ...and finally the postscript
                if !ignore_scripts {
                    if let Some(script_content) = scripts.get(&post_script_name) {
                        run_script(&post_script_name, script_content, &[], &script_shell).await?;
                    }
                }
            } else if !if_present {
                user_error(
                    t!("run.script-not-found", name = &script_name),
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
    script_shell: &String,
) -> Result<()> {
    let bin_dir = get_node_modules_bin_dir()?;
    let content = format!("{content} {}", args.join(" "));

    // Print the script info
    println!("{} script `{name}`", "Running".green().bold());
    println!("\n{}", format!("> {content}").bold());

    // Get the system specific shell, and the flag to make it quiet
    let quiet_flag = if cfg!(windows) { "/C" } else { "-c" };

    // Get the "real" PATH environment variable.
    let system_path = env::var("PATH")?;

    view! {
        mut command = Command::new(script_shell) {
            args: [quiet_flag, &content],
            env: args!(
                "PATH",
                format!("{}:{system_path}", bin_dir.to_string_lossy())
            ),
            //envs: args!(flattened_package_json)
        }
    };

    match command.status().await {
        // The shell ran successfully...
        Ok(status) => {
            // ... but we don't know
            match status.code() {
                Some(code) if !status.success() => {
                    println!();
                    user_error(t!("run.script-not-ok", code = &code.to_string()), code);
                }
                None => {
                    println!();
                    user_error(
                        t!("run.script-terminated-by-signal"),
                        SCRIPT_SIGNAL_EXIT_CODE,
                    );
                }
                _ => {}
            }
        }

        // The shell failed to execute. Likely this means that the
        // --script-shell argument was invalid.
        Err(_) => user_error(t!("run.failed-to-start-script-shell"), exitcode::OSFILE),
    }

    Ok(())
}
