use std::process::Command;

use eyre::Result;
use owo_colors::OwoColorize;
use rust_i18n::t;

mod list;
mod util;

use list::list_scripts;
use util::scripts_field_not_found;

use crate::consts::SCRIPT_SIGNAL_EXIT_CODE;
use crate::package_json::PackageJson;
use crate::util::user_error;

pub fn invoke(script_name: Option<String>, args: Option<Vec<String>>) -> Result<()> {
    let args = args.unwrap_or_default();
    let package_json = PackageJson::from_package_json_file()?;

    match script_name {
        Some(script_name) => {
            match package_json.scripts {
                Some(scripts) => match scripts.get(&script_name) {
                    Some(script_content) => {
                        let pre_script_name = format!("pre{script_name}");
                        let post_script_name = format!("post{script_name}");

                        // Run prescript...
                        if let Some(script_content) = scripts.get(&pre_script_name) {
                            run_script(&pre_script_name, script_content, &[])?;
                        }

                        // ...then the script itself...
                        run_script(&script_name, script_content, &args)?;

                        // ...and finally the postscript
                        if let Some(script_content) = scripts.get(&post_script_name) {
                            run_script(&post_script_name, script_content, &[])?;
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
        _ => list_scripts()?,
    }

    Ok(())
}

fn run_script(name: &str, content: &str, args: &[String]) -> Result<()> {
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

    let status = command.status()?;

    match status.code() {
        Some(code) if !status.success() => {
            user_error(t!("script-not-ok", code = &code.to_string()), code);
        }
        None => user_error(t!("script-terminated-by-signal"), SCRIPT_SIGNAL_EXIT_CODE),
        _ => {}
    }

    Ok(())
}
