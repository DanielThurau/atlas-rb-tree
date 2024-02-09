use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rb_tree::tree::Tree;

fn insert_benchmark(c: &mut Criterion) {
    c.bench_function("Tree insert", |b| {

        b.iter(||{
            let mut tree = Tree::new(1);
            tree.insert(black_box(1));});

    });
}

fn delete_benchmark(c: &mut Criterion) {
    c.bench_function("Tree delete", |b| {

        b.iter(|| {
            let mut tree = Tree::new(0);
            tree.insert(1);
            tree.delete(black_box(1))
        });
    });
}

criterion_group!(benches, insert_benchmark, delete_benchmark);
criterion_main!(benches);