// Copyright 2023 TiKV Project Authors. Licensed under Apache-2.0.

use std::time::Duration;

use futures_util::future::BoxFuture;
use tracing::instrument;
use tracing_active_tree::{frame, layer, tree::formating::FormatFlat};
use tracing_subscriber::prelude::*;

/*
This case is pretty like the example showed at the crate `tokio/async-backtrace`.

Simple Output:
6
└[examples/complex_call.rs:41] [span_id=6] ["long_task"] [elapsed=316.137µs]
 └[examples/complex_call.rs:43] [span_id=7] ["tokio::time::sleep(Duration::from_secs(40))"] [elapsed=291.971µs]
1
└[examples/complex_call.rs:23] [span_id=1] ["foo"] [elapsed=655.53µs]
 └[examples/complex_call.rs:28] [span_id=2] ["bar"] [elapsed=575.366µs]
  ├[examples/complex_call.rs:32] [span_id=3] ["futures::future::pending()"] [elapsed=539.468µs]
  └[examples/complex_call.rs:36] [span_id=4] ["baz"] [elapsed=513.632µs]
   └[examples/complex_call.rs:46] [span_id=5] ["quux"] [n=3] [elapsed=488.806µs]
    ├[examples/complex_call.rs:58] [span_id=8] ["real_work"] [_id=0] [elapsed=228.006µs]
    ├[examples/complex_call.rs:58] [span_id=9] ["real_work"] [_id=1] [elapsed=198.883µs]
    └[examples/complex_call.rs:58] [span_id=10] ["real_work"] [_id=2] [elapsed=181.24µs]
*/

#[instrument]
async fn foo() {
    bar().await
}

#[instrument]
async fn bar() {
    tokio::select! {
        _ = baz() => {}
        _ = frame!(futures::future::pending()) => {}
    }
}

#[instrument]
async fn baz() {
    quux(3).await
}

#[instrument]
async fn long_task() {
    frame!(tokio::time::sleep(Duration::from_secs(40))).await
}

#[instrument]
async fn quux(n: usize) {
    let tasks = (0..n).map(|n| Box::pin(real_work(n)) as BoxFuture<_>);
    tokio::spawn(long_task());
    // Give the newly spawned task a chance to be scheduled.
    tokio::task::yield_now().await;
    let print = async {
        println!(
            "{}",
            String::from_utf8(
                layer::global()
                    .fmt_bytes_with(|tree, buf| tree.traverse_with(FormatFlat::new(buf)).unwrap())
            )
            .unwrap()
        );
    };
    futures::future::join_all(tasks.chain(std::iter::once(Box::pin(print) as BoxFuture<_>))).await;
}

#[instrument]
async fn real_work(_id: usize) {
    tokio::task::yield_now().await;
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tracing_subscriber::registry()
        .with(layer::global().clone())
        .init();
    foo().await;
}
