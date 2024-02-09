use clap::{Id, Arg, ArgAction, Command, value_parser};

pub (crate) enum Args {
  Input,
  Output,
  Version
}

impl Into<Id> for Args {
  fn into(self) -> Id {
    match self {
      Args::Input => Id::from("input"),
      Args::Output => Id::from("output"),
      Args::Version => Id::from("version")
    }
  }
}

impl Into<&'static str> for Args {
  fn into(self) -> &'static str {
    match self {
      Args::Input => "input",
      Args::Output => "output",
      Args::Version => "version"
    }
  }
}

pub (crate) fn cli() -> Command {
  Command::new("hash-tree-crypt")
    .version(std::env!("CARGO_PKG_VERSION"))
    .about("Encrypts a file using a hash tree")
    .arg(Arg::new(Args::Input)
      .short('i')
      .long("input")
      .value_name("FILE")
      .help("The file to encrypt (or decrypt)")
      .value_parser(value_parser!(std::path::PathBuf)) 
      .value_hint(clap::ValueHint::FilePath)
      .required_unless_present(Args::Version)
    )
    .arg(Arg::new(Args::Output)
      .short('o')
      .long("output")
      .value_name("FILE")
      .help("The output file")
      .value_parser(value_parser!(std::path::PathBuf))
      .value_hint(clap::ValueHint::FilePath)
      .required(false)
    )
    .arg(Arg::new(Args::Version)
      .short('v')
      .long("version")
      .help("Prints version information")
      .action(ArgAction::SetTrue)
    )
}
