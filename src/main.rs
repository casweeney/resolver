use resolver::Config;
use resolver::ClapperArgs;
use clap::Parser;

fn main() {
    let args: ClapperArgs = ClapperArgs::parse();

    match Config::resolve(args) {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Error: resolver failed: {}", e);
            return;
        }
    };
}
