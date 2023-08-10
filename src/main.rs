use clap::{command, Arg};
use teshr;

fn main() {
    let matches = command!().arg(Arg::new("to_format")).get_matches();
    let to_format = matches.get_one::<String>("to_format");

    let target_enum = match to_format {
        None => panic!("handle this"),
        Some(t) => select_target(t.clone()),
    };
    println!("got: {:?}", target_enum);
    target_enum.translate(String::from("te😀st"));
    target_enum.translate(String::from("snake_case_test"));
}

fn select_target(query: String) -> teshr::TranslationTarget {
    match query.as_str() {
        "title" => teshr::TITLECASE,
        "upper" => teshr::UPPERCASE,
        // "snake" => TranslationTarget::SnakeCase,
        _ => panic!("unknown operation: {}", query),
    }
}
