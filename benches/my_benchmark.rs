use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;
use mlp::feed_forward;

pub fn criterion_benchmark(c: &mut Criterion) {
    const NUM_INPUTS: usize = 64;
    const NUM_HIDDEN_0: usize = 1;
    const NUM_OUTPUTS: usize = 128;

    let mut vertices = vec![
        vec![0.0; NUM_INPUTS].into_boxed_slice(),
        vec![0.0; NUM_INPUTS + NUM_HIDDEN_0].into_boxed_slice(),
        vec![0.0; NUM_INPUTS + NUM_HIDDEN_0 + NUM_OUTPUTS].into_boxed_slice(),
    ]
    .into_boxed_slice();

    let edges = vec![
        vec![0.0; NUM_INPUTS * NUM_HIDDEN_0].into_boxed_slice(),
        vec![0.0; (NUM_INPUTS + NUM_HIDDEN_0) * NUM_OUTPUTS].into_boxed_slice(),
    ]
    .into_boxed_slice();

    c.bench_function("feed_forward #1", |b| {
        b.iter(|| feed_forward(&mut vertices, &edges))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
