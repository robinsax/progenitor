mod cli;
mod errors;
mod author;
mod port;

use std::process;
use std::io::{Read, Write, stdin, stdout};
use std::fs;

use bytes::Bytes;

use progenitor::{SerialValue, SerialFormat};
use progenitor::ext::JsonSerial;

use self::errors::ExecError;
use self::cli::{CLIArgs, CLITemplate, CLIVerbTemplate, CLIOptionTemplate};
use self::author::{author, AuthorInput};
use self::port::{port, PortInput, PortOperation};

fn cli_template() -> CLITemplate {
    CLITemplate { verbs: vec![
        CLIVerbTemplate {
            verb: "help",
            options: Vec::new(),
            description: "show usage"
        },
        CLIVerbTemplate {
            verb: "author",
            options: vec![
                CLIOptionTemplate {
                    key: "in",
                    key_shorthand: Some("i"),
                    takes_value: true,
                    description: "file path to read from, or 'stdin'"
                },
                CLIOptionTemplate {
                    key: "out",
                    key_shorthand: Some("o"),
                    takes_value: true,
                    description: "file path to write to, or 'stdout'"
                },
                CLIOptionTemplate {
                    key: "format",
                    key_shorthand: Some("f"),
                    takes_value: true,
                    description: "input format"
                },
                CLIOptionTemplate {
                    key: "module",
                    key_shorthand: Some("m"),
                    takes_value: false,
                    description: "whether to output a complete module (as opposed to a snippet)"
                }
            ],
            description: "author .rs from an archetype definition"
        },
        CLIVerbTemplate {
            verb: "port",
            options: vec![
                CLIOptionTemplate {
                    key: "op",
                    key_shorthand: None,
                    takes_value: true,
                    description: "operation to perform"
                },
                CLIOptionTemplate {
                    key: "target",
                    key_shorthand: Some("t"),
                    takes_value: true,
                    description: "target directory"
                },
                CLIOptionTemplate {
                    key: "name",
                    key_shorthand: Some("n"),
                    takes_value: true,
                    description: "project name"
                }
            ],
            description: "manage cargo repository"
        }
    ] }
}

fn read_input(src: &String) -> Result<Bytes, ExecError> {
    if src == "stdin" {
        let mut raw_bytes = Vec::new();
        match stdin().read_to_end(&mut raw_bytes) {
            Err(err) => Err(ExecError::Io(format!("input read failed: {}", err))),
            Ok(_) => Ok(Bytes::from(raw_bytes))
        }
    }
    else {
        match fs::read(src) {
            Err(err) => Err(ExecError::Io(format!("invalid input source: {}", err))),
            Ok(bytes) => Ok(Bytes::from(bytes))
        }
    }
}

fn write_output(dest: &String, output: Bytes) -> Result<(), ExecError> {
    if dest == "stdout" {
        match stdout().write(&output) {
            Ok(_) => Ok(()),
            Err(err) => Err(ExecError::Io(format!("output write failed: {}", err)))
        }
    }
    else {
        match fs::write(dest, output) {
            Ok(_) => Ok(()),
            Err(err) => Err(ExecError::Io(format!("output write failed: {}", err)))
        }
    }
}

fn main() {
    macro_rules! error_exit {
        ($($v: tt)+) => {
            println!($($v)+);
            process::exit(1);
        };
    }

    macro_rules! handle_result {
        ($r: expr) => {
            match $r {
                Err(err) => { error_exit!("{}", err); },
                Ok(value) => value
            }
        }
    }

    let args = match CLIArgs::from_env(cli_template()) {
        Ok(args) => args,
        Err(err) => { error_exit!("{}", err); }
    };

    macro_rules! get_required_option {
        ($k: literal, $e: literal) => {
            match args.options.get($k) {
                Some(value) => value,
                None => { error_exit!($e); }
            }
        };
    }

    match args.verb.as_str() {
        "help" => {
            println!("usage:\n{}", cli_template());
        },
        "author" => {
            let input_src = get_required_option!("in", "no input specified");
            let input_format_str = get_required_option!("format", "no input format specified");
            let output_dest = get_required_option!("out", "no output specified");
            let as_module = args.options.contains_key("module");

            let input = handle_result!(read_input(input_src));
            let input_serial: SerialValue = SerialValue::Buffer(input);
    
            let input_value = match input_format_str.as_str() {
                "json" => handle_result!(JsonSerial::new().parse(input_serial)),
                _ => { error_exit!("unsupported format {}", input_format_str); }
            };

            let output = handle_result!(author(AuthorInput {
                value: input_value,
                as_module
            }));

            handle_result!(write_output(output_dest, output));
        },
        "port" => {
            let op = match get_required_option!("op", "no operation specified").as_str() {
                "create" => PortOperation::Create {
                    name: get_required_option!("name", "no name specified").to_owned()
                },
                _ => { error_exit!("invalid operation"); }
            };
            let target = get_required_option!("target", "no target directory specified").to_owned();

            handle_result!(port(PortInput {
                op,
                target
            }));
        },
        _ => { error_exit!("invalid verb"); }
    }
}
