
pub fn duplicate<T: Clone>(a: T) -> (T, T) {
    (a.clone(), a.clone())
}

pub fn duplicate2<T>(a: T) -> (T, T)
where
    T: Clone,
{
    (a.clone(), a.clone())
}