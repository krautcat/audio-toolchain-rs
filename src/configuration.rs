use std::str::FromStr;

use clap::{Arg, ArgMatches};

trait ConsumesCLIArgs {
    fn process_args(&mut self, args: &ArgMatches) -> &Self;
}
pub struct ConfigurationTagSet {
    tag: String,
    value: String,
}

impl ConfigurationTagSet {
    fn new() -> Self {
        ConfigurationTagSet {
            tag: String::from_str("").unwrap(),
            value: String::from_str("").unwrap(),
        }
    }
}

impl ConsumesCLIArgs for ConfigurationTagSet {
    fn process_args(&mut self, args: &ArgMatches) -> &Self {
        self.tag = args.get_one::<String>("tag").unwrap().to_string();
        self.value = args.get_one::<String>("value").unwrap().to_string();
        self
    }
}
enum ConfigurationSubcommandTag {
    Set(ConfigurationTagSet),
}
pub struct ConfigurationTag {
    subcommand_config: ConfigurationSubcommandTag,
}

impl ConfigurationTag {
    fn new(args: &ArgMatches) -> Self {
        match args.subcommand() {
            Some(("set", _)) => ConfigurationTag {
                subcommand_config: ConfigurationSubcommandTag::Set(ConfigurationTagSet::new()),
            },
        }
    }
}

impl ConsumesCLIArgs for ConfigurationTag {
    fn process_args(&mut self, args: &ArgMatches) -> &Self {
        self
    }
}
pub struct ConfigurationSort {}

impl ConfigurationSort {
    fn new(args: &ArgMatches) -> Self {
        ConfigurationSort {}
    }
}

impl ConsumesCLIArgs for ConfigurationSort {
    fn process_args(&mut self, args: &ArgMatches) -> &Self {
        self
    }
}
enum ConfigurationSubcommand {
    Tag(ConfigurationTag),
    Sort(ConfigurationSort),
}
pub struct Configuration {
    subcommand_config: ConfigurationSubcommand,
}

impl Configuration {
    pub fn new(args: &ArgMatches) -> Self {
        match args.subcommand() {
            Some(("tag", sub_matches)) => Configuration {
                subcommand_config: ConfigurationSubcommand::Tag(ConfigurationTag::new(sub_matches)),
            },
            Some(("sort", sub_matches)) => Configuration {
                subcommand_config: ConfigurationSubcommand::Sort(ConfigurationSort::new(
                    sub_matches,
                )),
            },
        }
    }

    pub fn process_args(&mut self, args: &ArgMatches) -> &Self {
        match self.subcommand_config {
            ConfigurationSubcommand::Sort(o) => o.process_args(args),
            ConfigurationSubcommand::Tag(o) => o.process_args(args)
        }
        self
    }
}
