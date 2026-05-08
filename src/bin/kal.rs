use katana_ast_lint::KatanaAstLint;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        process::exit(2);
    }

    match args[1].as_str() {
        "check" => run_check(),
        "--help" | "-h" | "help" => {
            print_usage();
            process::exit(0);
        }
        "--version" | "-v" | "version" => {
            println!("kal {}", env!("CARGO_PKG_VERSION"));
            process::exit(0);
        }
        _ => {
            eprintln!("Error: Unknown command '{}'", args[1]);
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

fn run_check() {
    let args: Vec<String> = env::args().collect();
    let mut mode_override = None;

    for arg in args.iter().skip(2) {
        match arg.as_str() {
            "--json" => {
                if mode_override == Some(katana_ast_lint::config::ReporterMode::Text) {
                    eprintln!("Error: Cannot specify both --json and --text");
                    process::exit(2);
                }
                mode_override = Some(katana_ast_lint::config::ReporterMode::Json);
            }
            "--text" => {
                if mode_override == Some(katana_ast_lint::config::ReporterMode::Json) {
                    eprintln!("Error: Cannot specify both --json and --text");
                    process::exit(2);
                }
                mode_override = Some(katana_ast_lint::config::ReporterMode::Text);
            }
            _ => {
                eprintln!("Error: Unknown option '{}'", arg);
                process::exit(2);
            }
        }
    }

    match KatanaAstLint::try_from_workspace() {
        Ok(mut linter) => {
            if let Some(mode) = mode_override {
                linter = linter.with_reporter_mode(mode);
            }
            if let Err(e) = linter.try_assert_clean() {
                match e {
                    katana_ast_lint::KalRunError::Violations(_) => {
                        process::exit(1);
                    }
                    _ => {
                        eprintln!("Error: {e}");
                        process::exit(2);
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(2);
        }
    }
}
