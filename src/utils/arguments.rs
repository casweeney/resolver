use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct ClapperArgs {
    #[clap(subcommand)]
    pub entity_type: EntityType,
}

#[derive(Debug, Subcommand)]
pub enum EntityType {
    /// Clones repositories
    Get(GetCommand),
    Scaffold(ScaffoldCommand),
}

#[derive(Debug, Args)]
pub struct GetCommand{
    #[clap(subcommand)]
    pub command: GetSubCommand,
}

#[derive(Debug, Args)]
pub struct ScaffoldCommand{
    #[clap(subcommand)]
    pub command: ScaffoldSubCommand,
}



#[derive(Debug, Subcommand)]
pub enum GetSubCommand {
    Dhjs(GetDir),
    Dhts(GetDir),
    Dfd(GetDir),
    Nestjs(GetDir)
}

#[derive(Debug, Subcommand)]
pub enum ScaffoldSubCommand {
    Reactjs(GetDir),
    Reactts(GetDir),
    Hardhat(GetDir),
    Nestjs(GetDir),
    Laravel(GetDir),
    Nextjs(GetDir),
}

#[derive(Debug, Args)]
pub struct GetDir {
    /// Specifies the name of the project directory to initialize
    pub dir_name: String
}