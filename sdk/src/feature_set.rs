use lazy_static::lazy_static;
use solana_sdk::{
    clock::Slot,
    hash::{Hash, Hasher},
    pubkey::Pubkey,
};
use std::collections::{HashMap, HashSet};

pub mod instructions_sysvar_enabled {
    solana_sdk::declare_id!("7TfFp6Tf2XqXQQfx16qvXbjekXtn68kiQj9pPfXZ5Bua");
}

pub mod lamports_mystery_solved { // make bighead feel better
    solana_sdk::declare_id!("2UZU7sLDrA2t8RAjRcxe2mUiCfkh3dBG5chef8QVR8or");
}

pub mod consistent_recent_blockhashes_sysvar {
    solana_sdk::declare_id!("ApdySrEmykK3PBLExdN2ehGCRPyVT6athFgu4H7H8e9J");
}

pub mod deprecate_rewards_sysvar {
    solana_sdk::declare_id!("GVriKcZATdgSzmhmu9PtimfheaKYksK5mhgoN5fbRhXg");
}

pub mod pico_inflation {
    solana_sdk::declare_id!("HC5H1tGb2RBRbrECSKmBVjwTsEEJziTTRsm3XSdTefrJ");
}

pub mod full_inflation {
    pub mod devnet_and_testnet {
        solana_sdk::declare_id!("JC1GQujpgHz92Am65FxcxByqqDGTdYJs6jLjgebnDpJF");
    }

    pub mod mainnet {
        pub mod certusone {
            pub mod vote {
                solana_sdk::declare_id!("7Ra6gKzEBFyYJ7fdsrCuzihZcxKu7AbmDy2FBEQFzap2");
            }
            pub mod enable {
                solana_sdk::declare_id!("54ZqXvMyCKShnBrGj2ni6m4kXTjRUrkc21dFoT9npJZk");
            }
        }
    }
}

pub mod spl_token_v2_multisig_fix {
    solana_sdk::declare_id!("EwSWSBRpzYZSEqdZ215WMmnce6WiEsk57rSEB3e7ghh6");
}

pub mod no_overflow_rent_distribution {
    solana_sdk::declare_id!("9TyDRDhs933rTCWGwzSUTSx1XeJT14sc17o1cNQzUaBq");
}

pub mod stake_program_v2 {
    solana_sdk::declare_id!("BpCWufPmpbYaT1xRbdCeZKXDxQ4Wj1Z8ijaG6tWFrJg1");
}

pub mod rewrite_stake {
    solana_sdk::declare_id!("HpGqShCRhP7QwMBXTs1KbATiHWa383EUWjg3kbQjN2Kf");
}

pub mod filter_stake_delegation_accounts {
    solana_sdk::declare_id!("HpGqShCRhP7QwMBXTs1KbATiHWa383EUWjg3kbQjN2Kf");
}

pub mod bpf_loader_upgradeable_program {
    solana_sdk::declare_id!("Cv6gGxiakDF6nd9Sxx53MbC4kij69qXpap8guiC6aK9U");
}

pub mod stake_program_v3 {
    solana_sdk::declare_id!("6tYrCsaWbGqgeW9tN3NRbViw6BBLYjnNsJBqLJZZoo5B");
}

pub mod require_custodian_for_locked_stake_authorize {
    solana_sdk::declare_id!("FKWSvfcXATHSBBNvr5VE6ns4tNsTG3EGzcDw2xVtowZQ");
}

pub mod spl_token_v2_self_transfer_fix {
    solana_sdk::declare_id!("2XDc17ZmSTbpqV3B5fmEGac4CKCYnbJj7vnfASvdzqyN");
}

pub mod warp_timestamp_again {
    solana_sdk::declare_id!("EgfbGcUzJotpXW8H4TnhVwDbrjm75xecKnrSAfqv4YvW");
}

pub mod check_init_vote_data {
    solana_sdk::declare_id!("CfH5RJxvjkQ1LaBSrnCDnYvHZtL6FuTQ751myi6DJEb5");
}

