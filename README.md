<br />

<h1 align="center">Speedy NAPI-RS template</h1>

## Use this template

Click `use this template` above, then find and replace `speedy-sourcemap` to your desired naming.

## Setup Rust toolchain

This project uses [Rust](https://www.rust-lang.org/) and [NAPI-RS](https://github.com/napi-rs/napi-rs), so you can install it with:

```bash
# Install Rust toolchain on MacOS, Linux.
# For Windows users, you may refer to the website listed above and install the corresponding `.exe`.
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

This command will install the latest stable version of Rust standard library, Cargo and other toolchain.

This project is also using Rust 2021, for those developers who have already installed Rust, you may update your toolchain to the latest stable version, which you can update it as follows:

```bash
rustup update stable
```

For more information about the setup, please heads to [Rust](https://www.rust-lang.org/) and [NAPI-RS](https://github.com/napi-rs/napi-rs)

## Setup Node.js

Pnpm is required for handling node modules, also pnpm workspace is enabled for the project, so you can install node modules with:

```bash
npm install -g pnpm

pnpm install # Install all node modules in the workspace
```

## Setup WebAssembly

```bash
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

Note: for Apple Silicons, you need to use `cargo install wasm-pack` to install wasm-pack

## Repo Structure

```
.
├── Cargo.lock
├── Cargo.toml
├── README.md
├── benchmark # Your benchmark goes here
├── core
│   └── your_native_repo_goes_here # Your native repo goes here
├── node
│   ├── Cargo.toml
│   ├── __tests__
│   │   └── unit.spec.ts
│   ├── binding.js
│   ├── build.rs
│   ├── index.d.ts
│   ├── npm # This package is not intended for editing
│   ├── package.json
│   ├── src
│   │   ├── lib.rs
│   │   ├── test.rs
│   │   └── types.rs
│   └── tsconfig.json
├── wasm # Your wasm binding goes here
├── package.json
├── pnpm-workspace.yaml
└── rustfmt.toml # This file is not intended for editing
```

This project is also using Cargo workspaces for managing multiple crates.

## Development

**Build Node bindings**

```bash
# This will only build the unoptimized version for testing
# , which is hugely useful when testing projects with heavy dependencies.
pnpm build:debug --dir node

# Regular optimized build
pnpm build --dir node
```

**Build WebAssembly bindings**

```bash
pnpm build:wasm
```

## Testing

**Rust**

```bash
cargo test # This will test the whole cargo workspaces, and workspace members are declared in the <project-root>/Cargo.toml
```

**Node**

```bash
pnpm test # This will run tests under the `node` and `wasm` directory
```

## Publishing

**Rust(Cargo)**

```bash
cargo publish
```

**Node**

Node addons are divided into difference npm packages for different platforms and architectures, which you can find [here](./node/npm) and this is not intended to be edited manually.

```bash
# Create a new npm release
cd node
npm version
```

Make sure you have your commit message starts with `chore(release): publish`, and CI will automatically publish the release.
For more details please refer to [CI.yaml](.github/workflows/CI.yaml)
