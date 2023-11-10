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

pub fn spawn_many_tasks() {
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

pub fn spawn_many_origin_tasks() {
    tokio::runtime::Handle::current()
        .block_on(tokio::spawn(origin_wide(42, 1000)))
        .unwrap()
}

pub fn many_with_baseline(c: &mut Criterion) {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    let _g = rt.enter();
    c.bench_function("many_baseline", |b| b.iter(|| spawn_many_origin_tasks()));
    tracing_subscriber::registry()
        .with(layer::global().clone())
        .init();
    c.bench_function("many_with_subs", |b| b.iter(|| spawn_many_tasks()));
}

criterion_group!(many_wide, many_with_baseline);
criterion_main!(many_wide);
