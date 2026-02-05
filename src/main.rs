#![allow(unused)]

use std::rc::Rc;

use crate::{
    clap_options::ClapOptions,
    converter::{Base, BinaryConverter},
    errors::ConverterError,
    reverter::Deserialize,
};
use clap::Parser;
use owo_colors::OwoColorize;
mod clap_options;
mod converter;
mod errors;
mod reverter;
mod targetoutput;
fn main() -> Result<(), ConverterError> {
    if let Err(err) = bougie_main() {
        pretty_print_error(&err);
        std::process::exit(1)
    }
    Ok(())
}

fn bougie_main() -> Result<(), ConverterError> {
    let args = ClapOptions::parse();
    let base = Base::deserialize(&args.base)?;
    let shared_code = if let Some(code) = args.code {
        Some(Rc::new(code))
    } else {
        None
    };
    let mut converter = BinaryConverter::with_base(base);
    if let (Some(target), Some(code)) = (args.target.as_deref(), shared_code.as_deref()) {
        let target_base = Base::deserialize(&target)?;
        converter.load_code(&code)?;
        let out = converter.to_target(target_base)?;
        println!("{}", out);

        return Ok(());
    }

    match (args.number, shared_code) {
        (Some(number), None) => {
            converter.load_number(number);
            converter.calculate()?;
            println!("{}", converter);
        }
        (None, Some(code)) => {
            converter.load_code(&code)?;
            converter.revert_v2()?;
            println!("{}", converter.reverse_print(&code));
        }
        _ => {
            return Err(ConverterError::CustomError(
                "Please pass either --number or --code".to_string(),
            ));
        }
    }

    Ok(())
}

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
            eprintln!("{} {}", "Error:".red().bold(), msg);
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
