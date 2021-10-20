// use std::collections::HashSet;
// use super::item::{Entity, Equipable, State, Curse};

// pub struct Wearings {
//     name: String,
//     image: &'static str,
//     shape: &'static str,
//     description: &'static str,
//     weight: i32,
//     durabillity: (u32, u32), //current, total
//     state: (State, HashSet<Curse>)
// }

// impl Wearings {
//     pub fn new(name: String, image: &'static str, shape: &'static str, description:&'static str, weight: i32, durabillity: (u32, u32), state: State) -> Box<Self> {
//         return Box::new(Self {
//             name,
//             image,
//             shape,
//             description,
//             weight,
//             durabillity,
//             state: (state, HashSet::new())
//         });
//     }
// }

// impl Entity for Wearings {
//     fn get_weight(&self) -> i32 {
//         self.weight
//     }

//     fn get_name(&self) -> &str {
//         self.name.as_ref()
//     }

//     fn take_damaged(&mut self, damage: u32) {
//         if self.durabillity.0 <= damage {
//             self.durabillity = (0, 0);
//             self.state.0 = State::Broken;
//         } else {
//             self.durabillity.0 -= damage;

//             if self.durabillity.0 < (self.durabillity.1 as f32 * 0.75).round() as u32 {
//                 self.state.0 = State::Damaged;
//             }
//         }
//     }

//     fn take_fixed(&mut self, point: u32) {
//         self.durabillity.0 += point;

//         if self.durabillity.0 >= self.durabillity.1 {
//             self.durabillity.0 = self.durabillity.1
//         }
//     }

//     fn enchant<T: Entity>(&mut self, item: T) -> Result<T, Option<T>> {
//         Ok(item)
//     }
// }

// impl Equipable for Wearings {

// }