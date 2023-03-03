//! Collection of all runtime features.
//!
//! Steps to add a new feature are outlined below. Note that these steps only cover
//! the process of getting a feature into the core Safecoin code.
//! - For features that are unambiguously good (ie bug fixes), these steps are sufficient.
//! - For features that should go up for community vote (ie fee structure changes), more
//!   information on the additional steps to follow can be found at:
//!   <https://spl.solana.com/feature-proposal#feature-proposal-life-cycle>
//!
//! 1. Generate a new keypair with `safecoin-keygen new --outfile feature.json --no-passphrase`
//!    - Keypairs should be held by core contributors only. If you're a non-core contributor going
//!      through these steps, the PR process will facilitate a keypair holder being picked. That
//!      person will generate the keypair, provide pubkey for PR, and ultimately enable the feature.
//! 2. Add a public module for the feature, specifying keypair pubkey as the id with
//!    `safecoin_sdk::declare_id!()` within the module.
//!    Additionally, add an entry to `FEATURE_NAMES` map.
//! 3. Add desired logic to check for and switch on feature availability.
//!
//! For more information on how features are picked up, see comments for `Feature`.

use {
    lazy_static::lazy_static,
    safecoin_sdk::{
        clock::Slot,
        hash::{Hash, Hasher},
        pubkey::Pubkey,
    },
    std::collections::{HashMap, HashSet},
};

pub mod deprecate_rewards_sysvar {
    safecoin_sdk::declare_id!("GVriKcZATdgSzmhmu9PtimfheaKYksK5mhgoN5fbRhXg");
}

pub mod pico_inflation {
    safecoin_sdk::declare_id!("HC5H1tGb2RBRbrECSKmBVjwTsEEJziTTRsm3XSdTefrJ");
}

pub mod full_inflation {
    pub mod devnet_and_testnet {
        safecoin_sdk::declare_id!("JC1GQujpgHz92Am65FxcxByqqDGTdYJs6jLjgebnDpJF");
    }

    pub mod mainnet {
        pub mod certusone {
            pub mod vote {
                safecoin_sdk::declare_id!("7Ra6gKzEBFyYJ7fdsrCuzihZcxKu7AbmDy2FBEQFzap2");
            }
            pub mod enable {
                safecoin_sdk::declare_id!("54ZqXvMyCKShnBrGj2ni6m4kXTjRUrkc21dFoT9npJZk");
            }
        }
    }
}

pub mod secp256k1_program_enabled {
    safecoin_sdk::declare_id!("3tF9nXQrHzTV5jdDqzn6LVWMoJagN1zRgr4APfY6qmpi");
}

pub mod safe_token_v2_multisig_fix {
    safecoin_sdk::declare_id!("EwSWSBRpzYZSEqdZ215WMmnce6WiEsk57rSEB3e7ghh6");
}

pub mod no_overflow_rent_distribution {
    safecoin_sdk::declare_id!("9TyDRDhs933rTCWGwzSUTSx1XeJT14sc17o1cNQzUaBq");
}

pub mod filter_stake_delegation_accounts {
    safecoin_sdk::declare_id!("HpGqShCRhP7QwMBXTs1KbATiHWa383EUWjg3kbQjN2Kf");
}

pub mod require_custodian_for_locked_stake_authorize {
    safecoin_sdk::declare_id!("FKWSvfcXATHSBBNvr5VE6ns4tNsTG3EGzcDw2xVtowZQ");
}

pub mod safe_token_v2_self_transfer_fix {
    safecoin_sdk::declare_id!("2XDc17ZmSTbpqV3B5fmEGac4CKCYnbJj7vnfASvdzqyN");
}

pub mod warp_timestamp_again {
    safecoin_sdk::declare_id!("EgfbGcUzJotpXW8H4TnhVwDbrjm75xecKnrSAfqv4YvW");
}

pub mod check_init_vote_data {
    safecoin_sdk::declare_id!("CfH5RJxvjkQ1LaBSrnCDnYvHZtL6FuTQ751myi6DJEb5");
}

pub mod secp256k1_recover_syscall_enabled {
    safecoin_sdk::declare_id!("Bk9KRo8RKyBuE4W9uWD6jijdP7nLMbi2h26wc9aZnsab");
}

pub mod system_transfer_zero_check {
    safecoin_sdk::declare_id!("ACrERgP7eXsQquYUNQwASR884izTTXnR2jDnxjWRPEde");
}

pub mod blake3_syscall_enabled {
    safecoin_sdk::declare_id!("EJjfy9f9NY7qyBWv5vkSFTNGEdrGPKoLZEUG5ktcHuTk");
}

pub mod dedupe_config_program_signers {
    safecoin_sdk::declare_id!("Ffpg1Q73ocztQKMTbkTMeTAVpcGUb95u2aUaCeFQ47NY");
}

pub mod verify_tx_signatures_len {
    safecoin_sdk::declare_id!("H9EqD4wZCFzqo3P5jbNrAQ8nhNq3xfaA3NLh9Z7sLXwY");
}

pub mod vote_stake_checked_instructions {
    safecoin_sdk::declare_id!("BjfQ729TvPN84xsvsGMUsRLAqGc1ttSCQ6C5Zd3rUg2N");
}

pub mod rent_for_sysvars {
    safecoin_sdk::declare_id!("8bwDv5gkxZap8PyNAahSSGvfdcoFmpyustP6U7ysHiKP");
}

pub mod libsecp256k1_0_5_upgrade_enabled {
    safecoin_sdk::declare_id!("EVHL8iX15Gf6PpjdMd7pqPivjPQ2LLbK9fkGsrsGWy9r");
}

pub mod tx_wide_compute_cap {
    safecoin_sdk::declare_id!("J3WyHmj9HcTEzMAGPKZLBVir6b5VniMBnNZk8puvxDxL");
}

pub mod safe_token_v2_set_authority_fix {
    safecoin_sdk::declare_id!("Cb3jN13cfCNDV9dp36djNpcZXF7r82UAE4U1tZjXnFx5");
}

pub mod merge_nonce_error_into_system_error {
    safecoin_sdk::declare_id!("4n5Ko6ax8yLi21CXoBMFbCy52QydH7jpy42W5df7GZqT");
}

