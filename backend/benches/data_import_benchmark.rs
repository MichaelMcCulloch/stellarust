use backend::data_import::DataImport;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::path::PathBuf;

fn data_import_benchmark(c: &mut Criterion) {
    let path = PathBuf::from("..").join("res/test_data/unzipper/zipped.sav");
    c.bench_function("data_import", |b| {
        b.iter(|| DataImport::from_file(black_box(&path)))
    });
}

criterion_group!(benches, data_import_benchmark);
criterion_main!(benches);
