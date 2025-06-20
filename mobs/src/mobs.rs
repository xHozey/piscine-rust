use std::collections::{HashMap, HashSet};
pub mod boss;
pub mod member;
use boss::Boss;
use member::Member;

pub struct Mob {
    pub name: String,
    pub boss: Boss,
    pub members: HashMap<String, Member>,
    pub cities: HashSet<String>,
    pub wealth: u64,
}