pub mod disable_fees_sysvar {
    safecoin_sdk::declare_id!("CcxsWdzHQ3wyNpLQ7E8konzi5cGz6UqXfeRTDTkGrCLt");
}

pub mod stake_merge_with_unmatched_credits_observed {
    safecoin_sdk::declare_id!("BrbirQE6i85NmJ3eB6jRLJbuYMT26HhSeHFYdxrfbUkk");
}

pub mod zk_token_sdk_enabled {
    safecoin_sdk::declare_id!("LtC5sq6KbLrLiyYceekkCYKbqHYjXt2RMjeWVYqh9za");
}

pub mod curve25519_syscall_enabled {
    safecoin_sdk::declare_id!("AbUw25egriAS7t3uHCtdJFjuzXRJdsihEMSLCmEfSWum");
}

pub mod versioned_tx_message_enabled {
    safecoin_sdk::declare_id!("AtsKod95wFiFsmFvwN1xsXWTurv4CgrCoKyZYQqRkVdW");
}

pub mod libsecp256k1_fail_on_bad_count {
    safecoin_sdk::declare_id!("ELGoo15JsBsoS649X4HDyqpisjsk9Z3xMQHA2tc5fsVh");
}

pub mod libsecp256k1_fail_on_bad_count2 {
    safecoin_sdk::declare_id!("57hm6ERNDfJLmgXj2jM9qsxUzNSDK3MtTrbTdDrN6wd8");
}

pub mod instructions_sysvar_owned_by_sysvar {
    safecoin_sdk::declare_id!("Cw3DVuS5cftJsULpNoexEES6voZx268FdZWdnYJ7AQr8");
}

pub mod stake_program_advance_activating_credits_observed {
    safecoin_sdk::declare_id!("8SgjbhRrT9Lu5gX8zyWtbeMynSMLfKs1VkDoVVbojSDp");
}

pub mod credits_auto_rewind {
    safecoin_sdk::declare_id!("BUS12ciZ5gCoFafUHWW8qaFMMtwFQGVxjsDheWLdqBE2");
}

pub mod demote_program_write_locks {
    safecoin_sdk::declare_id!("4mKMV9GnGmbgncfkSQJid9Kcnu1sYL5kJSWmHLGNNW2W");
}

pub mod ed25519_program_enabled {
    safecoin_sdk::declare_id!("BUiQnBRcQXphWNg9izLmu4Q7yvPCrETjbBAqPbJRbigo");
}

pub mod return_data_syscall_enabled {
    safecoin_sdk::declare_id!("JBReCcnWddZuLuQP2uHtirrqBTgbXq13nf27pFZn4yAG");
}

pub mod reduce_required_deploy_balance {
    safecoin_sdk::declare_id!("CChcJjMA6MbJnEa3JN5vSuuitmiZCrpiJCuYpwLNc6xJ");
}

pub mod sol_log_data_syscall_enabled {
    safecoin_sdk::declare_id!("FfA1fo35R7A1hAdTySiBYWP53vmtmNjoY93hk3BFij8q");
}

pub mod stakes_remove_delegation_if_inactive {
    safecoin_sdk::declare_id!("7cRMwESG7sdDrPBKWUWJZnUzPB1FRXj2YDCFRepjpcvX");
}

pub mod do_support_realloc {
    safecoin_sdk::declare_id!("7X2unCk3oLEhsrgkXKkRcfHcFiv2B361bMSoSXG7VogL");
}

pub mod prevent_calling_precompiles_as_programs {
    safecoin_sdk::declare_id!("1tqRX4kb9mxhLJFeUG7a9AZ3ZSt9ycCTyNmvYPcprdJ");
}

pub mod optimize_epoch_boundary_updates {
    safecoin_sdk::declare_id!("9aRcTW6CA2gm66oFyRqp9bV6qF5VYHiuSxdxTGm78rEi");
}

pub mod remove_native_loader {
    safecoin_sdk::declare_id!("ATWHHk7mdfaatNZTuKPUMvcBc9iKZStmph9ASzaWoSSj");
}

pub mod send_to_tpu_vote_port {
    safecoin_sdk::declare_id!("JAB9JKLm1zmdv9ntxnbTUxgGK9p2WdmDjzwf5ptMZQ4L");
}

pub mod requestable_heap_size {
    safecoin_sdk::declare_id!("2m2vf4z2t6DV7GuwUxiyUCuvUv4kf8n6ZsYWRXZPu7Cb");
}

pub mod disable_fee_calculator {
    safecoin_sdk::declare_id!("5fWcNUDYBNyX6ntr9c9JGqSMntZfwXMJxvMhD9TDh4Yo");
}

pub mod add_compute_budget_program {
    safecoin_sdk::declare_id!("HTTwDioiNtkTJf24JqUN8KNZYDevmVG3uH4sRQbT4Bch");
}

pub mod nonce_must_be_writable {
    safecoin_sdk::declare_id!("GwDyeFyJHbnVqXRPPty3vT6bk7YJyEvKiVSiJVzL3SiF");
}

pub mod safe_token_v3_3_0_release {
    safecoin_sdk::declare_id!("9steApkUSZKDra4d6GBqvjtLLovtqqquqxRj6HAySanV");
}

pub mod leave_nonce_on_success {
    safecoin_sdk::declare_id!("7ckuburuhbvpBc8tN9RoYR6uETFrC9Bf9rpffWvfH5Kj");
}

pub mod reject_empty_instruction_without_program {
    safecoin_sdk::declare_id!("CDSGXXUAYKsBjzByscc6QmiKghH68EBymxvreC2CEKHV");
}

pub mod fixed_memcpy_nonoverlapping_check {
    safecoin_sdk::declare_id!("AcdZ7SdrHcQvAABPKtENAqMChCZvRPVzqJD9TiKBsyWb");
}

pub mod reject_non_rent_exempt_vote_withdraws {
    safecoin_sdk::declare_id!("GxV3gunVo45B7U9PXFguaQcH5UhXuAvgoatzSDT5L6aM");
}

pub mod evict_invalid_stakes_cache_entries {
    safecoin_sdk::declare_id!("DuTNokfa22zREonYdWbZPoEwqjmKzPixetUyT4ffW3f2");
}

pub mod allow_votes_to_directly_update_vote_state {
    safecoin_sdk::declare_id!("Ff8b1fBeB86q8cjq47ZhsQLgv5EkHu3G1C99zjUfAzrq");
}

