# Resolver-cli
Resolver is a CLI tool that enable developers to scaffold projects for different development purpose, tools and programming languages.

## Installations
Installing `resolver-cli` requires that you already have `Rust` , and `cargo` installed. Use the following command to install Rust and Cargo:

```sh
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

To check whether you have Rust installed correctly, open a shell, and run the command below:
```sh
$ rustc --version
```

Check whether cargo is installed:
```sh
$ cargo --version
```

Now that you have Rust and Cargo installed, run the command below to install `resolver-cli`:
```sh
cargo install resolver-cli
```

## Usage
With `resolver-cli` you can create projects for different development tools and programming languages. Resolver-cli has three action commands `get`, `scaffold`, and `install`.

The `get` action is used to clone selected Diamond Standard Templates from GitHub which covers for Hardhat JavaScript, Hardhat TypeScript, and Foundry.

#### Diamond Standard Hardhat JavaScript
Creates a project boilerplate with Diamond Standard Hardhat JavaScript
```sh
resolver-cli get dhjs project_name
```

#### Diamond Standard Hardhat TypeScript
Creates a project boilerplate with Diamond Standard Hardhat TypeScript
```sh
resolver-cli get dhts project_name
```

#### Diamond Standard Foundry
Creates a project boilerplate with Diamond Standard Foundry
```sh
resolver-cli get dfd project_name
```


The `scaffold` action is used to scaffold projects for different development tools and languages which includes:
- ReactJS
- ReactTs
- Hardhat
- NestJs
- Laravel
- NextJs
- Foundry
- Vue
- Vite
- Starknet Foundry
- RainbowKit + Wagmi + Next.js
- ReactNative Expo
- Adonis.js

#### React
Creates a React project with a prompt to choose the scaffold type either js or ts
```sh
resolver-cli scaffold react project_name
```

#### Hardhat
Creates a Hardhat Solidity project
```sh
resolver-cli scaffold hardhat project_name
```

#### NestJs
Creates a NestJS project
```sh
resolver-cli scaffold nestjs project_name
```

#### Laravel
Creates a Laravel PHP project
```sh
resolver-cli scaffold laravel project_name
```

#### NextJS
Creates a NextJS project
```sh
resolver-cli scaffold nextjs project_name
```

#### Foundry
Creates a new foundry project
```sh
resolver-cli scaffold foundry project_name
```

#### Vue
Creates a Vue.js project
```sh
resolver-cli scaffold vue project_name
```

#### Vite
Creates (Vanilla TypeScript, Vue, React, Preact, Lit, Svelte) project using Vite
```sh
resolver-cli scaffold vite project_name
```

#### Starknet Foundry
Creates a starknet project
```sh
resolver-cli scaffold snforge project_name
```

#### RainbowKit + Wagmi + Next.js
Creates a rainbowKit + wagmi + next.js project
```sh
resolver-cli scaffold rainbowkit project_name
```

#### ReactNative Expo
Creates a ReactNative Expo project
```sh
resolver-cli scaffold reactnativeexpo project_name
```

#### Adonis.js
Creates an Adonis.js project
```sh
resolver-cli scaffold adonis project_name
```

The `install` action installs development tools like Node.js, Homebrew, Choco, Scarb, e.t.c. 

- To install node, run:
```resolver-cli install node```

- To install homebrew, run:
```resolver-cli install brew```

- To install choco, run:
```resolver-cli install choco```

- To install scarb, run:
```resolver-cli install scarb```

- To install latest version of starknet foundry, run:
```resolver-cli install snfoundry latest```

- To install a specific version of starknet foundry, run:
```resolver-cli install snfoundry $versionName```

Run `resolver-cli install --help` to see all supported installation tools.
