use super::*;
use soroban_sdk::{
    testutils::Address as _,
    token::{StellarAssetClient, TokenClient},
    vec,
};

#[test]
fn donation_splits_exactly_and_leaves_no_balance() {
    let env = Env::default();
    env.mock_all_auths();
    let issuer = Address::generate(&env);
    let token_id = env.register_stellar_asset_contract_v2(issuer).address();
    let token = TokenClient::new(&env, &token_id);
    let asset = StellarAssetClient::new(&env, &token_id);
    let contract_id = env.register(DonationSplitter, ());
    let splitter = DonationSplitterClient::new(&env, &contract_id);
    let owner = Address::generate(&env);
    let donor = Address::generate(&env);
    let first = Address::generate(&env);
    let second = Address::generate(&env);
    let shares = vec![
        &env,
        Share {
            recipient: first.clone(),
            basis_points: 7000,
        },
        Share {
            recipient: second.clone(),
            basis_points: 3000,
        },
    ];
    let id = splitter.create_team(
        &owner,
        &String::from_str(&env, "Garden"),
        &token_id,
        &shares,
    );
    asset.mint(&donor, &101);
    splitter.donate(&id, &donor, &101);
    assert_eq!(token.balance(&first), 70);
    assert_eq!(token.balance(&second), 31);
    assert_eq!(token.balance(&donor), 0);
    assert_eq!(token.balance(&contract_id), 0);
    assert_eq!(splitter.team_count(), 1);
}

#[test]
fn rejects_invalid_shares() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(DonationSplitter, ());
    let splitter = DonationSplitterClient::new(&env, &contract_id);
    let owner = Address::generate(&env);
    let token = Address::generate(&env);
    let first = Address::generate(&env);
    let duplicate = vec![
        &env,
        Share {
            recipient: first.clone(),
            basis_points: 5000,
        },
        Share {
            recipient: first,
            basis_points: 5000,
        },
    ];
    assert_eq!(
        splitter.try_create_team(
            &owner,
            &String::from_str(&env, "Garden"),
            &token,
            &duplicate
        ),
        Err(Ok(Error::DuplicateRecipient))
    );
}