pub mod cap_accounts_data_len {
    safecoin_sdk::declare_id!("8ChajqNo7MSfq7G31GP6tw1bkEZEu7FAz5aG4mvhXKEX");
}

pub mod max_tx_account_locks {
    safecoin_sdk::declare_id!("ANaFEkPDA4bPGcXhBodHMX9yZ83kxUZhgdUqPhiev84N");
}

pub mod require_rent_exempt_accounts {
    safecoin_sdk::declare_id!("9RGjVttxDUgh6H82kTZiRNLawVgQmKaLhdhvWSXUhbv");
}

pub mod filter_votes_outside_slot_hashes {
    safecoin_sdk::declare_id!("FZZFGwVXMN6r94Ugcrvpd1tukiSFygdcM9FwcrsaTVY2");
}

pub mod update_syscall_base_costs {
    safecoin_sdk::declare_id!("4YytQkV3PmnLHYDVnPAajtxCVAT64CPYWwYMHCXGsMw6");
}

pub mod stake_deactivate_delinquent_instruction {
    safecoin_sdk::declare_id!("437r62HoAdUb63amq3D7ENnBLDhHT2xY8eFkLJYVKK4x");
}

pub mod stake_redelegate_instruction {
    safecoin_sdk::declare_id!("3EPmAX94PvVJCjMeFfRFvj4avqCPL8vv3TGsZQg7ydMx");
}

pub mod vote_withdraw_authority_may_change_authorized_voter {
    safecoin_sdk::declare_id!("2BYtq3z2Ywz2bP2UR8RwP6yepGyexGQY6eJyesptZgn2");
}

pub mod safe_associated_token_account_v1_0_4 {
    safecoin_sdk::declare_id!("4sFvovX9vpPjdonPZPs2rLj3g2urvtwMvmw1RqxrdNjk");
}

pub mod reject_vote_account_close_unless_zero_credit_epoch {
    safecoin_sdk::declare_id!("8EEoVj3mSZs6mJ89hVFTNW6ZKoFWYfAWNneSP4F53NfP");
}

pub mod add_get_processed_sibling_instruction_syscall {
    safecoin_sdk::declare_id!("CNGf1dMHeDnvrSAAc3CxrFeseEhdouFuFhxhcbaqAeuc");
}

pub mod bank_transaction_count_fix {
    safecoin_sdk::declare_id!("6dj2gJhSm1XHUbpRKVNjngEMfJia4YydaqUurF16P7qE");
}

pub mod disable_bpf_deprecated_load_instructions {
    safecoin_sdk::declare_id!("2fbhBGLzd9rM6MhVPGf7y3Z4GCsUSpep3sasbMi7LzsM");
}

pub mod disable_bpf_unresolved_symbols_at_runtime {
    safecoin_sdk::declare_id!("HSx7BtUEsXXeimVnF1EcziqahCgxL9oBf9tb3PsSjmab");
}

pub mod record_instruction_in_transaction_context_push {
    safecoin_sdk::declare_id!("Fc7KWisNu1RwAsRwrdvEQpcwvBw8EFgX8tXurT16L96J");
}

pub mod syscall_saturated_math {
    safecoin_sdk::declare_id!("CPuuztV5gCNKSdDTnfGKgDTo9cNZVPYsVbZUv1UhmR7h");
}

pub mod check_physical_overlapping {
    safecoin_sdk::declare_id!("EVhbxh9zEjkfHEx4kjkx9TvadV6F6WkaaxUKdTxwdiV3");
}

pub mod limit_secp256k1_recovery_id {
    safecoin_sdk::declare_id!("Acois5V3T9jv2hdFJXuVeKQzcHonJWACG5cewknQredH");
}

pub mod disable_deprecated_loader {
    safecoin_sdk::declare_id!("GTUMCZ8LTNxVfxdrw7ZsDFTxXb7TutYkzJnFwinpE6dg");
}

pub mod check_slice_translation_size {
    safecoin_sdk::declare_id!("GmC19j9qLn2RFk5NduX6QXaDhVpGncVVBzyM8e9WMz2F");
}

pub mod stake_split_uses_rent_sysvar {
    safecoin_sdk::declare_id!("FQnc7U4koHqWgRvFaBJjZnV8VPg6L6wWK33yJeDp4yvV");
}

pub mod add_get_minimum_delegation_instruction_to_stake_program {
    safecoin_sdk::declare_id!("St8k9dVXP97xT6faW24YmRSYConLbhsMJA4TJTBLmMT");
}

pub mod error_on_syscall_bpf_function_hash_collisions {
    safecoin_sdk::declare_id!("8199Q2gMD2kwgfopK5qqVWuDbegLgpuFUFHCcUJQDN8b");
}

pub mod reject_callx_r10 {
    safecoin_sdk::declare_id!("3NKRSwpySNwD3TvP5pHnRmkAQRsdkXWRr1WaQh8p4PWX");
}

pub mod drop_redundant_turbine_path {
    safecoin_sdk::declare_id!("FAbr5JVAvaF2DtqZzxPHoPBn6dEP4gKTW5tS1GNYfQYw");
}

pub mod executables_incur_cpi_data_cost {
    safecoin_sdk::declare_id!("CL54RJ5iQE5Jk7VJaNegv98TY7tvLqivensTEUsEnhsS");
}

pub mod fix_recent_blockhashes {
    safecoin_sdk::declare_id!("6iyggb5MTcsvdcugX7bEKbHV8c6jdLbpHwkncrgLMhfo");
}

pub mod update_rewards_from_cached_accounts {
    safecoin_sdk::declare_id!("28s7i3htzhahXQKqmS2ExzbEoUypg9krwvtK2M9UWXh9");
}

pub mod safe_token_v3_4_0 {
    safecoin_sdk::declare_id!("GqQz6K1Rj99MbzLL8iVee9sSZH6b8XRd6N5r4gAn3Sff");
}

pub mod safe_associated_token_account_v1_1_0 {
    safecoin_sdk::declare_id!("3dbt1wqFxD9uSLoZ1bRfkaETTbvVUWqDPGmgFXixitCE");
}

pub mod default_units_per_instruction {
    safecoin_sdk::declare_id!("HgX6iprND6esDrPTuiSSbNmDuAbKSAwFifjjSYKuwwkK");
}

