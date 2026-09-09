# API Reference

## Public Functions

### `get_root`
* **Parameters:** `batch_date: Symbol`
* **Return Type:** `Option<BytesN<32>>`
* **Description:** Retrieves the anchored Merkle root for a given batch date.
* **Auth Requirements:** None (public read).

### `anchor_root`
* **Parameters:** `batch_date: Symbol`, `root: BytesN<32>`
* **Return Type:** `Result<(), ContractError>`
* **Description:** Anchors a new Merkle root for the given batch date.
* **Auth Requirements:** Must be authorized by the administrator.
