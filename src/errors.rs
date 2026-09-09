use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ContractError {
    AlreadyAnchored = 1,
    Unauthorized = 2,
    AlreadyInitialized = 3,
    NotInitialized = 4,
}
