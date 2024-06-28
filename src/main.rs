use clap::{command, value_parser, Arg, ArgAction, Command, ValueHint};
use clap_complete::{generate, Generator, Shell};
use std::io;
use std::process::ExitCode;

fn main() -> ExitCode {
    let matches = build_cli().get_matches();

    if let Some(generator) = matches.get_one::<Shell>("generator").copied() {
        let mut cmd = build_cli();
        eprintln!("Generating completion file for {generator}...");
        print_completions(generator, &mut cmd);
        return ExitCode::SUCCESS;
    }

    let to_format = matches.get_one::<String>("to_format");

    let target_enum = match to_format {
        None => panic!("handle this"),
        Some(t) => select_target(t.clone()),
    };
    println!("got: {:?}", target_enum);
    target_enum.translate(String::from("te😀st"));
    target_enum.translate(String::from("snake_case_test"));

    ExitCode::SUCCESS
}

fn select_target(query: String) -> teshr::TranslationTarget {
    match query.as_str() {
        "title" => teshr::TITLECASE,
        "upper" => teshr::UPPERCASE,
        // "snake" => TranslationTarget::SnakeCase,
        _ => panic!("unknown operation: {}", query),
    }
}

fn build_cli() -> Command {
    Command::new("teshr")
        .subcommand(
            Command::new("uppercase")
                .about("To uppercase")
                .long_about("Example: 'THE QUICK BROWN FOX'"),
        )
        .about("The target format")
        .arg(
            Arg::new("generator")
                .long("generate")
                .action(ArgAction::Set)
                .value_parser(value_parser!(Shell)),
        )
}

fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}
