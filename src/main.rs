use clap::{App, Arg};
use std::process::Command;

fn main() {
    // TODO: add an option to not preserve history (by default we preserve history in this tool)
    let matches = App::new("watchr")
        .setting(clap::AppSettings::TrailingVarArg)
        .arg(
            Arg::new("command")
                .multiple_occurrences(true)
                .required(true)
                .takes_value(true)
                .help("Command to watch over"),
        )
        .get_matches();

    let command: Vec<&str> = matches.values_of("command").unwrap().collect();
    // TODO: number of times to repeat, if given
    let mut count: usize = 0;

    let mut cmd = Command::new("sh");
    cmd.arg("-c"); // arg for `sh` command, for new shell
    cmd.arg(command.join(" ")); // the `command` is an arg to `sh` above

    // TODO: time interval
    loop {
        count += 1;
        println!("{:?}", cmd);
        // TODO: give better error message
        let output = cmd.output().expect("aaaaaaa");
        // TODO: display newlines and not escaped `\n`
        println!("{:?}", String::from_utf8_lossy(output.stdout.as_slice()));
    }
}
