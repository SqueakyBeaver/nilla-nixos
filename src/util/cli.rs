use clap::{
    CommandFactory, FromArgMatches, Parser,
    error::{ContextKind, ContextValue, Error, ErrorKind},
};
use nixos_cli_def::Cli;

pub struct Program {
    pub cli: Cli,
    pub extra_args: Vec<String>,
    pub subcommand: String,
}

pub fn parse_with_extra_args() -> Result<Program, Error> {
    let cmd = Cli::command().ignore_errors(true).get_matches();
    let mut cli = Cli::from_arg_matches(&cmd)?;

    let mut extra_args: Vec<String> = Vec::with_capacity(10);

    let (subcommand, _) = cmd.subcommand().unwrap_or(("", &cmd));

    loop {
        match cli.try_update_from(std::env::args_os().filter(|x| {
            !extra_args.contains(
                &x.clone()
                    .into_string()
                    .expect("Could not turn {x} into a normal String for some reason ;-;"),
            )
        })) {
            Ok(_) => break,
            Err(res) => match Some(res.kind()) {
                None => {}
                Some(ErrorKind::UnknownArgument) => match res.get(ContextKind::InvalidArg) {
                    Some(ContextValue::StyledStr(_)) => {}
                    Some(ContextValue::StyledStrs(_)) => {}
                    Some(ContextValue::Strings(s)) => extra_args.extend_from_slice(s),
                    Some(s) => extra_args.push(s.to_string()),
                    _ => {}
                },
                _ => {
                    println!("Whoops {}", res.kind());
                    res.exit();
                }
            },
        }
    }

    Ok(Program {
        cli: cli,
        extra_args: extra_args,
        subcommand: subcommand.to_string(),
    })
}
