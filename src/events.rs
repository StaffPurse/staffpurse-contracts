use soroban_sdk::{Env, Symbol, BytesN};

pub fn publish_anchored_event(env: &Env, batch_date: Symbol, root: BytesN<32>) {
    let topics = (Symbol::new(env, "anchored"), batch_date);
    env.events().publish(topics, root);
}
