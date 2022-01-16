use std::collections::HashMap;

#[derive(Debug)]
pub enum ArgError {
    MissingVariable,
    MissingValue,
    MissingFlagOrVariable,
    MissingFlag,
}

fn process_argument_error(error: Result<(), ArgError>) {
    match error {
        Ok(x) => x,
        Err(ArgError::MissingValue) => {
            println!("Missing value after '=' for defined variable");
            std::process::exit(0)
        }
        Err(ArgError::MissingVariable) => {
            println!("Missing varibale name after '--' and before '=' for defined variable");
            std::process::exit(0)
        }
        Err(ArgError::MissingFlagOrVariable) => {
            println!("Missing flag or variable name after '--'");
            std::process::exit(0)
        }
        Err(ArgError::MissingFlag) => {
            println!(
                "Short flag indicator '-' provided but no argument supplied, please supply flag"
            );
            std::process::exit(0);
        }
    };
}

pub struct ArgParser {
    args: Vec<String>,
    short_flags: Vec<String>,
    long_flags: Vec<String>,
    variables: HashMap<String, String>,
    positional: Vec<String>,
}

impl ArgParser {
    pub fn new() -> ArgParser {
        ArgParser {
            args: Vec::new(),
            short_flags: Vec::new(),
            long_flags: Vec::new(),
            variables: HashMap::new(),
            positional: Vec::new(),
        }
    }

    pub fn collect_args(mut self) -> Self {
        self.args = std::env::args().collect();
        self
    }

    pub fn debug(&self) {
        for arg in &self.short_flags {
            println! {"Short Flag: {:?}",arg};
        }

        for arg in &self.long_flags {
            println! {"long Flag: {:?}",arg};
        }

        for arg in &self.variables {
            println! {"Variable: {:?}",arg};
        }

        for arg in &self.positional {
            println! {"Positional: {:?}",arg};
        }
    }

    /// # Example
    ///
    /// ```
    /// use argparse::ArgParser;
    /// let mut arg_parser = ArgParser::new();
    /// let arg = String::from("-a");
    /// let res = arg_parser.process_short_flag(arg);
    /// assert_eq!(res.is_ok(),true )
    /// ```
    ///
    /// ```
    /// use argparse::ArgParser;
    /// let mut arg_parser = ArgParser::new();
    /// let arg = String::from("-");
    /// let res = arg_parser.process_short_flag(arg);
    /// assert_eq!(res.is_err(),true)
    /// ```
    pub fn process_short_flag(&mut self, arg: String) -> Result<(), ArgError> {
        let split: Vec<&str> = arg.split("-").collect();
        if split[1].len() == 0 {
            Err(ArgError::MissingFlag)
        } else {
            for character in split[1].chars() {
                let string_character = character.to_string();
                if self.short_flags.contains(&string_character) == false {
                    self.short_flags.push(string_character);
                }
            }
            Ok(())
        }
    }

    pub fn process_long_flag(&mut self, arg: String) -> Result<(), ArgError> {
        let key = arg.replace("--", "");
        if key.len() == 0 {
            // println!("Please provide flag/variable name after '--'");
            // std::process::exit(0);
            Err(ArgError::MissingFlagOrVariable)
        } else {
            self.long_flags.push(key);
            Ok(())
        }
    }

    pub fn process_variable(&mut self, arg: String) -> Result<(), ArgError> {
        let split: Vec<&str> = arg.split("=").collect();
        let key = split[0].replace("--", "");
        let value = split[1].to_string();
        if key.len() == 0 {
            Err(ArgError::MissingVariable)
        } else if value.len() == 0 {
            Err(ArgError::MissingValue)
        } else {
            self.variables.insert(key, split[1].to_string());
            Ok(())
        }
    }

    pub fn process_positional_argument(&mut self, arg: String) -> Result<(), ArgError> {
        self.positional.push(arg);
        Ok(())
    }

    pub fn parse(mut self) -> Self {
        for _ in 0..self.args.len() {
            if let Some(arg) = self.args.pop() {
                let mut res: Result<(), ArgError> = Result::Ok(());

                if arg.starts_with("-") == true && arg.starts_with("--") == false {
                    res = self.process_short_flag(arg);
                    // process_argument_error(res);
                } else if arg.starts_with("-") == true && arg.starts_with("--") == true {
                    if arg.contains("=") == true {
                        res = self.process_variable(arg);
                        // process_argument_error(res);
                    } else {
                        res = self.process_long_flag(arg);
                        // process_argument_error(res);
                    }
                } else if arg.starts_with("-") == false && arg.starts_with("--") == false {
                    res = self.process_positional_argument(arg);
                }
                process_argument_error(res);
            }
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialization() {
        let arg_parser = ArgParser::new();
        assert_eq!(arg_parser.args.len(), 0);
    }

    #[test]
    fn working_short_flag() {
        let mut arg_parser = ArgParser::new();
        let arg = String::from("-a");
        let res = arg_parser.process_short_flag(arg);
        assert_eq!(res.is_ok(), true)
    }
}
