use std::{borrow::Cow, collections::VecDeque};

use crate::errors::ConverterError;

#[derive(Debug, Clone)]
pub struct BinaryConverter<'a> {
    number: usize,
    base: Base,
    output: VecDeque<Cow<'a, str>>,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Base {
    Hex,
    Binary,
    Octal,
}

impl<'a> BinaryConverter<'a> {
    // loads number into the Converter while clearing it's output
    pub fn load_number(&mut self, number: usize) {
        self.number = number;
        self.output.clear();
    }
    /// takes in a ref to a string and tries to build a vec while refecning to the Map of the correspoinding base, *~~explodes~~* if corresponding thing isn't found
    pub fn load_code(&mut self, code: &str) -> Result<(), ConverterError> {
        self.output = self.parse_code(code)?;
        Ok(())
    }
    /// changes the base and clears the output
    pub fn load_base(&mut self, base: Base) {
        self.base = base;
        self.output.clear();
    }
    /// gives you the number
    pub fn number(&self) -> usize {
        self.number
    }
    /// gives you a refence to it's base
    pub fn base(&self) -> &Base {
        &self.base
    }
    /// creates a Self with the specified base
    pub fn with_base(base: Base) -> Self {
        Self {
            number: 0,
            base,
            output: VecDeque::new(),
        }
    }
    pub fn output(&self) -> &VecDeque<Cow<'a, str>> {
        &self.output
    }

    pub const HEX_MAP: [&'static str; 16] = [
        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "A", "B", "C", "D", "E", "F",
    ];
    pub const BIN_MAP: [&'static str; 2] = ["0", "1"];
    pub const OCTO_MAP: [&'static str; 8] = ["0", "1", "2", "3", "4", "5", "6", "7"];
    /// takes in self.number and self.base and calculates that into a vec with the converted numbers in self.output
    pub fn calculate(&mut self) -> Result<(), ConverterError> {
        match self.base {
            Base::Binary => {
                self.output = Self::convert(self.number, 2, Base::Binary);
                Ok(())
            }
            Base::Hex => {
                self.output = Self::convert(self.number, 16, Base::Hex);
                Ok(())
            }
            Base::Octal => {
                self.output = Self::convert(self.number, 8, Base::Octal);
                Ok(())
            }
        }
    }

    fn convert(starting_number: usize, divisor: usize, base: Base) -> VecDeque<Cow<'a, str>> {
        // limit
        let mut count: u32 = 0;
        let (mut current_number, _remainder): (usize, usize) = (starting_number, 0);
        let mut buffer: VecDeque<Cow<'_, str>> = VecDeque::with_capacity(8);
        let map: &[&str] = match base {
            Base::Binary => &Self::BIN_MAP,
            Base::Octal => &Self::OCTO_MAP,
            Base::Hex => &Self::HEX_MAP,
        };
        '_main_loop: loop {
            let calculated = current_number / divisor;
            let remainder = current_number % divisor;
            buffer.push_front(Cow::Borrowed({
                map.get(remainder).expect("map cover out of range")
            }));

            current_number = calculated;

            if calculated == 0 {
                break;
            }

            // Safety
            count += 1;
            if count >= 200 {
                break;
            }
        }
        buffer
    }

    pub fn check_if_base_and_output_is_coherent(&self) -> bool {
        let map: &[&str] = match self.base {
            Base::Binary => &Self::BIN_MAP,
            Base::Octal => &Self::OCTO_MAP,
            Base::Hex => &Self::HEX_MAP,
        };

        self.output
            .iter()
            .all(|digit| map.contains(&digit.as_ref()))
    }
    fn parse_code(&mut self, content: &str) -> Result<VecDeque<Cow<'a, str>>, ConverterError> {
        let map: &[&str] = match self.base {
            Base::Binary => &Self::BIN_MAP,
            Base::Octal => &Self::OCTO_MAP,
            Base::Hex => &Self::HEX_MAP,
        };
        let mut out: VecDeque<Cow<'a, str>> = VecDeque::with_capacity(content.len());

        for c in content.chars().map(|c| c.to_string()) {
            if !map.contains(&c.as_str()) {
                return Err(ConverterError::InvalidCharacter {
                    base: self.base().clone(),
                    letter: c.chars().nth(0).expect("well... there should be one"),
                });
            }

            out.push_back(Cow::Owned(c));
        }

        Ok(out)
    }
}
// ----- Helper Functions -----

impl Base {
    pub fn powers(&self) -> usize {
        match self {
            Self::Binary => 2,
            Self::Hex => 16,
            Self::Octal => 8,
        }
    }
}

impl<'a> Default for BinaryConverter<'a> {
    fn default() -> Self {
        Self {
            base: Base::Binary,
            output: VecDeque::new(),
            number: 0,
        }
    }
}

impl std::fmt::Display for Base {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let content = match self {
            Self::Binary => "Binary",
            Self::Hex => "Hex",
            Self::Octal => "Octal",
        };

        write!(f, "{}", content)
    }
}

use owo_colors::OwoColorize;

impl<'a> std::fmt::Display for BinaryConverter<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let thing = self
            .output
            .iter()
            .map(|c| c.as_ref())
            .collect::<Vec<&str>>()
            .join("");
        writeln!(
            f,
            "Base: [{}] with input number [{}]\nOutput: {} in [{}]",
            self.base.bright_green().bold(),
            self.number.cyan().bold(),
            thing.bold().bright_yellow().underline(),
            self.base.bright_blue().bold()
        )?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_binary() {
        let mut converter = BinaryConverter::with_base(Base::Binary);
        converter.number = 75;

        converter.calculate();

        println!("{}", converter);
    }
    #[test]
    fn test_hex() {
        let mut converter = BinaryConverter::with_base(Base::Hex);
        converter.number = 73223;

        converter.calculate();

        println!("{}", converter)
    }
    #[test]
    fn test_octo() {
        let mut converter = BinaryConverter::with_base(Base::Octal);
        converter.number = 73223;

        converter.calculate();

        println!("{}", converter)
    }
}
