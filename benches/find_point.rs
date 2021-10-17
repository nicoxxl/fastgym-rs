use criterion::{criterion_group, criterion_main, Criterion};
use fastgym::{
    find_point::{Action, FindPoint, ObsKind, Reward},
    Env,
};
use rand::{rngs::mock::StepRng, Rng};

pub fn gen_1d<R>(
    pos: [usize; 1],
    target: [usize; 1],
    random_generator: R,
) -> impl Env<[ObsKind; 10], Action = [Action; 1], Reward = Reward>
where
    R: Rng,
{
    FindPoint::new_full(pos, target, [10], random_generator)
}

pub fn benchmark_1d(c: &mut Criterion) {
    c.bench_function("act", |b| {
        b.iter_with_setup(
            || gen_1d([0], [10], StepRng::new(2, 1)),
            |mut env| env.step(&[Action::Plus]),
        )
    });
    c.bench_function("observe 10 ObsKind", |b| {
        let mut obs = [ObsKind::Nothing; 10];
        b.iter_with_setup(
            || FindPoint::new_full([0], [10], [10], StepRng::new(2, 1)),
            |env| env.observe(&mut obs),
        )
    });
    c.bench_function("observe 100 ObsKind", |b| {
        let mut obs = [ObsKind::Nothing; 100];
        b.iter_with_setup(
            || FindPoint::new_full([0], [10], [100], StepRng::new(2, 1)),
            |env| env.observe(&mut obs),
        )
    });
    c.bench_function("observe 1000 ObsKind", |b| {
        let mut obs = [ObsKind::Nothing; 1000];
        b.iter_with_setup(
            || FindPoint::new_full([0], [10], [1000], StepRng::new(2, 1)),
            |env| env.observe(&mut obs),
        )
    });

    c.bench_function("observe 10 i8", |b| {
        let mut obs = [0i8; 10];
        b.iter_with_setup(
            || FindPoint::new_full([0], [10], [10], StepRng::new(2, 1)),
            |env| env.observe(&mut obs),
        )
    });
    c.bench_function("observe 100 i8", |b| {
        let mut obs = [0i8; 100];
        b.iter_with_setup(
            || FindPoint::new_full([0], [10], [100], StepRng::new(2, 1)),
            |env| env.observe(&mut obs),
        )
    });
    c.bench_function("observe 1000 i8", |b| {
        let mut obs = [0i8; 1000];
        b.iter_with_setup(
            || FindPoint::new_full([0], [10], [1000], StepRng::new(2, 1)),
            |env| env.observe(&mut obs),
        )
    });

    c.bench_function("observe 10 f32", |b| {
        let mut obs = [0f32; 10];
        b.iter_with_setup(
            || FindPoint::new_full([0], [10], [10], StepRng::new(2, 1)),
            |env| env.observe(&mut obs),
        )
    });
    c.bench_function("observe 100 f32", |b| {
        let mut obs = [0f32; 100];
        b.iter_with_setup(
            || FindPoint::new_full([0], [10], [100], StepRng::new(2, 1)),
            |env| env.observe(&mut obs),
        )
    });
    c.bench_function("observe 1000 f32", |b| {
        let mut obs = [0f32; 1000];
        b.iter_with_setup(
            || FindPoint::new_full([0], [10], [1000], StepRng::new(2, 1)),
            |env| env.observe(&mut obs),
        )
    });
}

criterion_group!(benches, benchmark_1d);
criterion_main!(benches);
