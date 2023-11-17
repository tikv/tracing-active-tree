# Contributing to `tracing-active-tree`

Thanks for your interest in contributing to `tracing-active-tree`! This document outlines some of the conventions on building, running, and testing `tracing-active-tree`, the development workflow, commit message formatting, contact points and other resources.

## Building and setting up a development workspace

`tracing-active-tree` is mostly written in Rust, We are currently using the Rust nightly toolchain.

### Prerequisites

To build `tracing-active-tree` you'll need to at least have the following installed:

* `git` - Version control
* [`rustup`](https://rustup.rs/) - Rust installer and toolchain manager
* `make` - Build tool (run common workflows)

### Getting the repository

```bash
git clone https://github.com/tikv/tracing-active-tree.git
cd tracing-active-tree
```

### Configuring your Rust toolchain

`rustup` is the official toolchain manager for Rust, similar to `rvm` or `rbenv` from the Ruby world.

`tracing-active-tree` is pinned to a version of Rust using a `rust-toolchain` file. `rustup` and `cargo` will automatically use this file. We also use the `rustfmt` and `clippy` components, to install those:

```bash
rustup component add rustfmt
rustup component add clippy
```

### Building and testing

During interactive development, you may prefer using `cargo check`, which will parse, borrow check, and lint your code, but not actually compile it:

```bash
cargo check --all
```

`nextest` is necessary for `make test` & `make unit-test` to work. Install the crate with:

```bash
cargo install cargo-nextest --locked
```

You can run the test suite alone, or just run a specific test:

```bash
# Run the full suite
make test
```

`tracing-active-tree` follows the Rust community coding style. We use Rustfmt and [Clippy](https://github.com/Manishearth/rust-clippy) to automatically format and lint our code. Using these tools is checked in our CI. These are as part of `make check`.

```bash
make check
```

### Submitting a Pull Request

Following the [Commit Message and Pull Request Style](https://github.com/pingcap/community/blob/master/contributors/commit-message-pr-style.md) will help with the review process by making your change easier to understand.

### Signing off the Commit

The project uses [DCO check](https://github.com/probot/dco#how-it-works) and the commit message must contain a `Signed-off-by` line for [Developer Certificate of Origin](https://developercertificate.org/).

Use option `git commit -s` to sign off your commits. The bot will group and distinguish the signatures from all your commits in the pull request and append them to the final commit message body.

Thanks for your contributions!
