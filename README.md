# micropay-contracts

Soroban smart contracts for pay-per-use API billing with treasury-aware funding sources.

This project is funded and governed by the Stellar Treasury system:
https://github.com/YOUR-USERNAME/stellar-treasury

## Contract Responsibilities

- Accept treasury deposits through `deposit_from_treasury(address, amount)`
- Accept self-funded deposits through `deposit_from_user(address, amount)`
- Charge per API request through `charge(address, amount)`
- Return aggregate balances through `get_balance(address)`
- Emit auditable `deposit` and `charge` events

## Security Model

- `charge` is restricted to the configured backend wallet
- `deposit_from_treasury` requires treasury wallet authorization
- Overdrafts are rejected (`insufficient_balance`)

## Local Build

```bash
cargo build --target wasm32-unknown-unknown --release
```

## Deployment Notes

Set these addresses at initialization:

- backend signer wallet
- treasury signer wallet (shared with DAO treasury operations)
