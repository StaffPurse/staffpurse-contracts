# Protocol Mechanics

## State Machine
The contract maintains a mapping of batch dates to Merkle roots. Once a root is anchored for a date, it cannot be overwritten.

## Anchoring Batching Cadence
Roots are anchored on a scheduled batching cadence.

## Privacy Constraints
Only hashes are anchored on-chain. No raw data is ever exposed to the public network, ensuring strict privacy.
