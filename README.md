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
Then install dependencies by

`yarn install --frozen-lockfile`

And

`cargo build`

Then, to run project

`cargo leptos watch`

And, start another task for compiling tailwind on the fly

`yarn tailwindcss -i ./global.scss -o style/main.css --watch`

By default, you can access your local project at `http://localhost:3000`

## Environments
Set the following environment variables (updating for your project as needed):
```sh
export LEPTOS_OUTPUT_NAME="leptos_start"
export LEPTOS_SITE_ROOT="site"
export LEPTOS_SITE_PKG_DIR="pkg"
export LEPTOS_SITE_ADDR="127.0.0.1:3000"
export LEPTOS_RELOAD_PORT="3001"
```
