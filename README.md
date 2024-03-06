# Devblog website

Written in Rust with support of [Leptos](https://github.com/leptos-rs/leptos) web framework and the [cargo-leptos](https://github.com/akesson/cargo-leptos) tool.

## Installing Additional Tools

By default, `cargo-leptos` uses `nightly` Rust, `cargo-generate`, and `sass`. If you run into any trouble, you may need to install one or more of these tools.

1. `rustup toolchain install nightly --allow-downgrade` - make sure you have Rust nightly
2. `rustup target add wasm32-unknown-unknown` - add the ability to compile Rust to WebAssembly
3. `cargo install cargo-generate` - install `cargo-generate` binary (should be installed automatically in future)
4. `npm install -g sass` - install `dart-sass` (should be optional in future)
5. If you don't have `cargo-leptos` installed you can install it with -
`$ cargo install cargo-leptos`

## Coding workflows
### Build

Install dev dependencies:`yarn install --frozen-lockfile`

Install cargo dependencies:`cargo build`
### Dev workflow
Then, to run project

Compiler check: `yarn test`

Launch web:`yarn web`

Launch tailwind: `yarn style`

You can access your local project at `http://localhost:9121`

## Environments
Set the following environment variables (updating for your project as needed):
```sh
export DEVLOGS_SURREAL_DB_SOCKET_ADDRESS="127.0.0.1:8181"
export DEVLOGS_SURREAL_DB_NAME="devlog"
export DEVLOGS_SURREAL_DB_NAMESPACE="test"
export DEVLOGS_SURREAL_DB_USERNAME="root"
export DEVLOGS_SURREAL_DB_PASSWORD="root"
export LEPTOS_OUTPUT_NAME="leptos_start"
export LEPTOS_SITE_ROOT="site"
export LEPTOS_SITE_PKG_DIR="pkg"
export LEPTOS_SITE_ADDR="127.0.0.1:3000"
export LEPTOS_RELOAD_PORT="3001"
```
