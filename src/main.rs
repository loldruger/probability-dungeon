mod game_system;
use game_system::item;

use std::mem;

fn main() {
    let a = item::WeaponMelee::new(
        "지옥참마도".to_string(),
        "<-------------|--",
        "지옥에서 기어나온 지상최강의 검일 것이다. 확신할 수는 없따.",
        10,
        10,
        (10, 10),
        item::State::Normal
    );

    println!("{}", mem::size_of_val(&a));

    let asdf: (item::State, [item::Curse; 4], [item::Spec; 7]) = (
        item::State::Normal,
        [item::Curse::Nothing; 4],
        [item::Spec::Nothing; 7]
    );

    println!("{}", mem::size_of_val(&asdf));
}
