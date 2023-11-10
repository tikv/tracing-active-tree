use criterion::{criterion_group, criterion_main, Criterion};
use tracing::instrument;
use tracing_active_tree::{frame, layer};
use tracing_subscriber::prelude::*;

#[instrument]
pub async fn task(rand_field: u64) {
    frame!(tokio::task::yield_now()).await;
}

pub fn spawn_sole_task() {
    tokio::runtime::Handle::current()
        .block_on(tokio::spawn(task(42)))
        .unwrap()
}

pub async fn origin_task(_rand_field: u64) {
    tokio::task::yield_now().await;
}

pub fn spawn_sole_origin_task() {
    tokio::runtime::Handle::current()
        .block_on(tokio::spawn(task(42)))
        .unwrap()
}

pub fn oneshot_with_baseline(c: &mut Criterion) {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    let _g = rt.enter();
    c.bench_function("baseline", |b| b.iter(|| spawn_sole_origin_task()));
    tracing_subscriber::registry()
        .with(layer::global().clone())
        .init();
    c.bench_function("with_subs", |b| b.iter(|| spawn_sole_task()));
}

criterion_group!(oneshot, oneshot_with_baseline);
criterion_main!(oneshot);
