use criterion::{black_box, criterion_group, criterion_main, Criterion};
use atlas_rb_tree::tree::Tree;

fn insert_benchmark(c: &mut Criterion) {
    c.bench_function("Tree insert", |b| {
        b.iter(|| {
            let mut tree = Tree::new();
            tree.insert(black_box(1));
            tree.insert(black_box(2));
        });
    });
}

fn delete_benchmark(c: &mut Criterion) {
    c.bench_function("Tree delete", |b| {
        b.iter(|| {
            let mut tree = Tree::new();
            tree.insert(2);
            tree.insert(1);
            tree.delete(black_box(1))
        });
    });
}

criterion_group!(benches, insert_benchmark, delete_benchmark);
criterion_main!(benches);
