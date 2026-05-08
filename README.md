# bao

A small Rust CLI to open paths in your editor(s) and search the web from the terminal.

The crate is named `bao` but installs a single binary called `b` for quick typing.

## Features

- `b dev <path> [--cursor] [--code]` — open a project/file in [Cursor](https://cursor.sh) and/or [VS Code](https://code.visualstudio.com).
- `b search <query>` — open a Google search for the given query in your default browser.

## Requirements

- Rust toolchain (edition 2024) — install via [rustup](https://rustup.rs).
- For `b dev`: `cursor` and/or `code` available on your `PATH`.
- For `b search`: a default web browser configured for your OS.

## Installation

### From GitHub (recommended)

```bash
cargo install --git https://github.com/Leos0508/bao
```

To pin to a specific release:

```bash
cargo install --git https://github.com/Leos0508/bao --tag v0.1.0
```

Both commands install the `b` binary to `~/.cargo/bin` (make sure that directory is on your `PATH`).

### From a local clone

```bash
git clone https://github.com/Leos0508/bao
cd bao
cargo install --path .
```

Or build a release binary without installing:

```bash
cargo build --release
# Binary at ./target/release/b
```

## Usage

```bash
b <COMMAND> [ARGS]
```

### Open a project in your editor(s)

```bash
b dev .                       # open in both Cursor and VS Code (default)
b dev ~/Projects/some-repo    # open in both
b dev ./src/main.rs --cursor  # open in Cursor only
b dev . --code                # open in VS Code only
b dev . --cursor --code       # explicit "open in both"
```

By default, `b dev <path>` launches both Cursor and VS Code. Pass `--cursor` and/or `--code` to restrict which editor(s) get opened.

### Search the web

```bash
b search "rust clap derive example"
```

This opens `https://www.google.com/search?q=...` (URL-encoded) in your default browser.

### Help and version

```bash
b --help
b --version
b dev --help
b search --help
```

## Development

```bash
cargo run -- dev .
cargo run -- search "hello world"
cargo build
cargo test
```

## Project structure

```
.
├── Cargo.toml
├── Cargo.lock
└── src
    └── main.rs
```

## Dependencies

- [`clap`](https://crates.io/crates/clap) — argument parsing (derive feature).
- [`anyhow`](https://crates.io/crates/anyhow) — ergonomic error handling.
- [`open`](https://crates.io/crates/open) — open URLs/files with the OS default handler.
- [`urlencoding`](https://crates.io/crates/urlencoding) — URL-encode search queries.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## Author

Pham Hoang Bao &lt;baopham.201015@gmail.com&gt;
