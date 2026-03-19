#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype,
    symbol_short, Address, Env, Symbol, Vec, Map,
};

#[contract]
pub struct NFTGallery;

#[derive(Clone)]
#[contracttype]   // ⭐ THIS FIXES YOUR ERROR
pub struct NFT {
    pub id: u64,
    pub owner: Address,
    pub metadata: Symbol,
}

#[contractimpl]
impl NFTGallery {

    // Mint NFT
    pub fn mint(env: Env, owner: Address, id: u64, metadata: Symbol) {
        owner.require_auth();

        let mut storage: Map<u64, NFT> =
            env.storage()
                .instance()
                .get(&symbol_short!("NFTS"))
                .unwrap_or(Map::new(&env));

        let nft = NFT {
            id,
            owner: owner.clone(),
            metadata,
        };

        storage.set(id, nft);
        env.storage().instance().set(&symbol_short!("NFTS"), &storage);
    }

    // Transfer NFT
    pub fn transfer(env: Env, from: Address, to: Address, id: u64) {
        from.require_auth();

        let mut storage: Map<u64, NFT> =
            env.storage().instance().get(&symbol_short!("NFTS")).unwrap();

        let mut nft = storage.get(id).unwrap();

        if nft.owner != from {
            panic!("Not owner");
        }

        nft.owner = to.clone();
        storage.set(id, nft);

        env.storage().instance().set(&symbol_short!("NFTS"), &storage);
    }

    // Get single NFT
    pub fn get_nft(env: Env, id: u64) -> NFT {
        let storage: Map<u64, NFT> =
            env.storage().instance().get(&symbol_short!("NFTS")).unwrap();

        storage.get(id).unwrap()
    }

    // List NFTs
    pub fn list_all(env: Env) -> Vec<NFT> {
        let storage: Map<u64, NFT> =
            env.storage()
                .instance()
                .get(&symbol_short!("NFTS"))
                .unwrap_or(Map::new(&env));

        let mut result = Vec::new(&env);

        for (_, nft) in storage.iter() {
            result.push_back(nft);
        }

        result
    }
}