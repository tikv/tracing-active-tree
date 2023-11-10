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

### Finding something to work on

For beginners, we have prepared many suitable tasks for you. Checkout our [Help Wanted issues](https://github.com/tikv/tracing-active-tree/issues?q=is%3Aopen+is%3Aissue+label%3Astatus%2Fhelp-wanted) for a list, in which we have also marked the difficulty level.

If you are planning something big, for example, relates to multiple components or changes current behaviors, make sure to open an issue to discuss with us before going on.

### Linking issues

Code repositories in `tracing-active-tree` community require **ALL** the pull requests referring to its corresponding issues. In the pull request body, there **MUST** be one line starting with `Issue Number:` and linking the relevant issues via the [keyword](https://docs.github.com/en/issues/tracking-your-work-with-issues/linking-a-pull-request-to-an-issue#linking-a-pull-request-to-an-issue-using-a-keyword), for example:

If the pull request resolves the relevant issues, and you want GitHub to close these issues automatically after it merged into the default branch, you can use the syntax (`KEYWORD #ISSUE-NUMBER`) like this:

```issue-message
Issue Number: close #123
```

If the pull request links an issue but does not close it, you can use the keyword `ref` like this:

```issue-message
Issue Number: ref #456
```

Multiple issues should use full syntax for each issue and separate by a comma, like:

```issue-message
Issue Number: close #123, ref #456
```

For pull requests trying to close issues in a different repository, contributors need to first create an issue in the same repository and use this issue to track.

If the pull request body does not provide the required content, the bot will add the `do-not-merge/needs-linked-issue` label to the pull request to prevent it from being merged.

### Format of the commit message

The bot we use will extract the pull request title as the one-line subject and messages inside the `commit-message` code block as commit message body. For example, a pull request with title `pkg: what's changed in this one package` and body containing:

    ```commit-message
    any multiple line commit messages that go into
    the final commit message body

    * fix something 1
    * fix something 2
    ```

will get a final commit message:

```commit-message
pkg: what's changed in this one package (#12345)

any multiple line commit messages that go into
the final commit message body

* fix something 1
* fix something 2
```

The first line is the subject (the pull request title) and should be no longer than 50 characters, the other lines should be wrapped at 72 characters (see [this blog post](https://preslav.me/2015/02/21/what-s-with-the-50-72-rule/) for why).

If the change affects more than one subsystem, you can use comma to separate them like `util/codec,util/types:`.

If the change affects many subsystems, you can use ```*``` instead, like ```*:```.

The body of the commit message should describe why the change was made and at a high level, how the code works.

### Signing off the Commit

The project uses [DCO check](https://github.com/probot/dco#how-it-works) and the commit message must contain a `Signed-off-by` line for [Developer Certificate of Origin](https://developercertificate.org/).

Use option `git commit -s` to sign off your commits. The bot will group and distinguish the signatures from all your commits in the pull request and append them to the final commit message body.

Thanks for your contributions!
