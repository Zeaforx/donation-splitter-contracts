#![no_std]

use soroban_sdk::{
    contract, contracterror, contractevent, contractimpl, contracttype, token, Address, Env,
    String, Vec,
};

const MAX_MEMBERS: u32 = 10;
const MAX_NAME: u32 = 64;
const TTL: u32 = 180 * 17_280;
const TTL_THRESHOLD: u32 = 90 * 17_280;

#[contracttype]
#[derive(Clone)]
pub struct Share {
    pub recipient: Address,
    pub basis_points: u32,
}

#[contracttype]
#[derive(Clone)]
pub struct Team {
    pub id: u64,
    pub owner: Address,
    pub name: String,
    pub token: Address,
    pub shares: Vec<Share>,
}

#[contracttype]
#[derive(Clone)]
enum Key {
    NextId,
    Team(u64),
}

#[contracterror]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Error {
    InvalidName = 1,
    InvalidShares = 2,
    DuplicateRecipient = 3,
    TeamNotFound = 4,
    InvalidAmount = 5,
    AmountTooSmall = 6,
    Overflow = 7,
}

#[contractevent]
pub struct TeamCreated {
    #[topic]
    pub team_id: u64,
    #[topic]
    pub owner: Address,
    pub name: String,
    pub token: Address,
}

#[contractevent]
pub struct Donation {
    #[topic]
    pub team_id: u64,
    #[topic]
    pub donor: Address,
    pub token: Address,
    pub amount: i128,
}

#[contract]
pub struct DonationSplitter;

#[cfg(test)]
mod test;

#[contractimpl]
impl DonationSplitter {
    pub fn create_team(
        env: Env,
        owner: Address,
        name: String,
        token: Address,
        shares: Vec<Share>,
    ) -> Result<u64, Error> {
        owner.require_auth();
        if name.len() == 0 || name.len() > MAX_NAME {
            return Err(Error::InvalidName);
        }
        if shares.len() < 2 || shares.len() > MAX_MEMBERS {
            return Err(Error::InvalidShares);
        }
        let mut total: u32 = 0;
        for i in 0..shares.len() {
            let share = shares.get(i).unwrap();
            if share.basis_points == 0 {
                return Err(Error::InvalidShares);
            }
            for j in 0..i {
                if shares.get(j).unwrap().recipient == share.recipient {
                    return Err(Error::DuplicateRecipient);
                }
            }
            total = total
                .checked_add(share.basis_points)
                .ok_or(Error::InvalidShares)?;
        }
        if total != 10_000 {
            return Err(Error::InvalidShares);
        }
        let next: u64 = env.storage().instance().get(&Key::NextId).unwrap_or(1);
        let new_next = next.checked_add(1).ok_or(Error::Overflow)?;
        let team = Team {
            id: next,
            owner: owner.clone(),
            name: name.clone(),
            token: token.clone(),
            shares,
        };
        env.storage().instance().set(&Key::NextId, &new_next);
        env.storage().instance().extend_ttl(TTL_THRESHOLD, TTL);
        env.storage().persistent().set(&Key::Team(next), &team);
        env.storage()
            .persistent()
            .extend_ttl(&Key::Team(next), TTL_THRESHOLD, TTL);
        TeamCreated {
            team_id: next,
            owner,
            name,
            token,
        }
        .publish(&env);
        Ok(next)
    }

    pub fn donate(env: Env, team_id: u64, donor: Address, amount: i128) -> Result<(), Error> {
        donor.require_auth();
        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }
        let key = Key::Team(team_id);
        let team: Team = env
            .storage()
            .persistent()
            .get(&key)
            .ok_or(Error::TeamNotFound)?;
        // Every recipient must receive at least one smallest token unit.
        if amount < team.shares.len() as i128 {
            return Err(Error::AmountTooSmall);
        }
        let token = token::Client::new(&env, &team.token);
        let contract = env.current_contract_address();
        token.transfer(&donor, &contract, &amount);
        let mut sent = 0_i128;
        let last = team.shares.len() - 1;
        for i in 0..team.shares.len() {
            let share = team.shares.get(i).unwrap();
            let part = if i == last {
                amount.checked_sub(sent).ok_or(Error::Overflow)?
            } else {
                amount
                    .checked_mul(share.basis_points as i128)
                    .ok_or(Error::Overflow)?
                    / 10_000
            };
            if part <= 0 {
                return Err(Error::AmountTooSmall);
            }
            token.transfer(&contract, &share.recipient, &part);
            sent = sent.checked_add(part).ok_or(Error::Overflow)?;
        }
        env.storage()
            .persistent()
            .extend_ttl(&key, TTL_THRESHOLD, TTL);
        Donation {
            team_id,
            donor,
            token: team.token,
            amount,
        }
        .publish(&env);
        Ok(())
    }

    pub fn get_team(env: Env, team_id: u64) -> Result<Team, Error> {
        env.storage()
            .persistent()
            .get(&Key::Team(team_id))
            .ok_or(Error::TeamNotFound)
    }

    pub fn team_count(env: Env) -> u64 {
        let next: u64 = env.storage().instance().get(&Key::NextId).unwrap_or(1);
        next - 1
    }
}
