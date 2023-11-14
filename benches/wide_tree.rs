// Copyright 2023 TiKV Project Authors. Licensed under Apache-2.0.

use criterion::{criterion_group, criterion_main, Criterion};
use tracing::instrument;
use tracing_active_tree::{frame, layer};
use tracing_subscriber::prelude::*;

#[instrument]
pub async fn wide(rand_field: u64, total: u64) {
    futures::future::join_all((0..total).map(|x| task_l2(x))).await;
}

#[instrument]
pub async fn task_l2(counter: u64) {
    frame!(tokio::task::yield_now()).await;
}

pub fn with_subs() {
    tokio::runtime::Handle::current()
        .block_on(tokio::spawn(wide(42, 1000)))
        .unwrap()
}

pub async fn origin_wide(_rand_field: u64, total: u64) {
    futures::future::join_all((0..total).map(|x| task_l2(x))).await;
}

pub async fn origin_task_l2(_counter: u64) {
    tokio::task::yield_now().await;
}

pub fn baseline() {
    tokio::runtime::Handle::current()
        .block_on(tokio::spawn(origin_wide(42, 1000)))
        .unwrap()
}

pub fn run(c: &mut Criterion) {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    let _g = rt.enter();
    c.bench_function("wide_baseline", |b| b.iter(|| baseline()));
    tracing_subscriber::registry()
        .with(layer::global().clone())
        .init();
    c.bench_function("wide_with_subs", |b| b.iter(|| with_subs()));
}

criterion_group!(wide_tree, run);
criterion_main!(wide_tree);