pub mod check_program_owner {
    solana_sdk::declare_id!("93ksjAw2xaVgWcBaqQSasj6hNiX3yUkPLpVZ8hoE2jE6");
}

pub mod cpi_share_ro_and_exec_accounts {
    solana_sdk::declare_id!("6jpDA7yjNRZQvSDA287AViVQpwB4Q4tXPqAdPF52McSo");
}

pub mod skip_ro_deserialization {
    solana_sdk::declare_id!("1B3gx5ERfig2px7sLiyXx7Kg6nZZkvZBJ1MkUQGQUTj");
}

pub mod require_stake_for_gossip {
    solana_sdk::declare_id!("7WM5jgvcKh9yyEj4NhG6qt1MvahRB6mhkodaY5KQwT4k");
}

pub mod cpi_data_cost {
    solana_sdk::declare_id!("6U62sD7ffNehUa7QLia5DUPNmFvGeZgrGLnRPo5Jdw9v");
}

pub mod upgradeable_close_instruction {
    solana_sdk::declare_id!("EzdQuAfpfg1pDTLyj52PLMPJDDCo4aqzncsSprFvMvU5");
}

pub mod demote_sysvar_write_locks {
    solana_sdk::declare_id!("2BLLVga5LkTyuq79fQqFpnsLHhQyjzvd9MoEBFshF419");
}

pub mod sysvar_via_syscall {
    solana_sdk::declare_id!("92ep2YChAdo4XiDPmNkRcAfJZuRv46n5ko9z6S512awn");
}

pub mod check_duplicates_by_hash {
    solana_sdk::declare_id!("32PCUa9RhQc3ujBLj5VmC18es2nigw39qLBeR5bskJ7P");
}

pub mod enforce_aligned_host_addrs {
    solana_sdk::declare_id!("6yrRvLK2AT8U9dq4aSArH3MSRdHbSYQsCd99zHxeAXJ8");
}
pub mod set_upgrade_authority_via_cpi_enabled {
    solana_sdk::declare_id!("8voJR1noBKPVBgE7BV6g3DCu4FJshGmCs1dRPuKnyJH5");
}

pub mod update_data_on_realloc {
    solana_sdk::declare_id!("DcTaQRy9wRBGywHkFStEnBW8V7yiJkGADXYbByjM3UEQ");
}

pub mod keccak256_syscall_enabled {
    solana_sdk::declare_id!("8ubpzsyCpCcDVccAnPNuQV439SPw5xdCpFryBMU8wPv1");
}

pub mod stake_program_v4 {
    solana_sdk::declare_id!("8Sgh17Pmrt6kKCJuNFGzJTnwRSMt1ELCSBuYThLznCYR");
}

pub mod system_transfer_zero_check {
    solana_sdk::declare_id!("ACrERgP7eXsQquYUNQwASR884izTTXnR2jDnxjWRPEde");
}

pub mod memory_ops_syscalls {
    solana_sdk::declare_id!("45aF3i2bmR2e9bPnZNpyRVZexjbaYKyq7f4PG59jhXxg");
}

pub mod dedupe_config_program_signers {
    solana_sdk::declare_id!("Ffpg1Q73ocztQKMTbkTMeTAVpcGUb95u2aUaCeFQ47NY");
}

pub mod vote_stake_checked_instructions {
    solana_sdk::declare_id!("BjfQ729TvPN84xsvsGMUsRLAqGc1ttSCQ6C5Zd3rUg2N");
}

pub mod updated_verify_policy {
    solana_sdk::declare_id!("8JWHY3gNMHMMia4smn5fa6KH91znav6qUzLMAJVNaNoY");
}

pub mod libsecp256k1_0_5_upgrade_enabled {
    solana_sdk::declare_id!("EVHL8iX15Gf6PpjdMd7pqPivjPQ2LLbK9fkGsrsGWy9r");
}

