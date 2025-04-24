#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, String, Symbol, symbol_short, log};

#[contracttype]
#[derive(Clone)]
pub struct Photo {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub tips_received: u64,
}

#[contract]
pub struct PhotographerPortfolio;

const PHOTO_COUNT: Symbol = symbol_short!("P_COUNT");

#[contractimpl]
impl PhotographerPortfolio {
    pub fn upload_photo(env: Env, title: String, description: String) -> u64 {
        let mut photo_count: u64 = env.storage().instance().get(&PHOTO_COUNT).unwrap_or(0);
        photo_count += 1;

        let new_photo = Photo {
            id: photo_count,
            title,
            description,
            tips_received: 0,
        };

        env.storage().instance().set(&photo_count, &new_photo);
        env.storage().instance().set(&PHOTO_COUNT, &photo_count);
        env.storage().instance().extend_ttl(5000, 5000);

        log!(&env, "Uploaded Photo ID: {}", photo_count);
        photo_count
    }

    pub fn view_photo(env: Env, id: u64) -> Photo {
        env.storage().instance().get(&id).unwrap()
    }

    pub fn tip_photo(env: Env, id: u64, amount: u64) {
        let mut photo: Photo = env.storage().instance().get(&id).unwrap();
        photo.tips_received += amount;
        env.storage().instance().set(&id, &photo);

        log!(&env, "Photo ID {} tipped with {}", id, amount);
    }
}
