use soroban_sdk::{contracttype, BytesN, Env, Symbol};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AnchorRootEvent {
    pub batch_date: Symbol,
    pub root: BytesN<32>,
    pub ledger: u32,
}

pub fn emit_anchor_root(env: &Env, batch_date: Symbol, root: BytesN<32>) {
    let ledger = env.ledger().sequence();
    env.events().publish(
        (Symbol::new(env, "anchor_root"), batch_date.clone()),
        AnchorRootEvent {
            batch_date,
            root,
            ledger,
        },
    );
}
