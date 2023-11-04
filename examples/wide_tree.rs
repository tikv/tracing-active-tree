// Copyright 2023 TiKV Project Authors. Licensed under Apache-2.0.

use std::{cell::RefCell, collections::HashMap, time::Duration};

use futures::channel::oneshot;
use tracing::instrument;
use tracing_active_tree::{frame, layer};
use tracing_subscriber::prelude::*;

/*

This example shows how a wide active tree (probably some futures that waiting for many
other futures concurrently) looks like.

1
└[examples/wide_tree.rs:13] [span_id=1] ["join_many"] [max=5] [elapsed=974.567µs]
 ├[examples/wide_tree.rs:18] [span_id=3] ["nest_root"] [i=1] [elapsed=862.47µs]
 │└[examples/wide_tree.rs:8] [span_id=2251799813685250] ["dummy"] [elapsed=606.968µs]
 ├[examples/wide_tree.rs:18] [span_id=4] ["nest_root"] [i=2] [elapsed=867.766µs]
 │├[examples/wide_tree.rs:8] [span_id=7] ["dummy"] [elapsed=576.435µs]
 │└[examples/wide_tree.rs:8] [span_id=8] ["dummy"] [elapsed=539.281µs]
 ├[examples/wide_tree.rs:18] [span_id=5] ["nest_root"] [i=3] [elapsed=861.4µs]
 │├[examples/wide_tree.rs:8] [span_id=9] ["dummy"] [elapsed=516.472µs]
 │├[examples/wide_tree.rs:8] [span_id=10] ["dummy"] [elapsed=497.408µs]
 │└[examples/wide_tree.rs:8] [span_id=11] ["dummy"] [elapsed=475.498µs]
 └[examples/wide_tree.rs:18] [span_id=6] ["nest_root"] [i=4] [elapsed=873.938µs]
  ├[examples/wide_tree.rs:8] [span_id=12] ["dummy"] [elapsed=453.066µs]
  ├[examples/wide_tree.rs:8] [span_id=13] ["dummy"] [elapsed=421.089µs]
  ├[examples/wide_tree.rs:8] [span_id=14] ["dummy"] [elapsed=318.863µs]
  └[examples/wide_tree.rs:8] [span_id=15] ["dummy"] [elapsed=295.081µs]
 */

#[instrument(skip_all)]
async fn dummy(rx: oneshot::Receiver<()>) {
    let _ = rx.await;
}

#[instrument]
async fn join_many(max: usize) {
    let v = RefCell::new(vec![]);
    let futs = (0..max)
        .map(|i| {
            frame!("nest_root"; futures::future::join_all( (0..i).map(|_| {
                    let (tx, rx) = oneshot::channel();
                    v.borrow_mut().push(tx);
                    dummy(rx)
                })); i)
        })
        .collect::<Vec<_>>();
    let (tx, rx) = oneshot::channel();
    tokio::spawn(async move {
        tokio::task::yield_now().await;
        println!("{}", layer::global().fmt_string());
        tx.send(()).unwrap();
        drop(v);
    });
    futures::future::join_all(futs).await;
    rx.await.unwrap()
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tracing_subscriber::registry()
        .with(layer::global().clone())
        .init();
    join_many(5).await;
}
