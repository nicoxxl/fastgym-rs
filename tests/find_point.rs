use fastgym::{
    find_point::{Action, FindPoint, ObsKind, Reward},
    Env,
};
use rand::{rngs::mock::StepRng, Rng};

pub fn gen_1d<R>(
    pos: [usize; 1],
    target: [usize; 1],
    size: [usize; 1],
    random_generator: R,
) -> impl Env<[ObsKind; 10], Action = [Action; 1], Reward = Reward>
where
    R: Rng,
{
    FindPoint::new_full(pos, target, size, random_generator)
}

#[test]
fn test_win() {
    let mut env = gen_1d([2], [5], [10], StepRng::new(2, 1));
    let mut obs = [ObsKind::Nothing; 10];
    assert!(env.check());
    env.observe(&mut obs);
    assert_eq!(env.step(&[Action::Plus]), (false, Reward::Moved));
    assert_eq!(env.step(&[Action::Plus]), (false, Reward::Moved));
    assert_eq!(env.step(&[Action::Plus]), (true, Reward::Win));
}
