#[derive(Debug)]
struct Race {
    name String,
    laps: Vec<i32>,
}

impl Race {
    fn new(name: &str) -> Race { // No receiver, a static method, this becomes a static method on the struct. Typically used to create constructors which are called new by convention.
        Race { name: String::from(name), laps: Vec<i32>::new() }
    }

    fn add_lap(&mut self, lap i32) { // Exclusive borrowed read-write access to self, borrows the object from the caller using a unique and mutable reference. The object can be used again afterwards.
         self.laps.push(lap);
    }

    fn print_laps(&self) { // Shared and read-only borrowed access to self, borrows the object from the caller using a shared and immutable reference. The object can be used again afterwards.
        println!("Recorded {} laps for the race {}:", self.laps.len(), self.name);
        for (idx, lap) in self.laps.iter().enumerate() {
            println!("Lap nr: {idx}, {lap} sec");
        }
    }

    fn finish(self) { // Exclusive ownership of self, takes ownership of the object and moves it away from the caller. The method becomes the owner of the object. The object will be dropped (deallocated) when the method returns, unless its ownership is explicitly transmitted.
        let total = self.laps.iter().sum::<i32>();
        println!("Race {} is finished, total lap time: {}", self.name, total);
    }
}