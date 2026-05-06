use criterion::{criterion_group, criterion_main, Criterion};

fn call_overhead(c: &mut Criterion) {
    c.bench_function("call_overhead_baseline", |b| {
        b.iter(|| {
            let mut v = 0_u64;
            for i in 0..1000 {
                v = v.wrapping_add(i * 2 + 1);
            }
            v
        })
    });
}

fn serialization(c: &mut Criterion) {
    let data: Vec<f32> = (0..2000).map(|n| n as f32).collect();
    c.bench_function("serde_roundtrip_vec_f32", |b| {
        b.iter(|| {
            let encoded = serde_json::to_vec(&data).expect("serialize vec");
            let _: Vec<f32> = serde_json::from_slice(&encoded).expect("deserialize vec");
        })
    });
}

criterion_group!(benches, call_overhead, serialization);
criterion_main!(benches);

