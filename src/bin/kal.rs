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
    println!("Usage: kal <command>");
    println!();
    println!("Commands:");
    println!("  check    Execute the standard KAL rule set");
    println!("  help     Print this message");
    println!("  version  Print version information");
}

fn run_check() {
    match KatanaAstLint::try_from_workspace() {
        Ok(linter) => {
            if let Err(_e) = linter.try_assert_clean() {
                /* WHY: Exit with 1 if lint violations (Error severity) are found.
                   Violation details are already printed to stdout by the reporter. */
                process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(2);
        }
    }
}
