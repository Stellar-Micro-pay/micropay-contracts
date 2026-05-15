# micropay-contracts

> Soroban smart contracts for the MicroPay API payments platform.

This project is funded and governed by the Stellar Treasury system:
**https://github.com/Stellar-Treasury/Treasury-frontend**

---

## Overview

`micropay-contracts` is the on-chain layer of the MicroPay ecosystem.
It receives funds from the DAO Treasury, tracks per-user balances, and deducts
micro-amounts each time a protected API endpoint is consumed.

---

## Contract Functions

| Function | Caller | Description |
|---|---|---|
| `initialize(admin, treasury)` | Deployer | One-time setup |
| `deposit_from_treasury(user, amount)` | Treasury contract | Fund a user from DAO treasury |
| `deposit_from_user(user, amount)` | User wallet | User self-funds |
| `charge(user, amount)` | Admin (backend) | Deduct per API call |
| `get_balance(user)` | Anyone | Spendable balance |
| `get_treasury_balance(user)` | Anyone | Treasury-funded portion |

---

## Events Emitted

| Event | Payload |
|---|---|
| `deposit / treasury` | `(user, amount)` |
| `deposit / user` | `(user, amount)` |
| `charge / api` | `(user, amount)` |

---

## Security

- Only the **admin** address (backend wallet) may call `charge`.
- Only the registered **treasury contract** may call `deposit_from_treasury`.
- Overdraft is prevented with an explicit balance check.
- Both deposit flows are tracked separately for auditability.

---

## Build & Test

```bash
# Install Stellar CLI
cargo install --locked stellar-cli --features opt

# Build WASM
stellar contract build

# Run tests
cargo test --features testutils

# Deploy to testnet
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/micropay_contracts.wasm \
  --source YOUR_SECRET_KEY \
  --network testnet
```

---

## Environment Variables

| Variable | Description |
|---|---|
| `TREASURY_CONTRACT_ADDRESS` | Address of the DAO Treasury Soroban contract |
| `ADMIN_SECRET_KEY` | Backend wallet used to sign `charge` calls |

---

## Repository Structure

```
micropay-contracts/
├── src/
│   ├── lib.rs      ← Contract logic
│   └── test.rs     ← Unit tests
├── Cargo.toml
└── README.md
```

---

## Related Repositories

| Repo | Purpose |
|---|---|
| [micropay-backend](https://github.com/Stellar-Micro-pay/micropay-backend) | Node.js API gateway |
| [micropay-frontend](https://github.com/Stellar-Micro-pay/micropay-frontend) | Developer dashboard |
| [micropay-docs](https://github.com/Stellar-Micro-pay/micropay-docs) | Full documentation |
| [stellar-treasury](https://github.com/Stellar-Treasury/Treasury-frontend) | Governing DAO |
