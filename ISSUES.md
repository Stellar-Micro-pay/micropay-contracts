# GitHub Issues — micropay-contracts

Create these issues on GitHub after pushing. Labels: `feat`, `fix`, `docs`, `test`, `chore`.

---

## Open Issues

1. **feat(contracts): implement multi-token support (USDC, XLM)**
   Allow charging in tokens other than XLM stroops.

2. **feat(contracts): add spending limits per user**
   Treasury should be able to cap how much a funded account can spend per epoch.

3. **feat(contracts): add batch charge function**
   Allow admin to deduct from multiple users in a single transaction.

4. **feat(contracts): emit usage metadata in charge event**
   Include endpoint identifier in `charge` event for richer Treasury monitoring.

5. **fix(contracts): add reentrancy guard on deposit functions**
   Audit cross-contract call paths for potential reentrancy.

6. **fix(contracts): validate admin address on initialize**
   Reject zero/invalid addresses at initialization time.

7. **test(contracts): add fuzz tests for charge overflow edge cases**
   i128 arithmetic should be validated against max values.

8. **test(contracts): add test for update_treasury authorization**
   Ensure non-admin cannot update treasury address.

9. **test(contracts): simulate multiple concurrent users**
   Validate balance isolation between different wallets.

10. **docs(contracts): add NatSpec-style comments to all public functions**
    Inline documentation for each contract function.

11. **chore(contracts): set up GitHub Actions CI for Soroban build + test**
    Auto-build and test WASM on every PR.

12. **chore(contracts): pin Soroban SDK version in Cargo.lock**
    Ensure reproducible builds.

13. **feat(contracts): add pause/unpause functionality**
    Admin should be able to halt the contract in an emergency.

14. **feat(contracts): implement refund function**
    Allow users to withdraw unused balance.

15. **docs(contracts): document stroop denomination and conversion table**
    Add inline reference for XLM → stroop conversions.
