/*
Cart Pole Environment using random actions
without rendering of the environment
*/

use gym_rs::{CartPoleEnv, GymEnv, ActionType, Viewer};
use rand::{thread_rng, Rng};

fn main() {
    let mut env = CartPoleEnv::default();
    env.seed(0);

    let mut _state: Vec<f64> = env.reset();

    // This produces a different action (random number) with every run,
    // if you would like the agent to behave the same, use Pcg64 with seeding,
    // see env.seed for how it is done in the environment.
    let mut rng = thread_rng();
    let mut end: bool = false;
    let mut total_reward: f64 = 0.0;
    while !end {
        // randomly choose a discrete action to take
        let action = ActionType::Discrete(rng.gen_range(0, 2));
        let (_state, reward, done, _info) = env.step(action);
        end = done;
        total_reward += reward;
    }
    println!("total_reward: {}", total_reward);
}