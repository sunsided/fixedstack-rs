use criterion::{black_box, criterion_group, criterion_main, Criterion};
use stack_experiments::stack_safe::FixedStack as SafeFixedStack;
use stack_experiments::stack_unsafe::FixedStack as UnsafeFixedStack;

// Benchmark for pushing elements to the safe stack
fn bench_safe_push(c: &mut Criterion) {
    let mut group = c.benchmark_group("Safe FixedStack");
    group.bench_function("Push", |b| {
        b.iter(|| {
            let mut stack = SafeFixedStack::new(1024);
            for i in 0..1024 {
                stack.push(black_box(i));
            }
        })
    });
    group.bench_function("Pop", |b| {
        b.iter(|| {
            let mut stack = SafeFixedStack::new(1024);
            for i in 0..1024 {
                stack.push(i);
            }
            while let Some(_) = stack.pop() {
                // Do nothing
            }
        })
    });
    group.finish();
}

// Benchmark for pushing elements to the unsafe stack
fn bench_unsafe_push(c: &mut Criterion) {
    let mut group = c.benchmark_group("Unsafe FixedStack");
    group.bench_function("Push", |b| {
        b.iter(|| {
            let mut stack = UnsafeFixedStack::new(1024);
            for i in 0..1024 {
                stack.push(black_box(i));
            }
        })
    });
    group.bench_function("Pop", |b| {
        b.iter(|| {
            let mut stack = UnsafeFixedStack::new(1024);
            for i in 0..1024 {
                stack.push(i);
            }
            while let Some(_) = stack.pop() {
                // Do nothing
            }
        })
    });
    group.finish();
}

criterion_group!(benches, bench_safe_push, bench_unsafe_push);
criterion_main!(benches);
