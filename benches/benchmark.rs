use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use stack_experiments::stack_safe::FixedStack as SafeFixedStack;
use stack_experiments::stack_unsafe::FixedStack as UnsafeFixedStack;

fn configure_criterion() -> Criterion {
    Criterion::default()
        .sample_size(100)
        .measurement_time(std::time::Duration::new(15, 0))
}

const SIZES: [usize; 4] = [1024, 512 * 1024, 1024 * 1024, 2 * 1024 * 1024];

// Benchmark for pushing elements to the safe stack
fn bench_safe_push(c: &mut Criterion) {
    let mut group = c.benchmark_group("Safe FixedStack/Push");
    for elements in SIZES.iter() {
        group.throughput(Throughput::Elements(*elements as u64));
        group.bench_with_input(format!("{elements} elements"), elements, |b, &elements| {
            b.iter_with_setup(
                || SafeFixedStack::new(elements),
                |mut stack| {
                    for i in 0..elements {
                        stack.push(black_box(i));
                    }
                },
            )
        });
    }

    group.finish();
}

// Benchmark for popping elements off the safe stack
fn bench_safe_pop(c: &mut Criterion) {
    let mut group = c.benchmark_group("Safe FixedStack/Pop");
    for elements in SIZES.iter() {
        group.throughput(Throughput::Elements(*elements as u64));
        group.bench_with_input(format!("{elements} elements"), elements, |b, elements| {
            b.iter_with_setup(
                || {
                    let mut stack = SafeFixedStack::new(*elements);
                    for i in 0..*elements {
                        stack.push(i);
                    }
                    stack
                },
                |mut stack| {
                    while let Some(x) = stack.pop() {
                        // Do nothing
                        let _ = black_box(x);
                    }
                },
            )
        });
    }

    group.finish();
}

// Benchmark for pushing elements to the unsafe stack
fn bench_unsafe_push(c: &mut Criterion) {
    let mut group = c.benchmark_group("Unsafe FixedStack/Push");
    for elements in SIZES.iter() {
        group.throughput(Throughput::Elements(*elements as u64));
        group.bench_with_input(format!("{elements} elements"), elements, |b, &elements| {
            b.iter_with_setup(
                || UnsafeFixedStack::new(elements),
                |mut stack| {
                    for i in 0..elements {
                        stack.push(black_box(i));
                    }
                },
            )
        });
    }

    group.finish();
}

// Benchmark for popping elements off the unsafe stack
fn bench_unsafe_pop(c: &mut Criterion) {
    let mut group = c.benchmark_group("Unsafe FixedStack/Pop");
    for elements in SIZES.iter() {
        group.throughput(Throughput::Elements(*elements as u64));
        group.bench_with_input(format!("{elements} elements"), elements, |b, elements| {
            b.iter_with_setup(
                || {
                    let mut stack = UnsafeFixedStack::new(*elements);
                    for i in 0..*elements {
                        stack.push(i);
                    }
                    stack
                },
                |mut stack| {
                    while let Some(x) = stack.pop() {
                        // Do nothing
                        let _ = black_box(x);
                    }
                },
            )
        });
    }

    group.finish();
}

criterion_group! {
    name = benches;
    config = configure_criterion();
    targets = bench_safe_push, bench_unsafe_push, bench_safe_pop, bench_unsafe_pop
}

criterion_main!(benches);