pub mod stake_allow_zero_undelegated_amount {
    safecoin_sdk::declare_id!("sTKz343FM8mqtyGvYWvbLpTThw3ixRM4Xk8QvZ985mw");
}

pub mod require_static_program_ids_in_transaction {
    safecoin_sdk::declare_id!("8FdwgyHFEjhAdjWfV2vfqk7wA1g9X3fQpKH7SBpEv3kC");
}

pub mod stake_raise_minimum_delegation_to_1_sol {
    // This is a feature-proposal *feature id*.  The feature keypair address is `3YHAo6wWw5rDbQxb59BmJkQ3XwVhX3m8tdBVbtxnJmma`.
    safecoin_sdk::declare_id!("4xmyBuR2VCXzy9H6qYpH9ckfgnTuMDQFPFBfTs4eBCY1");
}

pub mod stake_minimum_delegation_for_rewards {
    safecoin_sdk::declare_id!("ELjxSXwNsyXGfAh8TqX8ih22xeT8huF6UngQirbLKYKH");
}

pub mod add_set_compute_unit_price_ix {
    safecoin_sdk::declare_id!("G4o7bAuRYtxChqR3bD2XhEqh9HE7TDJHdaxiQyegy3TY");
}

pub mod disable_deploy_of_alloc_free_syscall {
    safecoin_sdk::declare_id!("79HWsX9rpnnJBPcdNURVqygpMAfxdrAirzAGAVmf92im");
}

pub mod include_account_index_in_rent_error {
    safecoin_sdk::declare_id!("FrYss5jrBWh6S2NTAmurKd2LEn9Bm4h9idwqvBZ3tTng");
}

pub mod add_shred_type_to_shred_seed {
    safecoin_sdk::declare_id!("CeNiDsNf3t6fGegpcFb1WiyQiCwkfAEYrGVd8kKkLZoe");
}

pub mod warp_timestamp_with_a_vengeance {
    safecoin_sdk::declare_id!("FxQ1YgnmZfQw4ggDz9vThyTed8ZTukGKugndChYLHHuV");
}

pub mod separate_nonce_from_blockhash {
    safecoin_sdk::declare_id!("D7Z7kpxHoMog9cbx1WHiaRsUsneoVwWKCqS95BPLZu1P");
}

pub mod enable_durable_nonce {
    safecoin_sdk::declare_id!("2eWUxhwU8ZHRFrUz86CEvqUvWhQc5v7A1uDtV9vN93Ad");
}

pub mod vote_state_update_credit_per_dequeue {
    safecoin_sdk::declare_id!("CveezY6FDLVBToHDcvJRmtMouqzsmj4UXYh5ths5G5Uv");
}

pub mod quick_bail_on_panic {
    safecoin_sdk::declare_id!("HLRgJbaGwAJbmL6Tsu8N9oGfLhsc7DopsvP4UPjzjyr3");
}

pub mod nonce_must_be_authorized {
    safecoin_sdk::declare_id!("B9oxgU37tuCohjc322GiLiaD8c1LsZtJ7RwMjHWKKpVB");
}

pub mod nonce_must_be_advanceable {
    safecoin_sdk::declare_id!("8qKM9RwaBzj6u8Ax9AHBBy6GEwNcrduY6UmDF7KPqt8G");
}

pub mod vote_authorize_with_seed {
    safecoin_sdk::declare_id!("HeRuYxnBuFFAM9fd5FmaGSeBdHP4SEm1dR18rXUVJkyi");
}

pub mod cap_accounts_data_size_per_block {
    safecoin_sdk::declare_id!("FZpH2SfxGhFjmEWyzt6VY17ebJ7S9tcZ1D3XSpxF6qfj");
}

pub mod preserve_rent_epoch_for_rent_exempt_accounts {
    safecoin_sdk::declare_id!("8bxNk6MJqCQLeMrL2viWTKiSftGeUoie6KpEQD8dVzAd");
}

pub mod enable_bpf_loader_extend_program_ix {
    safecoin_sdk::declare_id!("GPg3fNRkPPTYvtJTEefRPPjtCer2fucW66iS74HUeSM9");
}

pub mod enable_early_verification_of_account_modifications {
    safecoin_sdk::declare_id!("9LD4rrdYAqUWRb39cWVzYgA9MVsVQAerTTnmbSrjbVSn");
}

pub mod skip_rent_rewrites {
    safecoin_sdk::declare_id!("CGB2jM8pwZkeeiXQ66kBMyBR6Np61mggL7XUsmLjVcrw");
}

pub mod prevent_crediting_accounts_that_end_rent_paying {
    safecoin_sdk::declare_id!("2dtQ7jzJtQdAWZM8baLKLo9taHw2Hg7fMLb6wvBL3LhG");
}

pub mod cap_bpf_program_instruction_accounts {
    safecoin_sdk::declare_id!("9k5ijzTbYPtjzu8wj2ErH9v45xecHzQ1x4PMYMMxFgdM");
}

pub mod loosen_cpi_size_restriction {
    safecoin_sdk::declare_id!("GDH5TVdbTPUpRnXaRyQqiKUa7uZAbZ28Q2N9bhbKoMLm");
}

pub mod use_default_units_in_fee_calculation {
    safecoin_sdk::declare_id!("8sKQrMQoUHtQSUP83SPG4ta2JDjSAiWs7t5aJ9uEd6To");
}

pub mod compact_vote_state_updates {
    safecoin_sdk::declare_id!("86HpNqzutEZwLcPxS6EHDcMNYWk6ikhteg9un7Y2PBKE");
}

pub mod incremental_snapshot_only_incremental_hash_calculation {
    safecoin_sdk::declare_id!("25vqsfjk7Nv1prsQJmA4Xu1bN61s8LXCBGUPp8Rfy1UF");
}

pub mod disable_cpi_setting_executable_and_rent_epoch {
    safecoin_sdk::declare_id!("B9cdB55u4jQsDNsdTK525yE9dmSc5Ga7YBaBrDFvEhM9");
}

pub mod on_load_preserve_rent_epoch_for_rent_exempt_accounts {
    safecoin_sdk::declare_id!("CpkdQmspsaZZ8FVAouQTtTWZkc8eeQ7V3uj7dWz543rZ");
}

