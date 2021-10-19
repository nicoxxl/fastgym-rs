use crate::Env;
use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObsKind {
    Nothing,
    Player,
    Target,
}
impl From<ObsKind> for i8 {
    fn from(ok: ObsKind) -> Self {
        match ok {
            ObsKind::Nothing => 0,
            ObsKind::Player => 1,
            ObsKind::Target => -1,
        }
    }
}
impl From<ObsKind> for f32 {
    fn from(ok: ObsKind) -> Self {
        match ok {
            ObsKind::Nothing => 0.,
            ObsKind::Player => 1.,
            ObsKind::Target => -1.,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Plus,
    Zero,
    Minus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Reward {
    Still,
    Moved,
    Win,
}

pub struct FindPoint<R, const C: usize> {
    pos: [usize; C],
    target: [usize; C],
    size: [usize; C],
    random_generator: R,
}

impl<R, const C: usize> FindPoint<R, C>
where
    R: Rng,
{
    pub fn new(size: [usize; C], random_generator: R) -> Self {
        Self {
            pos: [0; C],
            target: [0; C],
            size,
            random_generator,
        }
    }

    pub fn new_full(
        pos: [usize; C],
        target: [usize; C],
        size: [usize; C],
        random_generator: R,
    ) -> Self {
        Self {
            pos,
            target,
            size,
            random_generator,
        }
    }

    fn reset_inner(&mut self) {
        for i in 0..C {
            self.pos[i] = self.random_generator.gen_range(0..self.size[i]);
            self.target[i] = self.random_generator.gen_range(0..self.size[i]);
        }
    }

    fn step_inner(&mut self, action: &[Action; C]) -> (bool, Reward) {
        let old_pos = self.pos;
        for i in 0..C {
            match action[i] {
                Action::Plus => self.pos[i] = self.pos[i].saturating_add(1).min(self.size[i]),
                Action::Zero => {}
                Action::Minus => self.pos[i] = self.pos[i].saturating_sub(1),
            }
        }
        if self.pos == self.target {
            (true, Reward::Win)
        } else if self.pos == old_pos {
            (false, Reward::Still)
        } else {
            (false, Reward::Moved)
        }
    }
}

impl<OK, R, const X: usize> Env<[OK; X]> for FindPoint<R, 1>
where
    OK: From<ObsKind>,
    R: Rng,
{
    type Action = [Action; 1];
    type Reward = Reward;

    fn check(&self) -> bool {
        self.size[0] == X
    }

    fn reset(&mut self) {
        assert_eq!(self.size[0], X);
        self.reset_inner();
    }

    fn step(&mut self, action: &Self::Action) -> (bool, Self::Reward) {
        assert_eq!(self.size[0], X);
        self.step_inner(action)
    }

    fn observe(&self, obs: &mut [OK; X]) {
        assert_eq!(self.size[0], X);
        for x in 0..X {
            if self.pos == [x] {
                obs[x] = ObsKind::Player.into();
            } else if self.target == [x] {
                obs[x] = ObsKind::Target.into();
            } else {
                obs[x] = ObsKind::Nothing.into();
            }
        }
    }
}

impl<OK, R, const X: usize, const Y: usize> Env<[[OK; Y]; X]> for FindPoint<R, 2>
where
    OK: From<ObsKind> + Copy,
    R: Rng,
{
    type Action = [Action; 2];
    type Reward = Reward;

    fn check(&self) -> bool {
        self.size[0] == X && self.size[1] == Y
    }

    fn reset(&mut self) {
        assert_eq!(self.size[0], X);
        assert_eq!(self.size[1], Y);
        self.reset_inner()
    }

    fn step(&mut self, action: &Self::Action) -> (bool, Self::Reward) {
        assert_eq!(self.size[0], X);
        assert_eq!(self.size[1], Y);
        self.step_inner(action)
    }

    fn observe(&self, obs: &mut [[OK; Y]; X]) {
        assert_eq!(self.size[0], X);
        assert_eq!(self.size[1], Y);
        *obs = [[ObsKind::Nothing.into(); Y]; X];

        for x in 0..X {
            for y in 0..Y {
                if self.pos == [x, y] {
                    obs[x][y] = ObsKind::Player.into();
                } else if self.target == [x, y] {
                    obs[x][y] = ObsKind::Target.into();
                } else {
                    obs[x][y] = ObsKind::Nothing.into();
                }
            }
        }
    }
}
