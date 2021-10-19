use std::collections::VecDeque;

use rand::Rng;

use crate::Env;

pub enum Action {
    North,
    South,
    West,
    East,
}

pub enum Reward {
    Nothing,
    Win(usize),
    Lose,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Observation {
    Nothing,
    Snake,
    Target,
}
impl From<Observation> for i8 {
    fn from(obs: Observation) -> Self {
        match obs {
            Observation::Nothing => 0,
            Observation::Snake => 1,
            Observation::Target => -1,
        }
    }
}
impl From<Observation> for f32 {
    fn from(obs: Observation) -> Self {
        match obs {
            Observation::Nothing => 0.,
            Observation::Snake => 1.,
            Observation::Target => -1.,
        }
    }
}

pub struct Snake<R> {
    board_size: [usize; 2],
    target: [usize; 2],
    snake: VecDeque<[usize; 2]>,
    random_generator: R,
}

impl<R> Snake<R>
where
    R: Rng,
{
    pub fn new(board_size: [usize; 2], random_generator: R) -> Self {
        Self {
            board_size,
            target: [0, 0],
            snake: VecDeque::new(),
            random_generator,
        }
    }

    pub fn reset(&mut self) {
        self.target = [
            self.random_generator.gen_range(0..self.board_size[0]),
            self.random_generator.gen_range(0..self.board_size[1]),
        ];
        self.snake.clear();
        self.snake.push_back([
            self.random_generator.gen_range(0..self.board_size[0]),
            self.random_generator.gen_range(0..self.board_size[1]),
        ]);
    }

    pub fn step(&mut self, action: &Action) -> (bool, Reward) {
        let head = self.snake.back().unwrap();
        let new_head = match action {
            Action::North => {
                if head[0] == 0 {
                    [self.board_size[0] - 1, head[1]]
                } else {
                    [head[0] - 1, head[1]]
                }
            }
            Action::South => {
                if head[0] == self.board_size[0] - 1 {
                    [0, head[1]]
                } else {
                    [head[0] + 1, head[1]]
                }
            }
            Action::West => {
                if head[1] == 0 {
                    [head[1], self.board_size[1] - 1]
                } else {
                    [head[0], head[1] - 1]
                }
            }
            Action::East => {
                if head[1] == self.board_size[1] - 1 {
                    [head[0], 0]
                } else {
                    [head[0], head[1] + 1]
                }
            }
        };

        for part in self.snake.iter() {
            if &new_head == part {
                return (true, Reward::Lose);
            }
        }

        self.snake.push_back(new_head);
        if new_head == self.target {
            return (false, Reward::Win(self.snake.len()));
        } else {
            self.snake.pop_front();
            return (false, Reward::Nothing);
        }
    }
}

pub struct AbsSnake<R> {
    snake: Snake<R>,
}
impl<R> AbsSnake<R>
where
    R: Rng,
{
    pub fn new(board_size: [usize; 2], random_generator: R) -> Self {
        Self {
            snake: Snake::new(board_size, random_generator),
        }
    }
}

impl<Obs, R, const X: usize, const Y: usize> Env<[[Obs; Y]; X]> for AbsSnake<R>
where
    Obs: From<Observation> + Copy,
    R: Rng,
{
    type Action = Action;
    type Reward = Reward;

    fn check(&self) -> bool {
        self.snake.board_size == [X, Y]
    }

    fn reset(&mut self) {
        self.snake.reset()
    }

    fn step(&mut self, action: &Self::Action) -> (bool, Self::Reward) {
        self.snake.step(action)
    }

    fn observe(&self, obs: &mut [[Obs; Y]; X]) {
        *obs = [[Observation::Nothing.into(); Y]; X];
        for foo in self.snake.snake.iter() {
            obs[foo[0]][foo[1]] = Observation::Snake.into();
        }
        obs[self.snake.target[0]][self.snake.target[1]] = Observation::Target.into();
    }
}

pub struct PovSnake<R> {
    snake: Snake<R>,
}
impl<R> PovSnake<R>
where
    R: Rng,
{
    pub fn new(board_size: [usize; 2], random_generator: R) -> Self {
        Self {
            snake: Snake::new(board_size, random_generator),
        }
    }
}

impl<Obs, R, const X: usize, const Y: usize> Env<[[Obs; Y]; X]> for PovSnake<R>
where
    Obs: From<Observation> + Copy,
    R: Rng,
{
    type Action = Action;
    type Reward = Reward;

    fn check(&self) -> bool {
        self.snake.board_size == [X, Y]
    }

    fn reset(&mut self) {
        self.snake.reset()
    }

    fn step(&mut self, action: &Self::Action) -> (bool, Self::Reward) {
        self.snake.step(action)
    }

    fn observe(&self, obs: &mut [[Obs; Y]; X]) {
        *obs = [[Observation::Nothing.into(); Y]; X];
        let head = self.snake.snake.back().unwrap();
        let x_range = (head[0] - X / 2)..=(head[0] + (X / 2) + (X & 1));
        let y_range = (head[1] - Y / 2)..=(head[1] + (Y / 2) + (Y & 1));

        for foo in self.snake.snake.iter() {
            if x_range.contains(&foo[0]) && y_range.contains(&foo[1]) {
                let x = foo[0] - x_range.start();
                let y = foo[1] - y_range.start();
                obs[x][y] = Observation::Snake.into();
            }
        }
        if x_range.contains(&self.snake.target[0]) && y_range.contains(&self.snake.target[1]) {
            let x = self.snake.target[0] - x_range.start();
            let y = self.snake.target[1] - y_range.start();
            obs[x][y] = Observation::Target.into();
        }
    }
}
