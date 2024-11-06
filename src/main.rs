use std::time::Duration;

use gi_daily_checkin_bot::utils::{
    configs::configs::{read_config, read_cookie},
    requests::requests::{claim, is_claimed},
    schedules::schedules::{check_scheduler, config_scheduler},
};
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if check_scheduler() {
        let config = read_config()?;
        let exec_path = std::env::current_exe()?.to_str().unwrap().to_string();
        let app_path = std::env::current_dir()?.to_str().unwrap().to_string();

        config_scheduler(&config, &exec_path, &app_path)?;
    }

    loop {
        let cookie_value = read_cookie()?;
        if !cookie_value.is_empty() {
            let client = reqwest::Client::new();

            let claimed = is_claimed(&client, &cookie_value).await?;
            if claimed {
                println!("Reward has already been claimed.");
                break;
            } else {
                let success = claim(&client, &cookie_value).await?;
                if success {
                    println!("Reward claimed successfully!");
                    break;
                } else {
                    println!("Failed to claim the reward.");
                }
            }
        } else {
            eprintln!("Cookie value is empty. Please check your cookie.txt file.");
        }
        sleep(Duration::from_secs(60)).await;
    }

    Ok(())
}
