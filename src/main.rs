use notify_rust::{Hint, Notification, Timeout};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::Command;

fn main() {
    let status = Command::new("pkill").arg("swayidle").status();

    let sway_idle_command = get_command_from_config();

    match status {
        Ok(status) => {
            if status.success() {
                println!("Successfully killed swayidle.");
                notify(false);
            } else {
                println!("Starting swayidle...");

                let _ = Command::new("setsid")
                    .arg("sh")
                    .arg("-c")
                    .arg(sway_idle_command)
                    .spawn()
                    .expect("failed to spawn swayidle");

                println!("Successfully started swayidle.");
                notify(true);
            }
        }
        Err(_) => {
            println!("pkill could not be executed.");
        }
    }
}

fn get_command_from_config() -> String {
    let home_dir = env::var("HOME").expect("Couldn't get HOME environment variable.");
    let config_path = format!("{}/.config/sway/config", home_dir);

    let config_file = File::open(&config_path)
        .expect(&format!("Could not open file: {}", config_path.to_string()));

    let reader = BufReader::new(config_file);

    let mut in_section = false;
    let mut collected_lines = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Could not read line.");
        let trimmed = line.trim();

        if trimmed == "#! SWAYIDLE START !#" {
            in_section = true;
            continue;
        }
        if trimmed == "#! SWAYIDLE END !#" {
            break;
        }
        if in_section {
            let line = if trimmed.starts_with("exec ") {
                trimmed.replacen("exec ", "", 1)
            } else {
                trimmed.to_string()
            };
            collected_lines.push(line);
        }
    }
    collected_lines.join("").trim().to_string()
}

fn notify(status: bool) {
    let (summary, body) = if status {
        (
            "ENABLED".to_string(),
            "Swayidle is now enabled.".to_string(),
        )
    } else {
        (
            "DISABLED".to_string(),
            "Swayidle is now disabled.".to_string(),
        )
    };

    Notification::new()
        .summary(&summary)
        .body(&body)
        .icon("dialog-information")
        .appname("sway-idle-switch")
        .hint(Hint::Category("Device".to_owned()))
        .timeout(Timeout::Milliseconds(10000))
        .show()
        .unwrap();
}
