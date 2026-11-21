# Donation Splitter contracts

A Soroban contract that creates immutable teams and divides each donation across 2–10 recipients in one transaction. Shares are basis points and must total 10,000. The last recipient receives any rounding remainder. The contract takes no fee and retains no successful donation balance.

## Contract API

- `create_team(owner, name, token, shares) -> team_id`: owner authorizes; recipient addresses must be unique.
- `donate(team_id, donor, amount)`: donor authorizes; the token moves into the contract and immediately out to all recipients. A failed transfer reverts the transaction.
- `get_team(team_id)` and `team_count()`: public reads.
- `TeamCreated` and `Donation` events: public indexing and receipts.

Shares cannot be edited. Create a new team if membership changes, which keeps earlier donation terms clear. Team records use persistent storage with a 180-day TTL refreshed by donations; a dormant team may need restoration after expiry. Deploy on Stellar Testnet and set the contract ID in the backend and frontend `.env` files. This is an initial implementation and requires contract review before Mainnet use.

License: Apache-2.0.
