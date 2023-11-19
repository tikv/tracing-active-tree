# tracing-active-tree

`tracing-active-tree` is a Rust library that allows developers to retrieve the current calling stack at runtime on demand. The calling stack is organized as [span](https://docs.rs/tracing/latest/tracing/span/index.html) trees, and can be used for both synchronous and asynchronous contexts.

One common use case for this library is to print or log the calling trees on error for troubleshooting purposes. Unlike capturing stack backtraces using [`std::backtrace::Backtrace`](https://doc.rust-lang.org/std/backtrace/struct.Backtrace.html), `tracing-active-tree` offers several advantages:

- It supports asynchronous functions.
- It records trees with additional context [attributes](https://docs.rs/tracing/latest/tracing/#configuring-attributes).

However, `tracing-active-tree` has a few disadvantages as well:

- [Spans](https://docs.rs/tracing/latest/tracing/span/index.html) need to be declared beforehand.
- It requires more CPU and memory resources.

Two similar libraries, [`async-backtrace`](https://github.com/tokio-rs/async-backtrace) and [`await-tree`](https://github.com/risingwavelabs/await-tree/), are also available. `tracing-active-tree` takes inspiration from their implementation. However, both `async-backtrace` and `await-tree` require declaring spans using methods and structs provided by the library, which may be inconvenient for existing codebases. In contrast, `tracing-active-tree` does not require any changes to the existing code. It only requires registering an extra [Layer](https://docs.rs/tracing-subscriber/latest/tracing_subscriber/layer/trait.Layer.html) to the tracing subscribers [registry](https://docs.rs/tracing-subscriber/latest/tracing_subscriber/registry/index.html).

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
        layer::global().fmt_string()
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

There are overheads when tracing. When tracing huge task trees, the overhead may obvious. Run benches in the `benches` folder to check it.

You may also enable the feature `coarsetime` for a better performance of fetching system time.

There are some examples of the overheading, generally, huge (both wide or deep) trees will bring performance penalty:

```console
> cargo bench --features="coarsetime" --bench deep_tree
deep_baseline           time:   [194.64 µs 208.66 µs 223.05 µs]
deep_with_subs          time:   [4.7118 ms 4.8690 ms 5.0579 ms]
```

Tracing wide(a root future waits for many futures) trees has poorer performance than tracing deep(a tree like a linked list) tree:

```console
> cargo bench --features="coarsetime" --bench wide_tree
wide_baseline           time:   [895.63 µs 924.51 µs 953.05 µs]
wide_with_subs          time:   [16.192 ms 17.383 ms 18.629 ms]
```

> **NOTE**
> When you tweaking the parameters of those benchmarks, you may notice that the performance regression isn't linear with the amount of tree nodes. This imples that it is possible to optimize this. They should be linear.

But there isn't observable overheading when spawning many tiny tasks: 

```console
> cargo bench --features="coarsetime" --bench many_tasks
many_tasks_baseline     time:   [4.3762 ms 4.4394 ms 4.5032 ms]
many_tasks_with_subs    time:   [4.1975 ms 4.2936 ms 4.3886 ms]
```

## Contributing

If you're interested in contributing to `tracing-active-tree`, or want to build it from source, see [CONTRIBUTING.md](./CONTRIBUTING.md).

## License

`tracing-active-tree` is licensed under the Apache 2.0 license. See the [LICENSE](./LICENSE) file for more details.
