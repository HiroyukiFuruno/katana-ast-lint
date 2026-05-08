use katana_ast_lint::KatanaAstLint;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args[1] != "check" {
        eprintln!("Usage: kal check");
        process::exit(2);
    }

    match KatanaAstLint::try_from_workspace() {
        Ok(linter) => {
            if let Err(_e) = linter.try_assert_clean() {
                /* WHY: If try_assert_clean returns Err, it means there were lint violations
                   (Error severity) reported by the reporter.
                   The reporter already printed the details to stdout.
                   Note: According to our reporter implementation:
                   - Text mode: prints violations, then returns Err(msg) if any has Severity::Error
                   - Json mode: prints JSON, then returns Err if any has Severity::Error
                   So we just need to exit with 1.
                */

                /* WHY: However, we want to distinguish between "lint violations found" (1)
                   and "configuration/execution failure" (2).
                   Our try_from_workspace() and try_assert_clean() currently return String as error.
                */

                process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(2);
        }
    }
}