pub mod account_hash_ignore_slot {
    safecoin_sdk::declare_id!("SVn36yVApPLYsa8koK3qUcy14zXDnqkNYWyUh1f4oK1");
}

pub mod set_exempt_rent_epoch_max {
    safecoin_sdk::declare_id!("5wAGiy15X1Jb2hkHnPDCM8oB9V42VNA9ftNVFK84dEgv");
}

pub mod relax_authority_signer_check_for_lookup_table_creation {
    safecoin_sdk::declare_id!("FKAcEvNgSY79RpqsPNUV5gDyumopH4cEHqUxyfm8b8Ap");
}

pub mod stop_sibling_instruction_search_at_parent {
    safecoin_sdk::declare_id!("EYVpEP7uzH1CoXzbD6PubGhYmnxRXPeq3PPsm1ba3gpo");
}

pub mod vote_state_update_root_fix {
    safecoin_sdk::declare_id!("G74BkWBzmsByZ1kxHy44H3wjwp5hp7JbrGRuDpco22tY");
}

pub mod cap_accounts_data_allocations_per_transaction {
    safecoin_sdk::declare_id!("9gxu85LYRAcZL38We8MYJ4A9AwgBBPtVBAqebMcT1241");
}

pub mod epoch_accounts_hash {
    safecoin_sdk::declare_id!("5GpmAKxaGsWWbPp4bNXFLJxZVvG92ctxf7jQnzTQjF3n");
}

pub mod remove_deprecated_request_unit_ix {
    safecoin_sdk::declare_id!("EfhYd3SafzGT472tYQDUc4dPd2xdEfKs5fwkowUgVt4W");
}

pub mod disable_rehash_for_rent_epoch {
    safecoin_sdk::declare_id!("DTVTkmw3JSofd8CJVJte8PXEbxNQ2yZijvVr3pe2APPj");
}

pub mod increase_tx_account_lock_limit {
    safecoin_sdk::declare_id!("9LZdXeKGeBV6hRLdxS1rHbHoEUsKqesCC2ZAPTPKJAbK");
}

pub mod limit_max_instruction_trace_length {
    safecoin_sdk::declare_id!("GQALDaC48fEhZGWRj9iL5Q889emJKcj3aCvHF7VCbbF4");
}

pub mod check_syscall_outputs_do_not_overlap {
    safecoin_sdk::declare_id!("3uRVPBpyEJRo1emLCrq38eLRFGcu6uKSpUXqGvU8T7SZ");
}

pub mod enable_bpf_loader_set_authority_checked_ix {
    safecoin_sdk::declare_id!("5x3825XS7M2A3Ekbn5VGGkvFoAg5qrRWkTrY4bARP1GL");
}

pub mod enable_alt_bn128_syscall {
    safecoin_sdk::declare_id!("A16q37opZdQMCbe5qJ6xpBB9usykfv8jZaMkxvZQi4GJ");
}

pub mod enable_program_redeployment_cooldown {
    safecoin_sdk::declare_id!("J4HFT8usBxpcF63y46t1upYobJgChmKyZPm5uTBRg25Z");
}

pub mod commission_updates_only_allowed_in_first_half_of_epoch {
    safecoin_sdk::declare_id!("noRuG2kzACwgaY7TVmLRnUNPLKNVQE1fb7X55YWBehp");
}

pub mod enable_turbine_fanout_experiments {
    safecoin_sdk::declare_id!("D31EFnLgdiysi84Woo3of4JMu7VmasUS3Z7j9HYXCeLY");
}

pub mod disable_turbine_fanout_experiments {
    safecoin_sdk::declare_id!("Gz1aLrbeQ4Q6PTSafCZcGWZXz91yVRi7ASFzFEr1U4sa");
}

pub mod drop_merkle_shreds {
    safecoin_sdk::declare_id!("84zy5N23Q9vTZuLc9h1HWUtyM9yCFV2SCmyP9W9C3yHZ");
}

pub mod keep_merkle_shreds {
    safecoin_sdk::declare_id!("HyNQzc7TMNmRhpVHXqDGjpsHzeQie82mDQXSF9hj7nAH");
}

pub mod move_serialized_len_ptr_in_cpi {
    safecoin_sdk::declare_id!("74CoWuBmt3rUVUrCb2JiSTvh6nXyBWUsK4SaMj3CtE3T");
}

pub mod update_hashes_per_tick {
    safecoin_sdk::declare_id!("3uFHb9oKdGfgZGJK9EHaAXN4USvnQtAFC13Fh5gGFS5B");
}

pub mod enable_big_mod_exp_syscall {
    safecoin_sdk::declare_id!("EBq48m8irRKuE7ZnMTLvLg2UuGSqhe8s8oMqnmja1fJw");
}

pub mod disable_builtin_loader_ownership_chains {
    safecoin_sdk::declare_id!("4UDcAfQ6EcA6bdcadkeHpkarkhZGJ7Bpq7wTAiRMjkoi");
}

pub mod cap_transaction_accounts_data_size {
    safecoin_sdk::declare_id!("DdLwVYuvDz26JohmgSbA7mjpJFgX5zP2dkp8qsF2C33V");
}

pub mod enable_request_heap_frame_ix {
    safecoin_sdk::declare_id!("Hr1nUA9b7NJ6eChS26o7Vi8gYYDDwWD3YeBfzJkTbU86");
}

pub mod prevent_rent_paying_rent_recipients {
    safecoin_sdk::declare_id!("Fab5oP3DmsLYCiQZXdjyqT3ukFFPrsmqhXU4WU1AWVVF");
}

