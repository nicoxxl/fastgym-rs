pub mod find_point;

/// Main trait implement by each environements
pub trait Env<Obs> {
    type Action;
    type Reward;

    /// Return false is there is something wrong with the environement.
    /// (Loading error, inconsistencies with parameters, etc)
    fn check(&self) -> bool;

    /// Reset function to restart the environement
    fn reset(&mut self);

    /// Step function to take an action, return a tuple of (done, reward).
    /// Done will be true when the episode is finished and require to call
    /// `.reset()` next
    /// The reward is simply the reward signal.
    fn step(&mut self, action: &Self::Action) -> (bool, Self::Reward);

    /// This method is used get the observation, this is the main difference
    /// with the OpenAI gym, the goal is to be able to reuse objects and avoid
    /// allocation..
    fn observe(&self, obs: &mut Obs);
}
