use clap::{Id, Arg, ArgAction, Command, value_parser};

pub (crate) enum Args {
  Debug,
  Input,
  Output,
  Version
}

impl Into<Id> for Args {
  fn into(self) -> Id {
    match self {
      Args::Debug => Id::from("debug"),
      Args::Input => Id::from("input"),
      Args::Output => Id::from("output"),
      Args::Version => Id::from("version")
    }
  }
}

impl Into<&'static str> for Args {
  fn into(self) -> &'static str {
    match self {
      Args::Debug => "debug",
      Args::Input => "input",
      Args::Output => "output",
      Args::Version => "version"
    }
  }
}

pub (crate) fn cli(stdin_isnt_tty: bool) -> Command {
  let input_arg = Arg::new(Args::Input)
      .short('i')
      .long("input")
      .value_name("FILE")
      .help("The file to encrypt (or decrypt)")
      .value_parser(value_parser!(std::path::PathBuf)) 
      .value_hint(clap::ValueHint::FilePath);

  Command::new("hash-tree-crypt")
    .about("Encrypts a file using a hash tree")
    .arg(
      if stdin_isnt_tty {
        input_arg.required(false)
      } else {
        input_arg.required_unless_present(Args::Version)
      }
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
      .long("version")
      .help("Prints version information")
      .action(ArgAction::SetTrue)
    )
    .arg(Arg::new(Args::Debug)
      .long("debug")
      .help("Prints debug information")
      .action(ArgAction::SetTrue)
    )
}
