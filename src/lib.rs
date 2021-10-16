/// Main trait implement by each environements
pub trait Env<Obs, Action> {
    /// Reset function to restart the environement
    fn reset(&mut self);

    /// Step function to take an action, return a tuple of (done, reward).
    /// Done will be true when the episode is finished and require to call
    /// `.reset()` next
    /// The reward is simply the reward signal.
    fn step(&mut self, action: &Action) -> (bool, f32);

    /// This method is used get the observation, this is the main difference
    /// with the OpenAI gym, the goal is to be able to reuse objects and avoid
    /// allocation..
    fn observe(&self, obs: &mut Obs);
}
