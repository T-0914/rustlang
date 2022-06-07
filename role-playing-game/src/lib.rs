// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]
#[derive(Debug)]
pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        match Player::is_dead(self) {
            true => {
                if Player::not_a_wizard_yet(self) {
                    Some(Self {
                        health: 100,
                        // mana: self.mana,
                        // level: self.level,
                        ..*self
                    })
                } else {
                    Some(Self {
                        health: 100,
                        mana: Some(100),
                        // level: self.level,
                        ..*self
                    })
                }
            }
            false => None,
        }
    }
    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match Player::not_a_wizard_yet(self) {
            true => {
                if mana_cost < self.health {
                    Player::self_damaged(self, mana_cost);
                } else {
                    Player::self_damaged(self, self.health);
                }
                0
            }
            false => {
                if mana_cost > self.mana.unwrap() {
                    0
                } else {
                    self.mana = Some(self.mana.unwrap() - mana_cost);
                    mana_cost * 2
                }
            }
        }
    }
    fn is_dead(&self) -> bool {
        self.health == 0
    }

    fn not_a_wizard_yet(&self) -> bool {
        self.level < 10
    }

    fn self_damaged(&mut self, mana_cost: u32) {
        self.health = self.health - mana_cost;
    }
}
