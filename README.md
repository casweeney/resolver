# Resolver
Resolver is a CLI tool that enable developers to scaffold projects of different development tools and programming languages.

## Installations
Installing `resolver` requires that you already have `Rust` and `cargo` installed. Use the following command to install Rust and Cargo

```sh
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

To check whether you have Rust installed correctly, open a shell and run the command below:
```sh
$ rustc --version
```

Check whether cargo is installed:
```sh
$ cargo --version
```

Now that you have Rust and Cargo installed, run the command below to install `resolver`:
```sh
cargo install resolver
```

## Usage
With `resolver` you can create projects for different development tools and programming languages. Resolver has two action commands `get` and `scaffold`.

The `get` action is used to clone selected Diamond Standard Templates from GitHub which covers for Hardhat JavaScript, Hardhat TypeScript and Foundry.

The `scaffold` action is used to scaffold projects for different development tools and languages which includes:
- ReactJS
- ReactTs
- Hardhat
- NestJs
- Laravel
- NextJs

#### ReactJS
Creates a React project with JavaScript
```sh
resolver scaffold reactjs project_name
```

#### ReactTS
Creates a React project with TypeScript
```sh
resolver scaffold reactts project_name
```

#### Hardhat
Creates a Hardhat Solidity project
```sh
resolver scaffold hardhat project_name
```

#### NestJs
Creates a NestJS project
```sh
resolver scaffold nestjs project_name
```

#### Laravel
Creates a Laravel PHP project
```sh
resolver scaffold laravel project_name
```

#### NextJS
Creates a NextJS project
```sh
resolver scaffold nextjs project_name
```