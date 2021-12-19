use clap::{App, Arg};

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

    // TODO: watch over the command saved in `command` above

    println!("{:?}", command);
}
