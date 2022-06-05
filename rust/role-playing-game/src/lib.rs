pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    fn new(level: u32) -> Self {
        Self {
            health: 100,
            mana: (level >= 10).then(|| 100),
            level,
        }
    }

    pub fn revive(&self) -> Option<Player> {
        match self.health {
            0 => Some(Player::new(self.level)),
            _ => None,
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            Some(ref mut mana) if *mana >= mana_cost => {
                *mana -= mana_cost;
                mana_cost * 2
            }
            Some(_) => 0,
            None => {
                self.health -= self.health.min(mana_cost);
                0
            }
        }
    }
}
