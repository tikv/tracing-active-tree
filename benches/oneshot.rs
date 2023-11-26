// Copyright 2023 TiKV Project Authors. Licensed under Apache-2.0.

use criterion::{criterion_group, criterion_main, Criterion};
use tracing::instrument;
use tracing_active_tree::{frame, layer};
use tracing_subscriber::prelude::*;

#[instrument]
pub async fn task(rand_field: u64) {
    frame!(tokio::task::yield_now()).await;
}

pub fn with_subs() {
    tokio::runtime::Handle::current()
        .block_on(tokio::spawn(task(42)))
        .unwrap()
}

pub async fn origin_task(_rand_field: u64) {
    tokio::task::yield_now().await;
}

pub fn baseline() {
    tokio::runtime::Handle::current()
        .block_on(tokio::spawn(task(42)))
        .unwrap()
}

pub fn run(c: &mut Criterion) {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    let _g = rt.enter();
    c.bench_function("oneshot_baseline", |b| b.iter(baseline));
    tracing_subscriber::registry()
        .with(layer::global().clone())
        .init();
    c.bench_function("oneshot_with_subs", |b| b.iter(with_subs));
}

criterion_group!(oneshot, run);
criterion_main!(oneshot);
