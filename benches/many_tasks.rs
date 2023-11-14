// Copyright 2023 TiKV Project Authors. Licensed under Apache-2.0.

use criterion::{criterion_group, criterion_main, Criterion};

use tracing::instrument;
use tracing_active_tree::layer;
use tracing_subscriber::prelude::*;

#[instrument]
async fn dummy(n: usize) {
    for _ in 0..10 {
        tokio::task::yield_now().await;
    }
}

pub fn spawn_many_tasks(n: usize) {
    let futs = (0..n).map(|i| tokio::spawn(dummy(i)));
    tokio::runtime::Handle::current()
        .block_on(futures::future::try_join_all(futs))
        .unwrap();
}

async fn origin_dummy(_n: usize) {
    for _ in 0..10 {
        tokio::task::yield_now().await;
    }
}

pub fn origin_spawn_many_tasks(n: usize) {
    let futs = (0..n).map(|i| tokio::spawn(origin_dummy(i)));
    tokio::runtime::Handle::current()
        .block_on(futures::future::try_join_all(futs))
        .unwrap();
}

pub fn run(c: &mut Criterion) {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    let _g = rt.enter();
    c.bench_function("many_tasks_baseline", |b| {
        b.iter(|| origin_spawn_many_tasks(1000))
    });
    tracing_subscriber::registry()
        .with(layer::global().clone())
        .init();
    c.bench_function("many_tasks_with_subs", |b| {
        b.iter(|| spawn_many_tasks(1000))
    });
}

criterion_group!(many_tasks, run);
criterion_main!(many_tasks);
