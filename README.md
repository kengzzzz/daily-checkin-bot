# Genshin Impact Daily Check-In Bot

This program is a bot that automatically checks in daily on the Hoyolab website for the game Genshin Impact. It's easy to use and allows you to schedule check-ins according to your preferences.

## Features

- **Automatic Daily Check-In**: The program will check in for you every day without manual intervention.
- **Customizable Check-In Time**: You can set the check-in time according to the server's time zone.
- **Randomized Check-In Time**: Supports randomizing the check-in time within a specified range for added security.
- **Automatic Scheduler Setup**: The program automatically creates a Scheduled Task on Windows.

## Installation

### Requirements

- **Rust**: Rust must be installed on your machine. [Download here](https://www.rust-lang.org/tools/install)
- **Windows Operating System**: This program supports Windows only, as it utilizes the Windows Task Scheduler.

### Installation Steps

1. **Clone or download this project**:

   ```bash
   git clone https://github.com/kengzzzz/gi-daily-checkin-bot.git
   cd gi-daily-checkin-bot
   ```

2. **Create a `cookie.txt` file in the project folder**

Insert your Hoyolab website cookie into this file (do not share or publish this file as it contains personal information).

3. **Customize the `config.json` file according to your preferences:**

  ```json
  {
      "SERVER_UTC": 8,
      "DELAY_MINUTE": 0,
      "RANDOMIZE": false,
      "RANDOM_RANGE": 3600,
      "SCHEDULER_NAME": "HoyolabCheckInBot"
  }
  ```

  - `SERVER_UTC`: The time zone offset of the server (e.g., Asia server uses 8)
  - `DELAY_MINUTE`: Delay the check-in time (in minutes)
  - `RANDOMIZE`: Set to true if you want to randomize the check-in time
  - `RANDOM_RANGE`: The range for randomization (in seconds)
  - `SCHEDULER_NAME`: The name of the Scheduled Task to be created

4. **Compile the program:**

  ```bash
    cargo build --release
  ```

5. **Run the program with administrator privileges:**

To set up the Scheduled Task, the program needs to run with administrator rights.

## Usage

- The program will automatically set up a Scheduled Task in Windows for you.
- You can check the check-in status by viewing the messages displayed by the program or by checking the Hoyolab website.

## Additional Information

- `cookie.txt` file: Stores your Hoyolab website cookie, which is personal information. Please do not share or publish this file.
- Scheduled Task: The program will create a Scheduled Task with the name specified in `config.json` to automatically check in every day.
- **Time Settings**: The program calculates the check-in time based on your machine's local time zone and the server time zone specified.

## Troubleshooting

- If the program fails to create the Scheduled Task, it may be due to insufficient permissions. Try running the program as an administrator.
- If the check-in fails, verify that your `cookie.txt` contains a valid and unexpired cookie.

## Warnings

- Using this program may violate the terms of service of Hoyolab. Use it at your own risk.
- The developer is not responsible for any damages or account bans that may result from using this program.

## Support

If you have any questions or encounter any issues, please open an issue on the project's GitHub page.