lazy_static! {
    /// Map of feature identifiers to user-visible description
    pub static ref FEATURE_NAMES: HashMap<Pubkey, &'static str> = [
        (secp256k1_program_enabled::id(), "secp256k1 program"),
        (deprecate_rewards_sysvar::id(), "deprecate unused rewards sysvar"),
        (pico_inflation::id(), "pico inflation"),
        (full_inflation::devnet_and_testnet::id(), "full inflation on devnet and testnet"),
        (safe_token_v2_multisig_fix::id(), "safe-token multisig fix"),
        (no_overflow_rent_distribution::id(), "no overflow rent distribution"),
        (filter_stake_delegation_accounts::id(), "filter stake_delegation_accounts #14062"),
        (require_custodian_for_locked_stake_authorize::id(), "require custodian to authorize withdrawer change for locked stake"),
        (safe_token_v2_self_transfer_fix::id(), "safe-token self-transfer fix"),
        (full_inflation::mainnet::certusone::enable::id(), "full inflation enabled by Certus One"),
        (full_inflation::mainnet::certusone::vote::id(), "community vote allowing Certus One to enable full inflation"),
        (warp_timestamp_again::id(), "warp timestamp again, adjust bounding to 25% fast 80% slow #15204"),
        (check_init_vote_data::id(), "check initialized Vote data"),
        (secp256k1_recover_syscall_enabled::id(), "secp256k1_recover syscall"),
        (system_transfer_zero_check::id(), "perform all checks for transfers of 0 lamports"),
        (blake3_syscall_enabled::id(), "blake3 syscall"),
        (dedupe_config_program_signers::id(), "dedupe config program signers"),
        (verify_tx_signatures_len::id(), "prohibit extra transaction signatures"),
        (vote_stake_checked_instructions::id(), "vote/state program checked instructions #18345"),
        (rent_for_sysvars::id(), "collect rent from accounts owned by sysvars"),
        (libsecp256k1_0_5_upgrade_enabled::id(), "upgrade libsecp256k1 to v0.5.0"),
        (tx_wide_compute_cap::id(), "transaction wide compute cap"),
        (safe_token_v2_set_authority_fix::id(), "safe-token set_authority fix"),
        (merge_nonce_error_into_system_error::id(), "merge NonceError into SystemError"),
        (disable_fees_sysvar::id(), "disable fees sysvar"),
        (stake_merge_with_unmatched_credits_observed::id(), "allow merging active stakes with unmatched credits_observed #18985"),
        (zk_token_sdk_enabled::id(), "enable Zk Token proof program and syscalls"),
        (curve25519_syscall_enabled::id(), "enable curve25519 syscalls"),
        (versioned_tx_message_enabled::id(), "enable versioned transaction message processing"),
        (libsecp256k1_fail_on_bad_count::id(), "fail libsec256k1_verify if count appears wrong"),
        (libsecp256k1_fail_on_bad_count2::id(), "fail libsec256k1_verify if count appears wrong"),
        (instructions_sysvar_owned_by_sysvar::id(), "fix owner for instructions sysvar"),
        (stake_program_advance_activating_credits_observed::id(), "Enable advancing credits observed for activation epoch #19309"),
        (credits_auto_rewind::id(), "Auto rewind stake's credits_observed if (accidental) vote recreation is detected #22546"),
        (demote_program_write_locks::id(), "demote program write locks to readonly, except when upgradeable loader present #19593 #20265"),
        (ed25519_program_enabled::id(), "enable builtin ed25519 signature verify program"),
        (return_data_syscall_enabled::id(), "enable sol_{set,get}_return_data syscall"),
        (reduce_required_deploy_balance::id(), "reduce required payer balance for program deploys"),
        (sol_log_data_syscall_enabled::id(), "enable sol_log_data syscall"),
        (stakes_remove_delegation_if_inactive::id(), "remove delegations from stakes cache when inactive"),
        (do_support_realloc::id(), "support account data reallocation"),
        (prevent_calling_precompiles_as_programs::id(), "prevent calling precompiles as programs"),
        (optimize_epoch_boundary_updates::id(), "optimize epoch boundary updates"),
        (remove_native_loader::id(), "remove support for the native loader"),
        (send_to_tpu_vote_port::id(), "send votes to the tpu vote port"),
        (requestable_heap_size::id(), "Requestable heap frame size"),
        (disable_fee_calculator::id(), "deprecate fee calculator"),
        (add_compute_budget_program::id(), "Add compute_budget_program"),
        (nonce_must_be_writable::id(), "nonce must be writable"),
        (safe_token_v3_3_0_release::id(), "safe-token v3.3.0 release"),
        (leave_nonce_on_success::id(), "leave nonce as is on success"),
        (reject_empty_instruction_without_program::id(), "fail instructions which have native_loader as program_id directly"),
        (fixed_memcpy_nonoverlapping_check::id(), "use correct check for nonoverlapping regions in memcpy syscall"),
        (reject_non_rent_exempt_vote_withdraws::id(), "fail vote withdraw instructions which leave the account non-rent-exempt"),
        (evict_invalid_stakes_cache_entries::id(), "evict invalid stakes cache entries on epoch boundaries"),
        (allow_votes_to_directly_update_vote_state::id(), "enable direct vote state update"),
        (cap_accounts_data_len::id(), "cap the accounts data len"),
        (max_tx_account_locks::id(), "enforce max number of locked accounts per transaction"),
        (require_rent_exempt_accounts::id(), "require all new transaction accounts with data to be rent-exempt"),
        (filter_votes_outside_slot_hashes::id(), "filter vote slots older than the slot hashes history"),
        (update_syscall_base_costs::id(), "update syscall base costs"),
        (stake_deactivate_delinquent_instruction::id(), "enable the deactivate delinquent stake instruction #23932"),
        (vote_withdraw_authority_may_change_authorized_voter::id(), "vote account withdraw authority may change the authorized voter #22521"),
        (safe_associated_token_account_v1_0_4::id(), "SPL Associated Token Account Program release version 1.0.4, tied to token 3.3.0 #22648"),
        (reject_vote_account_close_unless_zero_credit_epoch::id(), "fail vote account withdraw to 0 unless account earned 0 credits in last completed epoch"),
        (add_get_processed_sibling_instruction_syscall::id(), "add add_get_processed_sibling_instruction_syscall"),
        (bank_transaction_count_fix::id(), "fixes Bank::transaction_count to include all committed transactions, not just successful ones"),
        (disable_bpf_deprecated_load_instructions::id(), "disable ldabs* and ldind* SBF instructions"),
        (disable_bpf_unresolved_symbols_at_runtime::id(), "disable reporting of unresolved SBF symbols at runtime"),
        (record_instruction_in_transaction_context_push::id(), "move the CPI stack overflow check to the end of push"),
        (syscall_saturated_math::id(), "syscalls use saturated math"),
        (check_physical_overlapping::id(), "check physical overlapping regions"),
        (limit_secp256k1_recovery_id::id(), "limit secp256k1 recovery id"),
        (disable_deprecated_loader::id(), "disable the deprecated BPF loader"),
        (check_slice_translation_size::id(), "check size when translating slices"),
        (stake_split_uses_rent_sysvar::id(), "stake split instruction uses rent sysvar"),
        (add_get_minimum_delegation_instruction_to_stake_program::id(), "add GetMinimumDelegation instruction to stake program"),
        (error_on_syscall_bpf_function_hash_collisions::id(), "error on bpf function hash collisions"),
        (reject_callx_r10::id(), "Reject bpf callx r10 instructions"),
        (drop_redundant_turbine_path::id(), "drop redundant turbine path"),
        (executables_incur_cpi_data_cost::id(), "Executables incur CPI data costs"),
        (fix_recent_blockhashes::id(), "stop adding hashes for skipped slots to recent blockhashes"),
        (update_rewards_from_cached_accounts::id(), "update rewards from cached accounts"),
        (safe_token_v3_4_0::id(), "SPL Token Program version 3.4.0 release #24740"),
        (safe_associated_token_account_v1_1_0::id(), "SPL Associated Token Account Program version 1.1.0 release #24741"),
        (default_units_per_instruction::id(), "Default max tx-wide compute units calculated per instruction"),
        (stake_allow_zero_undelegated_amount::id(), "Allow zero-lamport undelegated amount for initialized stakes #24670"),
        (require_static_program_ids_in_transaction::id(), "require static program ids in versioned transactions"),
        (stake_raise_minimum_delegation_to_1_sol::id(), "Raise minimum stake delegation to 1.0 SAFE #24357"),
        (stake_minimum_delegation_for_rewards::id(), "stakes must be at least the minimum delegation to earn rewards"),
        (add_set_compute_unit_price_ix::id(), "add compute budget ix for setting a compute unit price"),
        (disable_deploy_of_alloc_free_syscall::id(), "disable new deployments of deprecated sol_alloc_free_ syscall"),
        (include_account_index_in_rent_error::id(), "include account index in rent tx error #25190"),
        (add_shred_type_to_shred_seed::id(), "add shred-type to shred seed #25556"),
        (warp_timestamp_with_a_vengeance::id(), "warp timestamp again, adjust bounding to 150% slow #25666"),
        (separate_nonce_from_blockhash::id(), "separate durable nonce and blockhash domains #25744"),
        (enable_durable_nonce::id(), "enable durable nonce #25744"),
        (vote_state_update_credit_per_dequeue::id(), "Calculate vote credits for VoteStateUpdate per vote dequeue to match credit awards for Vote instruction"),
        (quick_bail_on_panic::id(), "quick bail on panic"),
        (nonce_must_be_authorized::id(), "nonce must be authorized"),
        (nonce_must_be_advanceable::id(), "durable nonces must be advanceable"),
        (vote_authorize_with_seed::id(), "An instruction you can use to change a vote accounts authority when the current authority is a derived key #25860"),
        (cap_accounts_data_size_per_block::id(), "cap the accounts data size per block #25517"),
        (stake_redelegate_instruction::id(), "enable the redelegate stake instruction #26294"),
        (preserve_rent_epoch_for_rent_exempt_accounts::id(), "preserve rent epoch for rent exempt accounts #26479"),
        (enable_bpf_loader_extend_program_ix::id(), "enable bpf upgradeable loader ExtendProgram instruction #25234"),
        (skip_rent_rewrites::id(), "skip rewriting rent exempt accounts during rent collection #26491"),
        (enable_early_verification_of_account_modifications::id(), "enable early verification of account modifications #25899"),
        (disable_rehash_for_rent_epoch::id(), "on accounts hash calculation, do not try to rehash accounts #28934"),
        (account_hash_ignore_slot::id(), "ignore slot when calculating an account hash #28420"),
        (set_exempt_rent_epoch_max::id(), "set rent epoch to Epoch::MAX for rent-exempt accounts #28683"),
        (on_load_preserve_rent_epoch_for_rent_exempt_accounts::id(), "on bank load account, do not try to fix up rent_epoch #28541"),
        (prevent_crediting_accounts_that_end_rent_paying::id(), "prevent crediting rent paying accounts #26606"),
        (cap_bpf_program_instruction_accounts::id(), "enforce max number of accounts per bpf program instruction #26628"),
        (loosen_cpi_size_restriction::id(), "loosen cpi size restrictions #26641"),
        (use_default_units_in_fee_calculation::id(), "use default units per instruction in fee calculation #26785"),
        (compact_vote_state_updates::id(), "Compact vote state updates to lower block size"),
        (incremental_snapshot_only_incremental_hash_calculation::id(), "only hash accounts in incremental snapshot during incremental snapshot creation #26799"),
        (disable_cpi_setting_executable_and_rent_epoch::id(), "disable setting is_executable and_rent_epoch in CPI #26987"),
        (relax_authority_signer_check_for_lookup_table_creation::id(), "relax authority signer check for lookup table creation #27205"),
        (stop_sibling_instruction_search_at_parent::id(), "stop the search in get_processed_sibling_instruction when the parent instruction is reached #27289"),
        (vote_state_update_root_fix::id(), "fix root in vote state updates #27361"),
        (cap_accounts_data_allocations_per_transaction::id(), "cap accounts data allocations per transaction #27375"),
        (epoch_accounts_hash::id(), "enable epoch accounts hash calculation #27539"),
        (remove_deprecated_request_unit_ix::id(), "remove support for RequestUnitsDeprecated instruction #27500"),
        (increase_tx_account_lock_limit::id(), "increase tx account lock limit to 128 #27241"),
        (limit_max_instruction_trace_length::id(), "limit max instruction trace length #27939"),
        (check_syscall_outputs_do_not_overlap::id(), "check syscall outputs do_not overlap #28600"),
        (enable_bpf_loader_set_authority_checked_ix::id(), "enable bpf upgradeable loader SetAuthorityChecked instruction #28424"),
        (enable_alt_bn128_syscall::id(), "add alt_bn128 syscalls #27961"),
        (enable_program_redeployment_cooldown::id(), "enable program redeployment cooldown #29135"),
        (commission_updates_only_allowed_in_first_half_of_epoch::id(), "validator commission updates are only allowed in the first half of an epoch #29362"),
        (enable_turbine_fanout_experiments::id(), "enable turbine fanout experiments #29393"),
        (disable_turbine_fanout_experiments::id(), "disable turbine fanout experiments #29393"),
        (drop_merkle_shreds::id(), "drop merkle shreds #29711"),
        (keep_merkle_shreds::id(), "keep merkle shreds #29711"),
        (move_serialized_len_ptr_in_cpi::id(), "cpi ignore serialized_len_ptr #29592"),
        (update_hashes_per_tick::id(), "Update desired hashes per tick on epoch boundary"),
        (enable_big_mod_exp_syscall::id(), "add big_mod_exp syscall #28503"),
        (disable_builtin_loader_ownership_chains::id(), "disable builtin loader ownership chains #29956"),
        (cap_transaction_accounts_data_size::id(), "cap transaction accounts data size up to a limit #27839"),
        (enable_request_heap_frame_ix::id(), "Enable transaction to request heap frame using compute budget instruction #30076"),
        (prevent_rent_paying_rent_recipients::id(), "prevent recipients of rent rewards from ending in rent-paying state #30151"),
        /*************** ADD NEW FEATURES HERE ***************/
    ]
    .iter()
    .cloned()
    .collect();

    /// Unique identifier of the current software's feature set
    pub static ref ID: Hash = {
        let mut hasher = Hasher::default();
        let mut feature_ids = FEATURE_NAMES.keys().collect::<Vec<_>>();
        feature_ids.sort();
        for feature in feature_ids {
            hasher.hash(feature.as_ref());
        }
        hasher.result()
    };
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct FullInflationFeaturePair {
    pub vote_id: Pubkey, // Feature that grants the candidate the ability to enable full inflation
    pub enable_id: Pubkey, // Feature to enable full inflation by the candidate
}

