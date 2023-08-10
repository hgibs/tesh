use clap::{command, Arg};
use teshr::{select_target, translate, TranslationTarget};

fn main() {
    let matches = command!().arg(Arg::new("to_format")).get_matches();
    let to_format = matches.get_one::<String>("to_format");

    let target_enum = match to_format {
        None => TranslationTarget::UpperCase,
        Some(t) => select_target(t.clone()),
    };
    println!("got: {:?}", target_enum);
    translate(String::from("te😀st"), target_enum);
}
