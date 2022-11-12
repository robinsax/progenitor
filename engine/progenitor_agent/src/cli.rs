use std::env;
use std::collections::HashMap;
use std::fmt::Display;

pub enum CLIError {
    Args(String)
}

impl Display for CLIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Args(message) => write!(f, "{}", message)
        }
    }
}

pub struct CLIOptionTemplate {
    pub key: &'static str,
    pub key_shorthand: Option<&'static str>,
    pub takes_value: bool,
    pub description: &'static str
}

pub struct CLIVerbTemplate {
    pub verb: &'static str,
    pub options: Vec<CLIOptionTemplate>,
    pub description: &'static str
}

pub struct CLITemplate {
    pub verbs: Vec<CLIVerbTemplate>
}

impl Display for CLITemplate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for verb in self.verbs.iter() {
            write!(f, "  {}\n      {}\n", verb.verb, verb.description)?;

            for option in verb.options.iter() {
                write!(f, "   --{}", option.key)?;
                if let Some(shorthand) = &option.key_shorthand {
                    write!(f, " (or -{})", shorthand)?;
                }
                if option.takes_value {
                    write!(f, " <value>")?;
                }
                write!(f, "\n      {}\n", option.description)?;
            }
        }

        Ok(())
    }
}

pub struct CLIArgs {
    pub verb: String,
    pub options: HashMap<String, String>
}

impl CLIArgs {
    pub fn from_env(template: CLITemplate) -> Result<Self, CLIError> {
        let mut args = env::args();

        args.next().unwrap();

        macro_rules! error {
            ($($v: tt)+) => {
                CLIError::Args(format!("{}\n\nusage:\n{}", format!($($v)+), template))
            };
        }

        let verb_str = args.next().ok_or_else(|| error!("missing verb"))?;

        let verb = template.verbs.iter()
            .find(|vt| vt.verb == verb_str)
            .ok_or_else(|| error!("unknown verb {}", verb_str))?;

        let mut options: HashMap<String, String> = HashMap::new();

        loop {
            let arg = match args.next() {
                Some(arg) => arg,
                None => break
            };

            let mut parse_option = |option: &CLIOptionTemplate| {
                let mut value = "".to_owned();
                if option.takes_value {
                    value = match args.next() {
                        Some(arg) => arg,
                        None => return Err(error!("missing value for option {}", option.key))
                    };
                }

                options.insert(option.key.to_owned(), value);
                Ok(())
            };

            if arg.starts_with("--") {
                let key = &arg[2..];
                
                let option = verb.options.iter()
                    .find(|ot| ot.key == key)
                    .ok_or_else(|| error!("unknown option {}", key))?;

                parse_option(option)?;
            }
            else if arg.starts_with("-") {
                let key = &arg[1..];

                let option = verb.options.iter()
                    .find(|ot| {
                        if let Some(shorthand) = &ot.key_shorthand {
                            *shorthand == key
                        }
                        else { false }
                    })
                    .ok_or_else(|| error!("unknown option shorthand {}", key))?;

                parse_option(option)?;
            }
        }

        Ok(Self {
            verb: verb.verb.to_owned(),
            options
        })
    }
}
