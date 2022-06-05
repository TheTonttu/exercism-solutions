pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        (self.health == 0).then(|| Self {
            health: 100,
            mana: self.mana.and(Some(100)),
            level: self.level,
        })
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        if let Some(ref mut mana) = self.mana {
            if *mana >= mana_cost {
                *mana -= mana_cost;
                mana_cost * 2
            } else {
                0
            }
        } else {
            self.health = self.health.saturating_sub(mana_cost);
            0
        }
    }
}
