// use super::items::item;

// pub struct Inventory {
//     gold: u32,
//     weight_total: u32,
//     weight_current: u32,
//     items: Vec<item::Entity>
// }

// impl Inventory {
//     pub fn new(weight_total: u32, capacity: usize) -> Self {
//         Inventory {
//             gold: 0,
//             weight_total,
//             weight_current: 0,
//             items: Vec::<GameItem>::with_capacity(capacity)
//         }
//     }

//     fn accumulate_weight(&mut self, weight: u32) -> Result<(), ()> {
//         self.weight_current += weight;

//         if self.weight_current <= self.weight_total {
//             return Ok(());
//         } else {
//             self.weight_current -= weight;
//             return Err(());
//         }
//     }

//     pub fn get_gold(&self) -> u32 {
//         return self.gold;
//     }

//     pub fn get_weight(&self) -> u32 {
//         return self.weight_current;
//     }

//     pub fn take_item(&mut self, item: GameItem) -> Result<(), GameItem> {
//         let weight = match &item {
//             GameItem::Equipable(_, item) => item.get_weight(),
//             GameItem::Wearables(_, item) => item.get_weight(),
//             GameItem::Disposables(_, item) => item.get_weight(),
//             GameItem::Gold(count) => *count
//         };

//         match self.accumulate_weight(weight) {
//             Ok(()) => {
//                 self.items.push(item);
//                 return Ok(());
//             },
//             Err(()) => {
//                 return Err(item);
//             }
//         }
//     }

//     pub fn drop_item(&mut self, item_id: u32, count: u32) -> Result<(), GameItem> {
//         // self.items.iter().find(|x| {x});
//         return Ok(());
//     }


//     pub fn get_items(&self) -> &Vec<GameItem> {
//         return &self.items;
//     }
// }