#![allow(unused)]

use crate::{
    clap_options::ClapOptions,
    converter::{Base, BinaryConverter},
    errors::ConverterError,
    reverter::Deserialize,
};
use clap::Parser;
use owo_colors::OwoColorize;
use std::rc::Rc;
mod clap_options;
mod converter;
mod errors;
mod reverter;
mod targetoutput;
fn main() -> Result<(), ConverterError> {
    if let Err(err) = bougie_main() {
        pretty_print_error(&err);
        // We tell the program that it may have exploded somewhere
        std::process::exit(1)
    }
    Ok(())
}
/// ## Fake main that loads all the env variables and does matching stuff
/// *~~So that I can ifl it in main~~*
fn bougie_main() -> Result<(), ConverterError> {
    let args = ClapOptions::parse();
    let base = Base::deserialize(&args.base)?;

    let mut converter = BinaryConverter::with_base(base.clone());
    if let (Some(target), Some(code)) = (args.target.as_deref(), args.code.as_deref()) {
        let code = code.to_ascii_uppercase();
        let target_base = Base::deserialize(&target)?;
        converter.load_code(&code)?;
        let out = converter.to_target(target_base.clone())?;
        if args.no_prefix {
            println!("{}", out);
        } else {
            println!("{}", insert_prefix(out.to_string(), &target_base)?)
        }

        return Ok(());
    }
    // matches number and shared code so if one of them is there we do the appropite function
    // both true is solved by mutally exculsive or smt in clap options
    match (args.number, args.code.as_deref()) {
        (Some(number), None) => {
            converter.load_number(number);
            converter.calculate()?;

            if args.no_prefix {
                println!("{}", converter);
            } else {
                println!("{}", insert_prefix(converter.to_string(), &base)?)
            }
        }
        (None, Some(code)) => {
            let code = code.to_ascii_uppercase();
            converter.load_code(&code)?;
            converter.revert_v2()?;
            println!("{}", converter.reverse_print(&code));

            // Don't need to insert if it's like going back to base 10!
            // if args.no_prefix {
            //     println!("{}", converter.reverse_print(&code));
            // } else {
            //     println!(
            //         "{}",
            //         insert_prefix(converter.reverse_print(&code).to_string(), &base)?
            //     )
            // }
        }
        _ => {
            return Err(ConverterError::CustomError(
                "Please pass either --number or --code".to_string(),
            ));
        }
    }

    Ok(())
}
/// Pretty Prints the Custom Errors
fn pretty_print_error(err: &ConverterError) {
    match err {
        ConverterError::InvalidCharacter { letter, base } => {
            eprintln!(
                "{} {} [{}] {} [{}]",
                "Error:".red().bold(),
                "Invalid character".yellow(),
                letter.bold().underline(),
                "conflicting with the base:",
                base.bold().underline()
            );
        }
        ConverterError::NumberOutOfMap => {
            eprintln!(
                "{} {}",
                "Error:".red().bold(),
                "Number not valid for this base".yellow()
            );
        }
        ConverterError::NumberAndBaseNotCoherent => {
            eprintln!(
                "{} {}",
                "Error:".red().bold(),
                "Number and base do not match".yellow()
            );
        }
        ConverterError::CustomError(msg) => {
            eprintln!("{} {}", "Error: ".red().bold(), msg);
        }
        ConverterError::FailedToConvert => {
            eprintln!(
                "{} {}",
                "Error:".red().bold(),
                "Failed to convert input".yellow()
            );
        }
    }
}
// ----- Helper Functions -----
/// Inserts a prefix determined by the base given <br>
/// *~~Also explodes if the output is more than 2 lines~~*
fn insert_prefix(mut target: String, base: &Base) -> Result<String, ConverterError> {
    // Finds where is the blank space
    let lines = target
        .split_once("\n")
        .and_then(|(kitten, bomb)| {
            let fuse = bomb.lines().count();
            if fuse > 1 {
                // sorry kitten if this explodes, we loved you either ways
                panic!("Internal Error, there is two lines when there should only be one in line 2 of output!")
            }

            Some((kitten, bomb))
        })
        .ok_or(ConverterError::CustomError(String::from(
            "Internal Error, there is two lines when there should be one",
        )))?;

    let where_at = lines.0.len()
        + lines.1.find(" ").ok_or(ConverterError::CustomError(
            "Internal Error, Internal format '[' not foudn".into(),
        ))?
        + 2;

    let prefix = match base {
        Base::Binary => "0b",
        Base::Hex => "0x",
        Base::Octal => "0o",
    };

    target.insert_str(
        where_at,
        &prefix.bold().bright_yellow().underline().to_string(),
    );

    Ok(target)
}

#[test]
fn test_insert() {
    let test_str = "Target: [Octal] with input code [BEEFED01]\nOutput: [27673766401]";
    let yeeted = insert_prefix(test_str.into(), &Base::Octal).unwrap();

    println!("{}", yeeted)
}
