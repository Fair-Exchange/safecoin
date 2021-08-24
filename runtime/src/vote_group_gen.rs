//! This thing generates random voter groups of a given size
//! Given the set of all authorized voters (their pubkeys) it selects one randomly
//! then it picks a shift distance (some prime number less than the voter set size)
//! and iteratively selects the rest of the group by shifting that distance
//! its treating the set of voters as a ring

use solana_sdk::{
    pubkey::Pubkey,
    hash::Hash,
};
use std::collections::HashMap;
use std::convert::TryInto;
use std::str::FromStr;

pub static SAFECOIN_ALWAYS_VOTER: &str = "83E5RMejo6d98FV1EAXTx5t4bvoDMoxE4DboDee3VJsu";

//#[derive(Clone, Debug, Serialize, Deserialize, AbiExample, PartialEq)]
//pub struct ArcPubkey(std::sync::Arc<Pubkey>);

#[derive(Clone, Debug, Serialize, Deserialize, AbiExample, PartialEq)]
pub struct VoteGroupGenerator {
    possible_voters: Vec<Pubkey>,
    all_distance: Vec<u32>, // a list of primes that are not factors of the possible voters group size
    pub has_ringer: bool,
    pub group_size: usize,
}

impl VoteGroupGenerator {
    pub fn always_voter() -> Pubkey {
         solana_sdk::pubkey::Pubkey::from_str(SAFECOIN_ALWAYS_VOTER).unwrap()
    }

    pub fn new(map: &HashMap<Pubkey, Pubkey>, size: usize) -> VoteGroupGenerator {
        let mut grp_size = size;
        let collected: Vec<_> = map.into_iter().collect();
        let mut temp = Vec::new();
        for x in collected {
            let key = x.0;
            if key.to_string() != SAFECOIN_ALWAYS_VOTER {
                let cloned: Pubkey = Pubkey::new_from_array(key.to_bytes());
                temp.push(cloned);
            }
            else {
                grp_size = grp_size - 1;
            }
        }
        let len = temp.len() as u32;
        let mut initial = Vec::new();
        initial.push(1);
        for val in [
            2, 3, 5, 7, 11, 13, 17, 23, 29, 31, 37, 41, 43, 47, 51, 53, 57, 59, 61, 67, 71, 73, 79,
            83, 87, 89, 97, 101, 103,
        ]
        .iter()
        {
            if (len > *val) && ((len % *val) != 0) {
                initial.push(*val);
            }
        }
        Self {
            possible_voters: temp,
            all_distance: initial.to_owned(),
            group_size: grp_size,
            has_ringer: grp_size < size,
        }
    }

    pub fn new_dummy() -> VoteGroupGenerator {
        let hm: HashMap<Pubkey, Pubkey> = HashMap::new();
        Self::new(&hm, 1)
    }

    fn ring_shift(&self, a: usize, b: usize) -> usize {
        let temp = a + b;
        temp % self.possible_voters.len()
    }

    pub fn in_group_for_hash(&self, hash: Hash, test_key: Pubkey) -> bool {
        fn hash2u64(hash_val: Hash,start_loc: usize) -> u64 {
            fn pop64(hunk: &[u8]) -> &[u8; 8] {
                hunk.try_into().expect("slice with incorrect length")
            }
            let ary = hash_val.to_bytes();
            let max = ary.len();
            if (max % 8) != 0 {
                panic!("bad hash");
            }
            let mut idx = start_loc;
            let mut val :u64 = 0;
            while idx < max {
                let temp = pop64(&ary[idx..(idx+8)]);
                let  valx  = u64::from_le_bytes(*temp);
                val = val ^ valx;
                idx += 8;
            }
            val
        }

        let seed1 = hash2u64(hash,0);
        let seed2 = hash2u64(hash,64);
        self.in_group_using_seeds(seed1,seed2,test_key)
    }

    pub fn in_group_using_seeds(&self, voter_seed: u64,  step_seed: u64,test_key: Pubkey) -> bool {
        if test_key.to_string() == SAFECOIN_ALWAYS_VOTER {
           return true;
        }
        let voters_len = self.possible_voters.len();
        let mut loc = (voter_seed % voters_len as u64) as usize;
        let first_key = Pubkey::new(&self.possible_voters[loc].to_bytes());
        if test_key == first_key {
            return true;
        }
        if self.group_size > 1 {
            let choose_dist = step_seed % self.all_distance.len() as u64;
            let dist = self.all_distance[choose_dist as usize] as usize;
            for _ in 0..(self.group_size - 1) {
                loc = self.ring_shift(loc, dist);
                let loc_key = Pubkey::new(&self.possible_voters[loc].to_bytes());
                if test_key == loc_key {
                    log::trace!("found {:?}", test_key);
                    return true;
                }
            }
        }
        false
    }
}

    #[test]
    fn test_vgg_multi() {
        let canary = Pubkey::new_unique();
        let mut hm: HashMap<Pubkey, Pubkey> = HashMap::new();
        hm.insert(canary, Pubkey::new_unique());

        for it in 0..4 {
            let val = Pubkey::new_unique();
            hm.insert(val, Pubkey::new_unique());
            println!("insert {}", it);
        }
        let vgg = VoteGroupGenerator::new(&hm, hm.len());
        for h in hm.keys() {
            let found = vgg.in_group_using_seeds(0, 1, *h);
            assert!(found);
        }

        let not_canary = Pubkey::new_unique();
        assert_eq!(vgg.in_group_using_seeds(0, 1, not_canary), false);
    }

    #[test]
    fn test_vgg_single() {
        let canary = Pubkey::new_unique();
        let mut hm: HashMap<Pubkey, Pubkey> = HashMap::new();
        hm.insert(canary, Pubkey::new_unique());

        let vgg = VoteGroupGenerator::new(&hm, hm.len());
        for h in hm.keys() {
            let found = vgg.in_group_using_seeds(0, 1, *h);
            assert!(found);
        }

        let not_canary = Pubkey::new_unique();
        assert_eq!(vgg.in_group_using_seeds(0, 1, not_canary), false);
    }

    #[test]
    fn test_vgg_magic() {
        let magic = solana_sdk::pubkey::Pubkey::from_str(SAFECOIN_ALWAYS_VOTER).unwrap();
        let mut hm: HashMap<Pubkey, Pubkey> = HashMap::new();
        hm.insert(magic, Pubkey::new_unique());

        for it in 0..4 {
            let val = Pubkey::new_unique();
            hm.insert(val, Pubkey::new_unique());
            println!("insert {}", it);
        }
        let vgg = VoteGroupGenerator::new(&hm, hm.len());
        assert_eq!(vgg.has_ringer,true);
        for h in hm.keys() {
            let found = vgg.in_group_using_seeds(0,1, *h);
            let result = h.to_string() != SAFECOIN_ALWAYS_VOTER;
            assert_eq!(found, result);
        }
        assert_eq!(vgg.in_group_using_seeds(0,1, magic), true);
    }

