pub trait Entity {
    fn get_name(&self) -> &str;
    fn get_weight(&self) -> i32;

    fn take_damaged(&mut self, damage: u32);
    fn take_fixed(&mut self, point: u32);

    fn enchant<T: Entity>(&mut self, item: T) -> Result<T, Option<T>>;
}

pub trait Equipable: Entity {
    // fn carve_slot<T: Entity>(&mut self, item: Self) -> Result<T, Option<T>>;
}

pub trait Disposable: Entity {
    fn spend(&self, count: u32, item: Self, action: dyn FnMut()->Result<(), ()>);
}

pub enum State {
    Normal,
    Damaged,
    Broken
}


pub enum Curse {
    None,
    ReverseProbability,
    Weak,
    Unstable
}

