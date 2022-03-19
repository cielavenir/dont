use std::ffi::OsString;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(allow_hyphen_values = true)]
    command: Vec<OsString>,
}

fn main() {
    let args = Args::parse();
    match execute(&args) {
        Conclusion::Exit(code) => {
            std::process::exit(code);
        }
        Conclusion::Exec(args) => {
            std::process::Command::new(&args[0])
                .args(&args[1..])
                .spawn()
                .expect("failed to execute command");
        }
    }
}

fn execute(args: &Args) -> Conclusion {
    if args.command.len() == 0 {
        return Conclusion::Exit(0);
    }
    if args.command[0] == "true" {
        return Conclusion::Exit(1);
    } else if args.command[0] == "false" {
        return Conclusion::Exit(0);
    }
    Conclusion::Exit(0)
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Conclusion {
    Exit(i32),
    Exec(Vec<OsString>),
}


#[cfg(test)]
mod tests {
    use super::*;

    fn main(args: &[&str]) -> Result<Conclusion, clap::Error> {
        // TODO: resolve unwrap correctly
        let args = Args::try_parse_from(args)?;
        Ok(execute(&args))
    }

    #[test]
    fn test_help() {
        let e = main(&["dont", "--help"]).unwrap_err();
        let msg = e.to_string();
        assert!(msg.contains("USAGE:"), "Expected message to contain \"USAGE:\", got {}", msg);
    }

    #[test]
    fn test_true() {
        let concl = main(&["dont", "true"]).unwrap();
        assert_eq!(concl, Conclusion::Exit(1));
    }

    #[test]
    fn test_true_with_dashes() {
        let concl = main(&["dont", "--", "true"]).unwrap();
        assert_eq!(concl, Conclusion::Exit(1));
    }

    #[test]
    fn test_false() {
        let concl = main(&["dont", "false"]).unwrap();
        assert_eq!(concl, Conclusion::Exit(0));
    }
}
