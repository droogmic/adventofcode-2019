use criterion::{criterion_group, criterion_main, Criterion};

use advent2019::get_string;
use advent2019::day3::get_wires;
use advent2019::day3::intersection_points;
use advent2019::day3::part1;
use advent2019::day3::part2;
use advent2019::day4::main4;

fn custom_criterion() -> Criterion {
    Criterion::default()
        .sample_size(20)
}

pub fn day3_benchmark(c: &mut Criterion) {
    c.bench_function("day3::wires", |b| b.iter(|| get_wires(get_string(3))));
    
    let wires = get_wires(get_string(3));
    c.bench_function("day3::intersection_points", |b| b.iter(|| {
            intersection_points(&wires.0, &wires.1)
    }));
    
    let wires = get_wires(get_string(3));
    c.bench_function("day3::part1", |b| b.iter(|| part1(&wires)));
    c.bench_function("day3::part2", |b| b.iter(|| part2(&wires)));
}

pub fn day4_benchmark(c: &mut Criterion) {
    c.bench_function("day4", |b| b.iter(|| main4()));
}

criterion_group!{
    name = benches;
    config = custom_criterion();
    targets = day3_benchmark, day4_benchmark
}
// criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);