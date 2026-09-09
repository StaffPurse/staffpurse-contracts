#!/usr/bin/env bash
# Deploys the StaffPurse anchoring contract to a Stellar network (default: testnet)
# and outputs the resulting contract ID to deployments/<network>.env
set -euo pipefail

NETWORK="${NETWORK:-testnet}"
IDENTITY="${IDENTITY:-deployer}"
OUT_DIR="deployments"

echo "============================================================"
echo "StaffPurse Soroban Contract Deployment"
echo "Target Network: $NETWORK"
echo "Identity:       $IDENTITY"
echo "============================================================"

# Ensure deployments directory exists
mkdir -p "$OUT_DIR"

# 1. Verify or generate identity
if ! stellar keys show "$IDENTITY" >/dev/null 2>&1; then
  echo "Generating new Stellar identity '$IDENTITY'..."
  stellar keys generate "$IDENTITY" --network "$NETWORK"
fi

DEPLOYER_ADDRESS=$(stellar keys address "$IDENTITY")
echo "Deployer Address: $DEPLOYER_ADDRESS"

# 2. Fund identity on testnet via Friendbot if needed
if [ "$NETWORK" = "testnet" ]; then
  echo "Checking funding via Friendbot..."
  curl -s "https://friendbot.stellar.org?addr=${DEPLOYER_ADDRESS}" >/dev/null 2>&1 || true
fi

# 3. Build the contract
echo "Building Soroban contract WASM..."
cargo build --target wasm32-unknown-unknown --release

WASM_FILE="target/wasm32-unknown-unknown/release/staffpurse_anchor.wasm"
if [ ! -f "$WASM_FILE" ]; then
  # Fallback check if package name is different
  WASM_FILE=$(find target/wasm32-unknown-unknown/release -maxdepth 1 -name "*.wasm" ! -name "*.optimized.wasm" | head -n 1)
fi

if [ -z "$WASM_FILE" ] || [ ! -f "$WASM_FILE" ]; then
  echo "Error: WASM file not found. Ensure the cargo crate compiles a cdylib target."
  exit 1
fi

echo "Optimizing WASM: $WASM_FILE"
stellar contract optimize --wasm "$WASM_FILE"
OPTIMIZED_WASM="${WASM_FILE%.wasm}.optimized.wasm"

# 4. Deploy contract
echo "Deploying to Stellar $NETWORK..."
CONTRACT_ID=$(stellar contract deploy \
  --wasm "$OPTIMIZED_WASM" \
  --source "$IDENTITY" \
  --network "$NETWORK")

echo "Contract deployed with ID: $CONTRACT_ID"

# 5. Initialize contract with admin
echo "Initializing contract admin..."
stellar contract invoke \
  --id "$CONTRACT_ID" \
  --source "$IDENTITY" \
  --network "$NETWORK" \
  -- initialize \
  --admin "$DEPLOYER_ADDRESS" || echo "Note: If initialize function is already called or not implemented, skipping."

# 6. Record deployment
ENV_FILE="${OUT_DIR}/${NETWORK}.env"
cat <<EOF > "$ENV_FILE"
# StaffPurse Soroban Anchoring Contract Deployment
# Generated: $(date -u +"%Y-%m-%dT%H:%M:%SZ")
NETWORK=${NETWORK}
CONTRACT_ID=${CONTRACT_ID}
ADMIN_ADDRESS=${DEPLOYER_ADDRESS}
WASM_PATH=${OPTIMIZED_WASM}
EOF

echo "Deployment recorded in $ENV_FILE"
echo "Deployment successful!"
