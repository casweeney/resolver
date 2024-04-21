use resolver_cli::resolve;
use resolver_cli::ClapperArgs;
use clap::Parser;
use colored::*;

fn main() {
    let args: ClapperArgs = ClapperArgs::parse();

    match resolve(args) {
        Ok(cfg) => {
            println!("{}", 
"---------------------------
Success: Happy building !!!
---------------------------"
.green().bold()
            );
            cfg
        },
        Err(e) => {
            eprintln!("{}", format!("Error: resolver failed: {}", e).red().bold().underline());
            return;
        }
    };
}
