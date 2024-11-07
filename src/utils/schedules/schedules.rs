#![allow(unused_imports)]

#[cfg(windows)]
use std::os::windows::process::CommandExt;
use std::process::Command;

use chrono::{DateTime, Local};
use clap::{arg, command, Parser};
use rand::Rng;

use crate::utils::configs::configs::Config;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(
        short = 'R',
        long = "runascron",
        help = "run program without scheduler"
    )]
    runascron: bool,
}

fn calculate_time(config: &Config) -> (u32, u32, u32) {
    let local_now: DateTime<Local> = Local::now();
    let local_offset = local_now.offset().utc_minus_local();
    let target_offset = config.server_utc * 3600;
    let mut delta_seconds = (local_offset - target_offset) as i64;

    delta_seconds += config.delay_minute * 60;

    if config.randomize {
        let random_seconds: u64 = rand::thread_rng().gen_range(0..=config.random_range);
        delta_seconds += random_seconds as i64;
    }

    let total_seconds = delta_seconds.rem_euclid(24 * 3600);
    let target_hour = (total_seconds / 3600) as u32;
    let target_minute = ((total_seconds % 3600) / 60) as u32;
    let target_second = (total_seconds % 60) as u32;

    (target_hour, target_minute, target_second)
}

pub fn check_scheduler() -> bool {
    let args = Args::parse();
    !args.runascron
}

pub fn config_scheduler(
    config: &Config,
    exec_path: &str,
    app_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let (target_hour, target_minute, target_second) = calculate_time(config);

    let mut powershell_script = format!(
        "$Time = New-ScheduledTaskTrigger -Daily -At {:02}:{:02}:{:02};\n",
        target_hour, target_minute, target_second
    );

    let argument = if config.randomize { "" } else { "-Argument -R" };

    powershell_script.push_str(&format!(
        "$Action = New-ScheduledTaskAction -Execute '{}' {} -WorkingDirectory '{}';\n",
        exec_path, argument, app_path
    ));

    powershell_script.push_str(
        "$Setting = New-ScheduledTaskSettingsSet -StartWhenAvailable -AllowStartIfOnBatteries -DontStopIfGoingOnBatteries -WakeToRun -RunOnlyIfNetworkAvailable -MultipleInstances Parallel -Priority 3 -RestartCount 30 -RestartInterval (New-TimeSpan -Minutes 1);\n"
    );

    powershell_script.push_str(
        "$Principal = New-ScheduledTaskPrincipal -UserId 'SYSTEM' -LogonType ServiceAccount -RunLevel Highest;\n"
    );

    powershell_script.push_str(&format!(
        "Register-ScheduledTask -Force -TaskName '{}' -Trigger $Time -Action $Action -Settings $Setting -Principal $Principal -Description 'Genshin Hoyolab Daily Check-In Bot';\n",
        config.scheduler_name
    ));

    #[cfg(windows)]
    {
        let status = Command::new("powershell")
            .args(&[
                "-NoProfile",
                "-ExecutionPolicy",
                "Bypass",
                "-Command",
                &powershell_script,
            ])
            .creation_flags(0x08000000)
            .status()?;

        if !status.success() {
            eprintln!("PERMISSION ERROR: please run as administrator to enable task scheduling");
            std::process::exit(1);
        } else {
            println!("Program scheduled daily!");
        }
    }

    Ok(())
}
