// Partial SPL Token v2.0.x declarations inlined to avoid an external dependency on the safe-token crate
safecoin_sdk::declare_id!("7v5TwK92hUSqduoL3R8NtzTNfNzMA48nJL4mzPYMdDrD");

pub(crate) mod new_token_program {
    safecoin_sdk::declare_id!("nTokHfnBtpt4V6xiEbBSduiGCrQ6wSF3rxC8WeWAQ9F");
}

/*
    safe_token::state::Account {
        mint: Pubkey,
        owner: Pubkey,
        amount: u64,
        delegate: COption<Pubkey>,
        state: AccountState,
        is_native: COption<u64>,
        delegated_amount: u64,
        close_authority: COption<Pubkey>,
    }
*/
pub const SAFE_TOKEN_ACCOUNT_MINT_OFFSET: usize = 0;
pub const SAFE_TOKEN_ACCOUNT_OWNER_OFFSET: usize = 32;

pub mod state {
    const LEN: usize = 165;
    pub struct Account;
    impl Account {
        pub fn get_packed_len() -> usize {
            LEN
        }
    }
}

pub mod native_mint {
    safecoin_sdk::declare_id!("Safe111111111111111111111111111111111111112");

    /*
        Mint {
            mint_authority: COption::None,
            supply: 0,
            decimals: 9,
            is_initialized: true,
            freeze_authority: COption::None,
        }
    */
    pub const ACCOUNT_DATA: [u8; 82] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
}