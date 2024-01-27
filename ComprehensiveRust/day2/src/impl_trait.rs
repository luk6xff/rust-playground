pub fn add_42_millions(x: impl Into<i32>) -> i32 {
    x.into() + 42_000_000
}

pub fn pair_of(x:u32) -> impl std::fmt::Debug {
    (x + 1, x - 1)
}