use clap::{Arg, ArgAction, Command};

pub fn command_line() -> clap::ArgMatches {
  let matches = Command::new("core-engine")
    .about("Core Engine: a powerful engine for JavaScript and TypeScript.")
    .subcommand_required(true)
    .arg_required_else_help(true)
    .author("yazaldefi <yazaldefilimon@gmail.com>")
    .subcommand(
      Command::new("run")
        .about("run a javascript or typescript file.")
        .arg(
          Arg::new("file")
            .help("the javascript or typescript file to execute.")
            .required(true),
        )
        .arg(
          Arg::new("debug")
            .short('d')
            .long("debug")
            .help("enable the disassembler and debugger for detailed analysis.")
            .action(clap::ArgAction::SetTrue),
        ),
    )
    .subcommand(
      Command::new("compile")
        .about("compile a javascript or typescript file to bytecode.")
        .arg(
          Arg::new("file")
            .help("the javascript or typescript file to compile.")
            .required(true),
        )
        .arg(
          Arg::new("debug")
            .short('d')
            .long("debug")
            .action(ArgAction::SetTrue)
            .help("enable the disassembler and debugger for detailed analysis during compilation."),
        ),
    )
    .get_matches();

  return matches;
}
