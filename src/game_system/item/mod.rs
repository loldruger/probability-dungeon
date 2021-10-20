pub mod wearings;
pub mod weapon_melee;
// pub mod weapon_ranged;
pub mod disposables;

pub use weapon_melee::WeaponMelee;

pub trait Entity {
    fn get_name(&self) -> &str;
    fn get_weight(&self) -> i16;

    fn take_damaged(&mut self, damage: u32);
    fn take_fixed(&mut self, point: u8);

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

#[derive(Copy, Clone)]
pub enum Curse {
    Nothing,
    ReverseProbability,
    Weak,
    Unstable
}

#[derive(Copy, Clone)]
pub enum Spec {
    Nothing,
    Damage
}