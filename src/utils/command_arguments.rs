use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct ClapperArgs {
    #[clap(subcommand)]
    pub entity_type: EntityType,
}

// ---------------
// Valid Commands: get | scaffold
// ---------------
#[derive(Debug, Subcommand)]
pub enum EntityType {
    /// Clones projects boilerplate for diamond standard (JavaScript, TypeScript and Foundry) and NestJs
    Get(GetCommand),
    /// Scaffolds projects for any development tool
    Scaffold(ScaffoldCommand),
    /// Installs dependencies and software development tools
    Install(InstallCommand),
}

// ----------------
// GetCommand Args
// ----------------
#[derive(Debug, Args)]
pub struct GetCommand {
    #[clap(subcommand)]
    pub command: GetSubCommand,
}

#[derive(Debug, Subcommand)]
pub enum GetSubCommand {
    /// Clones a diamond standard JavaScript project
    Dhjs(GetDir),
    /// Clones a diamond standard TypeScript project
    Dhts(GetDir),
    /// Clones a diamond standard Foundry project
    Dfd(GetDir),
    /// Clones a NestJS project boilerplate
    Nestjs(GetDir),
}

// --------------------
// ScaffoldCommand Args
// --------------------
#[derive(Debug, Args)]
pub struct ScaffoldCommand {
    #[clap(subcommand)]
    pub command: ScaffoldSubCommand,
}

#[derive(Debug, Subcommand)]
pub enum ScaffoldSubCommand {
    /// Scaffolds a Hardhat project
    Hardhat(GetDir),
    /// Scaffolds a NestJS project
    Nestjs(GetDir),
    /// Scaffolds a Laravel project
    Laravel(GetDir),
    /// Scaffolds a Next project
    Nextjs(GetDir),
    /// Scaffolds a Foundry project
    Foundry(GetDir),
    /// Scaffold a Vue.js project
    Vue(GetDir),
    /// Scaffold (Vanilla TypeScript, Vue, React, Preact, Lit, Svelte) projects using Vite
    Vite(GetDir),
    /// Scaffold a Noir project
    Noir(GetDir),
    /// Scaffold a Starknet Foundry project
    Snforge(GetDir),
    /// Scaffold a RainbowKit + Wagmi + Next.js project
    RainbowKit(GetDir),
    /// Scafold a React-Native Expo project
    ReactNativeExpo(GetDir),
    /// Scafold an Adonis.js Project
    Adonis(GetDir),
    /// Scaffolds a create-react-app project
    React(GetDir),
}

// ----------------
// InstallCommand Args
// ----------------
#[derive(Debug, Args)]
pub struct InstallCommand {
    #[clap(subcommand)]
    pub command: InstallSubCommand,
}

#[derive(Debug, Subcommand)]
pub enum InstallSubCommand {
    /// Installs Homebrew
    Brew,
    /// Installs Chocolatey
    Choco,
    /// Installs Node.js
    Node,
    /// Installs Scarb
    Scarb,
    /// Installs Forge
    Forge,
    /// Installs Starkli
    Starkli,
    /// Installs Nargo
    Noir,
    // installs Starknet Foundry
    Snfoundry(Version),
}

// --------------------------------------
// GetDir: For passing the directory name
// --------------------------------------
#[derive(Debug, Args)]
pub struct GetDir {
    /// Specifies the name of the project directory to initialize
    pub dir_name: String,
}

#[derive(Debug, Args)]
pub struct Version {
    pub version_name: String,
}
