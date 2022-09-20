use criterion::{black_box, criterion_group, criterion_main, Criterion};
use jsoncrawler_lib::crawl;
use jsoncrawler_lib::tokio::runtime::Builder;

/// benchmark crawl speed
pub fn bench_speed(c: &mut Criterion) {
    let mut group = c.benchmark_group("perf/crawl");
    let sample_count = 100;
    let sample_title = format!("jsoncrawler {} samples", sample_count);

    group.sample_size(sample_count);
    
    group.bench_function(sample_title, |b| {
        let runtime = Builder::new_multi_thread().enable_all().build().unwrap();
        b.to_async(runtime).iter(|| black_box(crawl(vec![])))
    });
    group.finish();
}

criterion_group!(benches, bench_speed);
criterion_main!(benches);
