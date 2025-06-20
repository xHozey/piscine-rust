mod mobs;
use crate::mobs::Mob;
use mobs::{member::*, boss::*};
use mobs::*;

impl Member {
    pub fn new(age: u32) -> Self {
        Self {
            age,
            role: Role::Associate
        }
    }
    pub fn get_promotion(&mut self) {
        match self.role {
                Role::Associate => self.role = Role::Soldier,
                Role::Caporegime => self.role = Role::Underboss,
                Role::Soldier => self.role = Role::Caporegime,
                Role::Underboss => panic!(),
        }
    }
}

impl Boss {
    pub fn new(name: &str, age: u32) -> Self {
        Self {
            name: name.to_string(), age
        }
    }
}

impl Mob {
    pub fn recruit(&mut self, member: (&str, u32)) {
        self.members.insert(member.0.to_string(), Member::new(member.1));
    }
    pub fn attack(&mut self, mob: &mut Mob) {
        let mut attacker_score = 0;
        for (_, m) in &self.members {
            match m.role {
                Role::Associate => attacker_score += 1,
                Role::Caporegime => attacker_score += 3,
                Role::Soldier => attacker_score += 2,
                Role::Underboss => attacker_score += 4,
            }
        }
        let mut defender_score = 0;
        for (_, m) in &mob.members {
            match m.role {
                Role::Associate => defender_score += 1,
                Role::Caporegime => defender_score += 3,
                Role::Soldier => defender_score += 2,
                Role::Underboss => defender_score += 4,
            }
        }
        if attacker_score <= defender_score {

        }
    }
    pub fn steal(&mut self, target: &mut Mob, steal_worth: u64) {

    }
    fn conquer_city(&mut self, _mobs: &[&Mob],_city: String ) {

    }
}