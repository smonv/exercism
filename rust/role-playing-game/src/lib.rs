// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        if self.health == 0 {
            let mana = if self.level >= 10 { Some(100) } else { None };

            return Some(Player {
                health: 100,
                mana,
                level: self.level,
            });
        }

        None
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        if let Some(mana) = self.mana {
            if mana_cost > mana {
                return 0;
            }

            self.mana = Some(mana - mana_cost);
        } else {
            if mana_cost > self.health {
                self.health = 0;
            } else {
                self.health = self.health - mana_cost;
            }

            return 0;
        }

        mana_cost * 2
    }
}
