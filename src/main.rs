use clap::Parser;
use std::{process::Command, thread::sleep, time};

#[derive(Parser, Debug)]
#[command(
    author = "Amanjeev Sethi",
    version,
    about = "Incomplete `GNU cat` in Rust for learning purposes"
)]
pub struct Args {
    #[arg(name = "clear", short = 'c', default_value = "false", required = false)]
    clear: bool,
    #[arg(name = "duration", short = 'd', default_value = "1", required = false)]
    duration: usize,
    #[arg(name = "command", required = true)]
    command: String,
}

fn main() {
    let args = Args::parse();
    let command = args.command;
    let duration = args.duration;
    let clear = args.clear;
    let mut cmd = Command::new("sh");
    cmd.arg("-c"); // arg for `sh` command, for new shell
    cmd.arg(command.clone()); // the `command` is an arg to `sh` above

    let mut count: usize = 0;

    loop {
        count += 1;

        let output = cmd
            .output()
            .expect("Sorry! Something went wrong while trying to run your command");
        let o = String::from_utf8_lossy(output.stdout.as_slice()).to_string();

        if clear {
            print!("\x1B[2J\x1B[1;1H");
        }

        print!(
            "Iteration number {}: Every {} seconds for command {} {} ",
            count,
            duration,
            command,
            o,
        );
        sleep(time::Duration::from_secs(
            duration
                .to_string()
                .parse::<u64>()
                .expect("Sorry! I failed to parse duration into second"),
        ));
    }
}