lazy_static! {
    /// Set of feature pairs that once enabled will trigger full inflation
    pub static ref FULL_INFLATION_FEATURE_PAIRS: HashSet<FullInflationFeaturePair> = [
        FullInflationFeaturePair {
            vote_id: full_inflation::mainnet::certusone::vote::id(),
            enable_id: full_inflation::mainnet::certusone::enable::id(),
        },
    ]
    .iter()
    .cloned()
    .collect();
}

/// `FeatureSet` holds the set of currently active/inactive runtime features
#[derive(AbiExample, Debug, Clone, Eq, PartialEq)]
pub struct FeatureSet {
    pub active: HashMap<Pubkey, Slot>,
    pub inactive: HashSet<Pubkey>,
}
impl Default for FeatureSet {
    fn default() -> Self {
        // All features disabled
        Self {
            active: HashMap::new(),
            inactive: FEATURE_NAMES.keys().cloned().collect(),
        }
    }
}
impl FeatureSet {
    pub fn is_active(&self, feature_id: &Pubkey) -> bool {
        self.active.contains_key(feature_id)
    }

    pub fn activated_slot(&self, feature_id: &Pubkey) -> Option<Slot> {
        self.active.get(feature_id).copied()
    }

    /// List of enabled features that trigger full inflation
    pub fn full_inflation_features_enabled(&self) -> HashSet<Pubkey> {
        let mut hash_set = FULL_INFLATION_FEATURE_PAIRS
            .iter()
            .filter_map(|pair| {
                if self.is_active(&pair.vote_id) && self.is_active(&pair.enable_id) {
                    Some(pair.enable_id)
                } else {
                    None
                }
            })
            .collect::<HashSet<_>>();

        if self.is_active(&full_inflation::devnet_and_testnet::id()) {
            hash_set.insert(full_inflation::devnet_and_testnet::id());
        }
        hash_set
    }

