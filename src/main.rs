use resolver::resolve;
use resolver::ClapperArgs;
use clap::Parser;

fn main() {
    let args: ClapperArgs = ClapperArgs::parse();

    match resolve(args) {
        Ok(cfg) => {
            println!(
"
---------------------------
Success: Happy building !!!
---------------------------
"
            );
            cfg
        },
        Err(e) => {
            eprintln!("Error: resolver failed: {}", e);
            return;
        }
    };
}
