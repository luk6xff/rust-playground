
pub fn duplicate<T: Clone>(a: T) -> (T, T) {
    (a.clone(), a.clone())
}