pub mod merge_nonce_error_into_system_error {
    solana_sdk::declare_id!("4n5Ko6ax8yLi21CXoBMFbCy52QydH7jpy42W5df7GZqT");
}

pub mod voter_groups_consensus {
    solana_sdk::declare_id!("CyQUUV5ymWwFYoafJZ7bvVa73zukmE31LcvKs5pqRtxz");
}

lazy_static! {
    /// Map of feature identifiers to user-visible description
    pub static ref FEATURE_NAMES: HashMap<Pubkey, &'static str> = [
        (instructions_sysvar_enabled::id(), "instructions sysvar"),
        (consistent_recent_blockhashes_sysvar::id(), "consistent recentblockhashes sysvar"),
        (deprecate_rewards_sysvar::id(), "deprecate unused rewards sysvar"),
        (pico_inflation::id(), "pico inflation"),
        (full_inflation::devnet_and_testnet::id(), "full inflation on devnet and testnet"),
        (spl_token_v2_multisig_fix::id(), "safe-token multisig fix"),
        (no_overflow_rent_distribution::id(), "no overflow rent distribution"),
        (stake_program_v2::id(), "solana_stake_program v2"),
        (rewrite_stake::id(), "rewrite stake"),
        (filter_stake_delegation_accounts::id(), "filter stake_delegation_accounts #14062"),
        (bpf_loader_upgradeable_program::id(), "upgradeable bpf loader"),
        (stake_program_v3::id(), "solana_stake_program v3"),
        (require_custodian_for_locked_stake_authorize::id(), "require custodian to authorize withdrawer change for locked stake"),
        (spl_token_v2_self_transfer_fix::id(), "safe-token self-transfer fix"),
        (full_inflation::mainnet::certusone::enable::id(), "full inflation enabled by Certus One"),
        (full_inflation::mainnet::certusone::vote::id(), "community vote allowing Certus One to enable full inflation"),
        (warp_timestamp_again::id(), "warp timestamp again, adjust bounding to 25% fast 80% slow #15204"),
        (check_init_vote_data::id(), "check initialized Vote data"),
        (check_program_owner::id(), "limit programs to operating on accounts owned by itself"),
        (require_stake_for_gossip::id(), "require stakes for propagating crds values through gossip #15561"),
        (cpi_data_cost::id(), "charge the compute budget for data passed via CPI"),
        (upgradeable_close_instruction::id(), "close upgradeable buffer accounts"),
        (demote_sysvar_write_locks::id(), "demote builtins and sysvar write locks to readonly #15497"),
        (sysvar_via_syscall::id(), "provide sysvars via syscalls"),
        (check_duplicates_by_hash::id(), "use transaction message hash for duplicate check"),
        (enforce_aligned_host_addrs::id(), "enforce aligned host addresses"),
        (update_data_on_realloc::id(), "Retain updated data values modified after realloc via CPI"),
        (set_upgrade_authority_via_cpi_enabled::id(), "set upgrade authority instruction via cpi calls for upgradable programs"),
        (keccak256_syscall_enabled::id(), "keccak256 syscall"),
        (stake_program_v4::id(), "solana_stake_program v4"),
        (system_transfer_zero_check::id(), "perform all checks for transfers of 0 lamports"),
        (memory_ops_syscalls::id(), "add syscalls for memory operations"),
        (dedupe_config_program_signers::id(), "dedupe config program signers"),
        (vote_stake_checked_instructions::id(), "vote/state program checked instructions #18345"),
        (updated_verify_policy::id(), "Update verify policy"),
        (libsecp256k1_0_5_upgrade_enabled::id(), "upgrade libsecp256k1 to v0.5.0"),
        (merge_nonce_error_into_system_error::id(), "merge NonceError into SystemError"),
        (voter_groups_consensus::id(), "use a random subset of voters for block consensus"),
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
#[derive(AbiExample, Debug, Clone)]
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
}
