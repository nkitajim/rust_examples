use bit_count::*;

use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};

fn bench_i32_bit_count(c: &mut Criterion) {
	c.bench_function(
		"i32_bit_count",
		|b| b.iter(|| black_box(i32_count_bit(255)))
	);
}

fn bench_inline_i32_bit_count(c: &mut Criterion) {
	c.bench_function(
		"inline_i32_bit_count",
		|b| b.iter(|| black_box(inline_i32_count_bit(255)))
	);
}

fn bench_count_ones(c: &mut Criterion) {
	c.bench_function(
		"count_ones",
		|b| b.iter(|| black_box(255_i32.count_ones()))
	);
}

criterion_group!(
	benches,
	bench_i32_bit_count,
	bench_inline_i32_bit_count,
	bench_count_ones
);
criterion_main!(benches);
