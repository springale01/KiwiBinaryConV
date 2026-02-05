use std::{borrow::Cow, collections::VecDeque, rc::Rc};

use owo_colors::OwoColorize;

use crate::{
    converter::{Base, BinaryConverter},
    errors::ConverterError,
};

/*
GOAL:  target's base                                 self's code                                 self's base
Target: [BASE] {<------ given by the CLI} with input code [CODE] {<--- given by the Cli} with base [BASE] {<---- given by the cli}
Output: [OUTPUT] {<---- From the Vec and should not be a number}
*/
impl<'a> BinaryConverter<'a> {
    /// gives you back a string that could be used to be printed
    pub fn to_target(&mut self, target: Base) -> Result<String, ConverterError> {
        // assuming self's code and base are already filled;
        let loaded_code = self.output().clone();
        self.revert_v2()?;
        self.load_base(target.clone());
        self.calculate()?;

        {
            let thing = Self::cow_deque_to_string(self.output());

            let thing2: String = Self::cow_deque_to_string(&loaded_code);
            Ok(format!(
                "Target: [{}] with input code [{}]\nOutput: [{}]",
                target.bold().bright_cyan(),
                thing2.bold().bright_green(),
                thing.bold().bright_yellow()
            ))
        }
    }
    fn cow_deque_to_string(deque: &VecDeque<Cow<'_, str>>) -> String {
        let mut s = String::new();
        for part in deque {
            s.push_str(part.as_ref());
        }
        s
    }
}
