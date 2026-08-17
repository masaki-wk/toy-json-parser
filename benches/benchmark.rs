use criterion::{Criterion, criterion_group, criterion_main};
use paste::paste;
use std::fs;
use std::hint::black_box;
use toy_json_parser::{Lexer, Parser};

fn workload(input: &str) {
    let lexer = Lexer::new(input.chars());
    let mut parser = Parser::new(lexer);
    let _value = parser.parse().unwrap();
}

fn do_benchmark(c: &mut Criterion, id: &str, path: &str) {
    let input = fs::read_to_string(path).unwrap();
    c.bench_function(id, |b| b.iter(|| workload(black_box(&input))));
}

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

generate_benchmarks! {
    citm_catalog,
}

criterion_group!(benches, benchmark_citm_catalog);
criterion_main!(benches);
