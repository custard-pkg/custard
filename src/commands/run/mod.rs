use eyre::Result;
use owo_colors::OwoColorize;
use rust_i18n::t;
use tokio::process::Command;

mod list;
mod util;

use util::scripts_field_not_found;

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

    match script_name {
        Some(script_name) => {
            if !run_with_lifecycle_cmd && LIFECYCLE_SCRIPTS.contains(&script_name.as_str()) {
                eprintln!(
                    "{} {}",
                    "tip:".yellow().bold(),
                    &t!("run-with-lifecycle-script-command", name = &script_name)
                );
            }

            match package_json.scripts {
                Some(scripts) => match scripts.get(&script_name) {
                    Some(script_content) => {
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
                    }
                    _ => user_error(
                        t!("script-not-found", name = &script_name),
                        exitcode::CONFIG,
                    ),
                },

                // The `scripts` field was not found
                _ => scripts_field_not_found(),
            }
        }
        _ => list::scripts().await?,
    }

    Ok(())
}

async fn run_script(name: &str, content: &str, args: &[String]) -> Result<()> {
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
