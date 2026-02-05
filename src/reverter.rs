use std::{borrow::Cow, collections::VecDeque};

use owo_colors::OwoColorize;

use crate::{
    converter::{Base, BinaryConverter},
    errors::ConverterError,
};

impl<'a> BinaryConverter<'a> {
    // newer one
    /// This uses the output(a vec) and the base and recreate the number from that and muts slef
    pub fn revert_v2(&mut self) -> Result<(), ConverterError> {
        let map: &[&str] = match self.base() {
            Base::Binary => &Self::BIN_MAP,
            Base::Octal => &Self::OCTO_MAP,
            Base::Hex => &Self::HEX_MAP,
        };

        if !self.check_if_base_and_output_is_coherent() {
            return Err(ConverterError::NumberAndBaseNotCoherent);
        }

        let base = map.len();
        let mut value: usize = 0;
        for digit in self.output() {
            let idx = map.iter().position(|&x| x == digit.as_ref()).ok_or(
                ConverterError::CustomError(format!(
                    "digit: [{}] is not found in the static map related to the base [{}]",
                    digit,
                    self.base()
                )),
            )?;

            value = value
                .checked_mul(base)
                .and_then(|thing| thing.checked_add(idx))
                .ok_or(ConverterError::CustomError(format!(
                    "Value: [{}] might have overflowed during function revert_v2",
                    value
                )))?;
        }
        self.load_number(value);
        Ok(())
    }
    /// ~~Don't use this it's very cursed~~
    fn revert(&mut self) -> Result<(), ConverterError> {
        // GOAL: turn hex/bin/others back to normal digets
        if !self.check_if_base_and_output_is_coherent() {
            return Err(ConverterError::NumberAndBaseNotCoherent);
        }

        match self.base() {
            Base::Binary => {
                let power = if let Some(checked_sub) = self.output().len().checked_sub(1) {
                    checked_sub
                } else {
                    return Err(ConverterError::CustomError(
                        "Why did you use this ;-;".into(),
                    ));
                };
                let two_power_box = vec![power..=0]
                    .into_iter()
                    .flat_map(|x| x)
                    .collect::<Vec<usize>>();

                let mut buf: VecDeque<usize> = VecDeque::with_capacity(8);
                for (bin, power_size) in self.output().iter().zip(two_power_box) {
                    let parsed = bin
                        .parse::<usize>()
                        .expect("How did it get past the bouncer...");

                    let powered = if let Some(checked_pow) = parsed.checked_pow(power_size as u32) {
                        checked_pow
                    } else {
                        return Err(ConverterError::CustomError(
                            "Why did you use this ;-;".into(),
                        ));
                    };

                    buf.push_front(powered);
                }

                let calculated = buf.iter().fold(0usize, |acc, x| acc + *x);
                self.load_number(calculated);

                Ok(())
            }
            Base::Hex => {
                // problem is the method up above doesn't work down here since it instantly explodes if I try to parse a hex 'A' into a usize
                todo!("It's never going to be done")
            }
            Base::Octal => {
                // it works for this one but I'm too lazy
                todo!()
            }
            _ => Err(ConverterError::CustomError(
                "Why did you use this ;-;".into(),
            )),
        }
    }
    pub fn reverse_print(&self, code: &str) -> Cow<'_, str> {
        let thing = Cow::Owned(format!(
            "Base: [{}] with input code [{}]\nOutput: {}",
            self.base().bold().bright_green(),
            code.bold().bright_cyan(),
            self.number().bold().yellow()
        ));
        thing
    }
}

pub trait Deserialize<T> {
    type Error;
    fn deserialize(content: &str) -> Result<T, Self::Error>;
}

impl Deserialize<Base> for Base {
    type Error = ConverterError;
    fn deserialize(content: &str) -> Result<Base, Self::Error> {
        match content.to_lowercase().trim() {
            "binary" => Ok(Self::Binary),
            "bin" => Ok(Self::Binary),
            "hexadecimal" => Ok(Self::Hex),
            "hex" => Ok(Self::Hex),
            "octal" => Ok(Self::Octal),
            "eight" => Ok(Self::Octal),
            "oct" => Ok(Self::Octal),

            _ => Err(ConverterError::CustomError({
                format!(
                    "Content Passed: [{}] did not match into a vaild base",
                    content
                )
            })),
        }
    }
}
