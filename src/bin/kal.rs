use katana_ast_lint::config::ReporterMode;
use katana_ast_lint::{KalRunError, KatanaAstLint};
use std::env;
use std::process;

fn main() {
    let mut args = env::args().skip(1);
    match args.next().as_deref() {
        Some("check") => run_check(args),
        Some("--help" | "-h" | "help") => {
            print_usage();
            process::exit(0);
        }
        Some("--version" | "-v" | "version") => {
            println!("kal {}", env!("CARGO_PKG_VERSION"));
            process::exit(0);
        }
        Some(command) => {
            eprintln!("Error: Unknown command '{command}'");
            print_usage();
            process::exit(2);
        }
        None => {
            print_usage();
            process::exit(2);
        }
    }
}

fn print_usage() {
    println!("Usage: kal <command> [options]");
    println!();
    println!("Commands:");
    println!("  check    Execute the standard KAL rule set");
    println!("  help     Print this message");
    println!("  version  Print version information");
    println!();
    println!("Options for 'check':");
    println!("  --json   Force JSON output");
    println!("  --text   Force text output");
}

fn run_check(args: impl Iterator<Item = String>) {
    let mode_override = parse_reporter_mode(args).unwrap_or_else(|e| exit_with_error(&e));
    let mut linter = KatanaAstLint::try_from_workspace().unwrap_or_else(|e| exit_with_error(&e));

    if let Some(mode) = mode_override {
        linter = linter.with_reporter_mode(mode);
    }
    match linter.try_assert_clean() {
        Ok(()) => {}
        Err(KalRunError::Violations(_)) => process::exit(1),
        Err(error) => exit_with_error(&error),
    }
}

fn parse_reporter_mode(args: impl Iterator<Item = String>) -> Result<Option<ReporterMode>, String> {
    let mut mode_override = None;
    for arg in args {
        let mode = match arg.as_str() {
            "--json" => ReporterMode::Json,
            "--text" => ReporterMode::Text,
            _ => return Err(format!("Unknown option '{arg}'")),
        };
        if mode_override.is_some_and(|previous| previous != mode) {
            return Err("Cannot specify both --json and --text".to_string());
        }
        mode_override = Some(mode);
    }
    Ok(mode_override)
}

fn exit_with_error(error: &impl std::fmt::Display) -> ! {
    eprintln!("Error: {error}");
    process::exit(2);
}
