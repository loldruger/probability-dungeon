use super::{Entity, Disposable};

struct Disposables {
    name: String,
    image: &'static str,
    shape: &'static str,
    freshness: (u8, u8), //current, total
    weight: i32,
}