    /// All features enabled, useful for testing
    pub fn all_enabled() -> Self {
        Self {
            active: FEATURE_NAMES.keys().cloned().map(|key| (key, 0)).collect(),
            inactive: HashSet::new(),
        }
    }

    /// Activate a feature
    pub fn activate(&mut self, feature_id: &Pubkey, slot: u64) {
        self.inactive.remove(feature_id);
        self.active.insert(*feature_id, slot);
    }

    /// Deactivate a feature
    pub fn deactivate(&mut self, feature_id: &Pubkey) {
        self.active.remove(feature_id);
        self.inactive.insert(*feature_id);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_full_inflation_features_enabled_devnet_and_testnet() {
        let mut feature_set = FeatureSet::default();
        assert!(feature_set.full_inflation_features_enabled().is_empty());
        feature_set
            .active
            .insert(full_inflation::devnet_and_testnet::id(), 42);
        assert_eq!(
            feature_set.full_inflation_features_enabled(),
            [full_inflation::devnet_and_testnet::id()]
                .iter()
                .cloned()
                .collect()
        );
    }

    #[test]
    fn test_full_inflation_features_enabled() {
        // Normal sequence: vote_id then enable_id
        let mut feature_set = FeatureSet::default();
        assert!(feature_set.full_inflation_features_enabled().is_empty());
        feature_set
            .active
            .insert(full_inflation::mainnet::certusone::vote::id(), 42);
        assert!(feature_set.full_inflation_features_enabled().is_empty());
        feature_set
            .active
            .insert(full_inflation::mainnet::certusone::enable::id(), 42);
        assert_eq!(
            feature_set.full_inflation_features_enabled(),
            [full_inflation::mainnet::certusone::enable::id()]
                .iter()
                .cloned()
                .collect()
        );

        // Backwards sequence: enable_id and then vote_id
        let mut feature_set = FeatureSet::default();
        assert!(feature_set.full_inflation_features_enabled().is_empty());
        feature_set
            .active
            .insert(full_inflation::mainnet::certusone::enable::id(), 42);
        assert!(feature_set.full_inflation_features_enabled().is_empty());
        feature_set
            .active
            .insert(full_inflation::mainnet::certusone::vote::id(), 42);
        assert_eq!(
            feature_set.full_inflation_features_enabled(),
            [full_inflation::mainnet::certusone::enable::id()]
                .iter()
                .cloned()
                .collect()
        );
    }

    #[test]
    fn test_feature_set_activate_deactivate() {
        let mut feature_set = FeatureSet::default();

        let feature = Pubkey::new_unique();
        assert!(!feature_set.is_active(&feature));
        feature_set.activate(&feature, 0);
        assert!(feature_set.is_active(&feature));
        feature_set.deactivate(&feature);
        assert!(!feature_set.is_active(&feature));
    }
}
