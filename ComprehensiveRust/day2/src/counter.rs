use std::collections::HashMap;

/// Counter counts the number of times each value of type T has been seen.
pub struct Counter<T> {
    values: HashMap<T, u64>
}

impl<T> Counter<T>
where T: std::hash::Hash + std::cmp::Eq
{
    /// Create a new Counter
    pub fn new() -> Self {
        Counter {
            values: HashMap::new(),
        }
    }

    /// Count an occurence of a value.
    pub fn count(&mut self, value: T) {
        if self.values.contains_key(&value) {
            let count = self.values.get_mut(&value).unwrap();
            *count += 1;
        } else {
            self.values.insert(value, 1);
        }
    }

    /// Return the number of times a value has been seen.
    pub fn times_seen(&self, value: T) -> u64 {
        self.values.get(&value).unwrap_or(&0).clone()
    }
}
