#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype,
    symbol_short, Env, String, Symbol, Vec
};

#[contracttype]
#[derive(Clone)]
pub struct Journal {
    tanggal: String,
    mood: String,
    catatan: String,
}

const DATA: Symbol = symbol_short!("DATA");

#[contract]
pub struct MoodJournal;

#[contractimpl]
impl MoodJournal {

    pub fn tambah_journal(
        env: Env,
        tanggal: String,
        mood: String,
        catatan: String
    ) -> String {

        let mut journals: Vec<Journal> =
            env.storage()
            .instance()
            .get(&DATA)
            .unwrap_or(Vec::new(&env));

        let journal = Journal {
            tanggal,
            mood,
            catatan,
        };

        journals.push_back(journal);

        env.storage().instance().set(&DATA, &journals);

        String::from_str(&env, "Journal berhasil ditambah")
    }

    pub fn lihat_journal(env: Env) -> Vec<Journal> {
        env.storage()
            .instance()
            .get(&DATA)
            .unwrap_or(Vec::new(&env))
    }
}