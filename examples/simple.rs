// Copyright 2023 TiKV Project Authors. Licensed under Apache-2.0.

use tracing::instrument;
use tracing_active_tree::layer;
use tracing_subscriber::prelude::*;

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
    println!("{}", debug_dump_current_tree());
}

fn debug_dump_current_tree() -> String {
    layer::global().fmt_string()
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tracing_subscriber::registry()
        .with(layer::global().clone())
        .init();
    foo().await;
}
