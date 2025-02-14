use criterion::{criterion_group, criterion_main, Criterion};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

fn bench(c: &mut Criterion) {
    /*
    c.bench_function("d1p1", |b| b.iter(|| day1::p1()));
    c.bench_function("d1p2", |b| b.iter(|| day1::p2()));
    c.bench_function("d2p1", |b| b.iter(|| day2::p1()));
    c.bench_function("d2p2", |b| b.iter(|| day2::p2()));
    c.bench_function("d3p1", |b| b.iter(|| day3::p1()));
    c.bench_function("d3p2", |b| b.iter(|| day3::p2()));
    c.bench_function("d4p1", |b| b.iter(|| day4::p1()));
    c.bench_function("d4p2", |b| b.iter(|| day4::p2()));
    c.bench_function("d5p1", |b| b.iter(|| day5::p1()));
    c.bench_function("d5p2", |b| b.iter(|| day5::p2()));*/
    c.bench_function("d6p1", |b| b.iter(|| day6::p1()));
    //c.bench_function("d6p2", |b| b.iter(|| day6::p2()));
}

criterion_group!(benches, bench);
criterion_main!(benches);