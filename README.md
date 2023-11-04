# tracing-active-tree

`tracing-active-tree` allows Rust developers to retrieve the current calling stack at runtime on demand, organized as [span](https://docs.rs/tracing/latest/tracing/span/index.html) trees, for both synchronous and asynchronous context.

A typical use scenario is printing out or logging down the calling trees on error for trouble shooting. Comparing to capturing stack backtrace by [`std::backtrace::Backtrace`](https://doc.rust-lang.org/std/backtrace/struct.Backtrace.html), `tracing-active-tree` has the following advantages:
- Supports asynchronous functions
- Records trees with extra [attributes](https://docs.rs/tracing/latest/tracing/#configuring-attributes) of context.

At the same time, `tracing-active-tree` has the following disadvantages:
- [Spans](https://docs.rs/tracing/latest/tracing/span/index.html) requires prior declaration.
- Cost more CPU.

Besides, there are similar libraries, e.g, two candidates: [async-backtrace](https://github.com/tokio-rs/async-backtrace) and [await-tree](https://github.com/risingwavelabs/await-tree/). They are great libraries, and `tracing-active-tree` referenced the implementation of them. But both of them require declaring spans using the methods provides by the library, which we think will be not friendly to existed codes. `tracing-active-tree` do not require the change current codes, but just register an extra [Layer](https://docs.rs/tracing-subscriber/latest/tracing_subscriber/layer/trait.Layer.html) to registry of tracing [subscribers](https://docs.rs/tracing-subscriber/latest/tracing_subscriber/index.html).

## Installation

Add the following line to `Cargo.toml`:

```toml
tracing-active-tree = { git = "https://github.com/tikv/tracing-active-tree.git", branch = "master" }
```

## Usage

*Run the following example by `cargo run --example simple`.*

1. Import the library:

```rust
use tracing_active_tree::layer::{self, CurrentStacksLayer};
```

2. Creating spans which are wished to be display in the calling trees.
```rust
#[instrument(fields(answer = 43))]
async fn foo() {
    bar().await;
}

#[instrument]
async fn bar() {
    futures::join!(fiz(), buz());
}

#[instrument(skip_all)]
async fn fiz() {
    tokio::task::yield_now().await;
}

#[instrument(skip_all)]
async fn buz() {
    baz().await;
}

#[instrument(skip_all)]
async fn baz() {
}
```

3. Register the `CurrentStacksLayer` to tracing subscriber:
```rust
    tracing_subscriber::registry()
        .with(layer::global().clone())
        .init();
```

4. Dump the tree:
```rust
#[instrument(skip_all)]
async fn baz() {
    println!("{}", debug_dump_current_tree());
}

fn debug_dump_current_tree() -> String {
    let layer =
        dispatcher::get_default(|d| d.downcast_ref::<CurrentStacksLayer>().unwrap().clone());
    layer.fmt_string()
}
```

5. The following will be printed to stdout:
```
1
└[examples/simple.rs:5] [span_id=1] ["foo"] [answer=43] [elapsed=114.659µs]
 └[examples/simple.rs:10] [span_id=2] ["bar"] [elapsed=92.5µs]
  ├[examples/simple.rs:15] [span_id=3] ["fiz"] [elapsed=79.072µs]
  └[examples/simple.rs:20] [span_id=4] ["buz"] [elapsed=65.938µs]
   └[examples/simple.rs:25] [span_id=5] ["baz"] [elapsed=59.61µs]
```

## Benchmark

TODO

## Contributing

TODO

## License

TiKV is under the Apache 2.0 license. See the [LICENSE](./LICENSE) file for details.
