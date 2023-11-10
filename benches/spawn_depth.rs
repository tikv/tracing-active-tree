use criterion::{criterion_group, criterion_main, Criterion};
use futures_util::future::BoxFuture;

use tracing_active_tree::{frame, layer, root};
use tracing_subscriber::prelude::*;

pub fn count_down(rand_field: u64, total: u64) -> BoxFuture<'static, ()> {
    if total == 0 {
        Box::pin(futures::future::ready(())) as _
    } else {
        Box::pin(frame!(count_down(rand_field, total - 1))) as _
    }
}

pub fn spawn_depth_tasks() {
    tokio::runtime::Handle::current()
        .block_on(tokio::spawn(root!(count_down(42, 1000))))
        .unwrap()
}

pub fn origin_count_down(rand_field: u64, total: u64) -> BoxFuture<'static, ()> {
    if total == 0 {
        Box::pin(futures::future::ready(())) as _
    } else {
        // NOTE: This pin is useless and can be removed.
        // But we want to test the overhead of tracing, so make the indirect level the same.
        // In practice, the extra indirect of tracing only exists at the async recursive case,
        // which is (relatively) rare.
        Box::pin(count_down(rand_field, total - 1))
    }
}

pub fn spawn_depth_origin_tasks() {
    tokio::runtime::Handle::current()
        .block_on(tokio::spawn(origin_count_down(42, 1000)))
        .unwrap()
}

pub fn many_with_baseline(c: &mut Criterion) {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    let _g = rt.enter();
    c.bench_function("deep_baseline", |b| b.iter(|| spawn_depth_origin_tasks()));
    tracing_subscriber::registry()
        .with(layer::global().clone())
        .init();
    c.bench_function("deep_with_subs", |b| b.iter(|| spawn_depth_tasks()));
}

criterion_group!(many_wide, many_with_baseline);
criterion_main!(many_wide);
