use super::{Entity, Equipable, State, Curse, Spec};

pub struct WeaponMelee {
    name: String,
    image: &'static str,
    description: &'static str,
    weight: i16,
    damage: u32,
    durabillity: (u8, u8), //current, total
    state: (State, [Curse; 4], [Spec; 11])
}

impl WeaponMelee {
    pub fn new(name: String, image: &'static str, description: &'static str, damage: u32, weight: i16, durabillity: (u8, u8), state: State) -> Self {
        return Self {
            name,
            image,
            damage,
            description,
            weight,
            durabillity,
            state: (
                state,
                [Curse::Nothing; 4],
                [Spec::Nothing; 11]
            )
        };
    }

    // pub fn attack(&mut self, )
}

impl Entity for WeaponMelee {
    fn get_weight(&self) -> i16 {
        return self.weight;
    }

    fn get_name(&self) -> &str { 
        return self.name.as_ref();
    }

    fn take_damaged(&mut self, damage: u32) {
        let point: u8 = 10;

        if self.durabillity.0 <= point {
            self.durabillity = (0, 0);
            self.state.0 = State::Broken;
        } else {
            self.durabillity.0 -= point;

            if self.durabillity.0 < (self.durabillity.1 as f32 * 0.75).round() as u8 {
                self.state.0 = State::Damaged;
            }
        }
    }

    fn take_fixed(&mut self, point: u8) {
        self.durabillity.0 += point;

        if self.durabillity.0 >= self.durabillity.1 {
            self.durabillity.0 = self.durabillity.1
        }
    }

    fn enchant<T: Entity>(&mut self, item: T) -> Result<T, Option<T>> {
        Ok(item)
    }
}

impl Equipable for WeaponMelee {

}