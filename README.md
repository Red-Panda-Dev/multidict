# multidict

[![Crates.io Version](https://img.shields.io/crates/v/rust-anticaptcha?label=Version&style=flat&color=green)](https://crates.io/crates/rust-anticaptcha)
[![Crates.io Downloads (latest version)](https://img.shields.io/crates/dv/rust-anticaptcha?style=flat&label=Downloads&color=blue)](https://crates.io/crates/rust-anticaptcha)
![Crates.io MSRV](https://img.shields.io/crates/msrv/rust-anticaptcha?label=cargo)
[![Static Badge](https://img.shields.io/badge/docs-docs.rs-green?label=Documentation&labelColor=gray)](https://docs.rs/rust-anticaptcha/)


[![Build Doc](https://github.com/Red-Panda-Dev/rust-anticaptcha/actions/workflows/build_doc.yml/badge.svg?branch=master)](https://github.com/Red-Panda-Dev/rust-anticaptcha/actions/workflows/build_doc.yml)
[![Tests](https://github.com/Red-Panda-Dev/rust-anticaptcha/actions/workflows/test.yml/badge.svg?branch=master)](https://github.com/Red-Panda-Dev/rust-anticaptcha/actions/workflows/test.yml)
[![Build Dev](https://github.com/Red-Panda-Dev/rust-anticaptcha/actions/workflows/build_dev.yml/badge.svg?branch=master)](https://github.com/Red-Panda-Dev/rust-anticaptcha/actions/workflows/build_dev.yml)
[![Build Release](https://github.com/Red-Panda-Dev/rust-anticaptcha/actions/workflows/build_release.yml/badge.svg?branch=master)](https://github.com/Red-Panda-Dev/rust-anticaptcha/actions/workflows/build_release.yml)

The `multidict` crate was inspired by Python `MultiDict` [library](https://multidict.aio-libs.org/en/stable/).
`multidict` is useful for working with HTTP headers, URL query, form-data args etc.
HTTP Headers and URL query string require specific data structure: `multidict`.
It behaves mostly like a regular map but it may have several values for the same key and preserves insertion ordering

## How to install?

We recommend using the latest version of Rust. `rust-anticaptcha` supports Rust 2021.

Install by Cargo command:
```bash
cargo add multidict
```

Add line in your `Cargo.toml` file:
```toml
multidict = "0.0.1"
```

## How to test?

Run full tests (with docstrings):
```bash
cargo test
```
Or only project specified tests:
 ```bash
 cargo test --tests
 ```
