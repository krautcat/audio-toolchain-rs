use std::ffi::OsString;
use clap::{arg, Command, ArgMatches};
use clap::error::{Result as ClapResult, Error as ClapError, ErrorKind as ClapErrorKind};

use crate::configuration::Configuration;

pub mod configuration;

enum CLIExitCodes {
    Help = 1,
    Version = 2,
    CLIParsingError = 3
}
struct CLIParser {
    main_command: Command
}

impl CLIParser {
    fn new() -> Self {
        let cmd = Command::new("kat")
            .version("1.0")
            .author("Georgiy Odisharia <math.kraut.cat@gmail.com>")
            .about("Krautcat audio toolchain")
            .subcommand_required(true)
            .arg_required_else_help(true)
            .subcommand(
                Command::new("tag")
                    .subcommand_required(true)
                    .arg_required_else_help(true)
                    .subcommand(
                        Command::new("set")
                            .arg_required_else_help(true)
                    )
            );

        return CLIParser {
            main_command: cmd
        };
    }
    
    fn parse(&mut self) -> ClapResult<ArgMatches, ClapError> {
        self.parse_from(std::env::args_os().into_iter())
    }

    fn parse_from<I, T>(&mut self, args: I) -> ClapResult<ArgMatches, ClapError>
    where
        I: Iterator<Item = T>,
        T: Into<OsString> + Clone
    {
        self.main_command.try_get_matches_from_mut(args)
    }

    fn print_help(&mut self) -> i32 {
        let _io_result = self.main_command.print_help();
        return CLIExitCodes::Help as i32;
    }

    fn print_version(&mut self) -> i32 {
        println!("{}", self.main_command.get_version().unwrap());
        return CLIExitCodes::Version as i32;
    }
}

fn main() {
    
    let mut parser = CLIParser::new();
    let matches = parser.parse();

    let args = match matches {
        Ok(m) => {
            m
        },
        Err(e) => {
            let exit_code: i32;
            match e.kind() {
                ClapErrorKind::DisplayHelp => {
                    exit_code = parser.print_help();
                }
                ClapErrorKind::DisplayVersion => {
                    exit_code = parser.print_version()
                }
                ClapErrorKind::InvalidSubcommand => {
                    parser.print_help();
                    exit_code = CLIExitCodes::CLIParsingError as i32; 
                }
                _ => {
                    exit_code = 3;
                }
            }
            std::process::exit(exit_code)
        }
    };

    let config = Configuration::new(&args).process_args(&args);

    println!("{}", config)
}
