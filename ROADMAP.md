# Roadmap for Stellar Wave

## Before applying

- Compile and test the contract with the supported Stellar toolchain.
- Add unit tests for authorization, duplicate recipients, invalid basis points, minimum amounts, rounding, and token transfer rollback.
- Deploy to Stellar Testnet, publish the contract ID and deployment ledger, and document a transaction that creates a team and a transaction that donates.
- Review storage lifetime and restoration, then document the behavior for dormant teams.

## Scoped contributor issues after the baseline works

1. **Contract events guide:** document event topics, payloads, and an example decoder. Acceptance: another developer can recover team and donation details from one event without reading contract source.
2. **Rounding tests:** add table-driven tests covering 2–10 recipients and tiny valid donations. Acceptance: all payouts are positive and their sum always equals the donor amount.
3. **Storage restoration guide:** document how to restore archived team records. Acceptance: a Testnet walkthrough demonstrates restoring a dormant team and reading it again.

Each issue should be opened separately on GitHub with its acceptance criteria and reviewed during an active Wave.
