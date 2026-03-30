#![cfg(test)]
use super::*;
use soroban_sdk::{testutils::{Address as _}, Address, BytesN, Env};

#[test]
fn test_end_to_end_flow() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(StellarIsko, ());
    let client = StellarIskoClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let student = Address::generate(&env);
    let treasury = Address::generate(&env);
    
    let token_admin = Address::generate(&env);
    let token_id = env.register_stellar_asset_contract_v2(token_admin.clone()).address();
    let token_client = token::StellarAssetClient::new(&env, &token_id);
    let token_logic = token::Client::new(&env, &token_id);
    
    client.initialize(&admin, &treasury, &token_id);

    token_client.mint(&student, &500);

    // --- STEP 1: Student pays ---
    client.request_and_pay(&student, &150);
    assert_eq!(token_logic.balance(&student), 350);
    assert_eq!(token_logic.balance(&treasury), 150);

    // --- STEP 2: Admin issues ---
    let doc_hash = BytesN::from_array(&env, &[7u8; 32]);
    client.issue_soulbound_credential(&admin, &student, &doc_hash);

    // --- STEP 3: Verification ---
    assert_eq!(client.verify_credential(&student, &doc_hash), true);
}

#[test]
#[should_panic(expected = "Error(Auth, InvalidAction)")] 
fn test_unauthorized_minting() {
    let env = Env::default();
    // No mock_all_auths here to trigger failure
    let contract_id = env.register(StellarIsko, ());
    let client = StellarIskoClient::new(&env, &contract_id);

    let fake_admin = Address::generate(&env);
    let student = Address::generate(&env);
    let doc_hash = BytesN::from_array(&env, &[9u8; 32]);

    client.issue_soulbound_credential(&fake_admin, &student, &doc_hash);
}

#[test]
fn test_state_verification() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(StellarIsko, ());
    let client = StellarIskoClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let student = Address::generate(&env);
    let doc_hash = BytesN::from_array(&env, &[1u8; 32]);
    
    // Test that verification is FALSE before issuance
    assert_eq!(client.verify_credential(&student, &doc_hash), false);

    client.issue_soulbound_credential(&admin, &student, &doc_hash);

    // Test that verification is TRUE after issuance (State check)
    assert_eq!(client.verify_credential(&student, &doc_hash), true);
}
