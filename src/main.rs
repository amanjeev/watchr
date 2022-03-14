use clap::{App, Arg};
use std::process::Command;
use std::thread::sleep;
use std::time;

fn main() {
    // TODO: define target os only to linux
    let watchr_app = App::new("watchr")
        .version("0.1.0")
        .about(
            "Watch a command every few seconds. Like watch command but with a few better options.",
        )
        .author("Amanjeev Sethi")
        .arg(
            Arg::new("clear")
                .required(false)
                .short('c')
                .help("Clear the terminal on each iteration")
                .takes_value(false),
        )
        .arg(
            Arg::new("duration")
                .required(false)
                .short('d')
                .help("Duration in seconds to repeat the command execution")
                .takes_value(true)
                .default_value("1"),
        )
        .setting(clap::AppSettings::TrailingVarArg)
        .arg(
            Arg::new("command")
                .multiple_occurrences(true)
                .required(true)
                .takes_value(true)
                .help("Command to watch over"),
        )
        .get_matches();

    let command: Vec<&str> = watchr_app
        .values_of("command")
        .expect("Sorry! I failed to gather the values of your command")
        .collect();
    let duration = watchr_app
        .value_of("duration")
        .expect("Sorry! I failed to get the value of duration");
    let mut count: usize = 0;

    let mut cmd = Command::new("sh");
    cmd.arg("-c"); // arg for `sh` command, for new shell
    cmd.arg(command.join(" ")); // the `command` is an arg to `sh` above

    loop {
        count += 1;

        let output = cmd
            .output()
            .expect("Sorry! Something went wrong while trying to run your command");
        let o = String::from_utf8_lossy(output.stdout.as_slice()).to_string();

        if watchr_app.is_present("clear") {
            print!("\x1B[2J\x1B[1;1H");
        }

        print!(
            "Iteration number {}: Every {} seconds for command {} {} ",
            count,
            duration,
            command.join(" "),
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
