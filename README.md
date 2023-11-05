# tracing-active-tree

`tracing-active-tree` is a Rust library that allows developers to retrieve the current calling stack at runtime on demand. The calling stack is organized as [span](https://docs.rs/tracing/latest/tracing/span/index.html) trees, and can be used for both synchronous and asynchronous contexts.

One common use case for this library is to print or log the calling trees on error for troubleshooting purposes. Unlike capturing stack backtraces using `std::backtrace::Backtrace`, `tracing-active-tree` offers several advantages:

- It supports asynchronous functions.
- It records trees with additional context attributes.

However, `tracing-active-tree` has a few disadvantages as well:

- Spans need to be declared beforehand.
- It requires more CPU and memory resources.

Two similar libraries, `async-backtrace` and `await-tree`, are also available. `tracing-active-tree` takes inspiration from their implementation. However, both `async-backtrace` and `await-tree` require declaring spans using methods provided by the library, which can be inconvenient for existing codebases. In contrast, `tracing-active-tree` does not require any changes to the existing code. It only requires registering an extra Layer to the tracing subscribers registry.

## Installation

Add the following line to your `Cargo.toml` file:

```toml
tracing-active-tree = { git = "https://github.com/tikv/tracing-active-tree.git", branch = "master" }
```

## Usage

*Run the following example using `cargo run --example simple`.*

1. Import the library:

    ```rust
    use tracing_active_tree::layer::{self, CurrentStacksLayer};
    ```

1. Create spans that you wish to display in the calling trees:

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

1. Register the `CurrentStacksLayer` to the tracing subscriber:

    ```rust
        tracing_subscriber::registry()
            .with(layer::global().clone())
            .init();
    ```

1. Dump the tree:

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

1. The following tree will be printed to the stdout:

    ```sh
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

`tracing-active-tree` is licensed under the Apache 2.0 license. See the [LICENSE](./LICENSE) file for more details.
