use criterion::{Criterion, criterion_group, criterion_main};
use paste::paste;
use std::fs;
use std::hint::black_box;
use toy_json_parser::{Lexer, Parser};

// The workload function: parses a JSON input string and ignores the result (for benchmarking).
fn workload(input: &str) {
    let lexer = Lexer::new(input.chars());
    let mut parser = Parser::new(lexer);
    let _value = parser.parse().unwrap();
}

// Benchmarks the workload function with given id and path.
fn do_benchmark(c: &mut Criterion, id: &str, path: &str) {
    let input = fs::read_to_string(path).unwrap();
    c.bench_function(id, |b| b.iter(|| workload(black_box(&input))));
}

// Macro to define benchmark functions.
macro_rules! generate_benchmarks {
    ($($basefilename:ident),* $(,)?) => {
        $(
            paste! {
                fn [<benchmark_ $basefilename>](c: &mut Criterion) {
                    let id = stringify!($basefilename);
                    let path = concat!("benches/json-benchmark/", stringify!($basefilename), ".json");
                    do_benchmark(c, id, path)
                }
            }
        )+
    }
}

// Generate benchmark functions.
generate_benchmarks! {
    canada,
    citm_catalog,
    twitter,
}

// Register benchmark functions to Criterion.
criterion_group!(benches, benchmark_canada, benchmark_citm_catalog, benchmark_twitter);
criterion_main!(benches);
