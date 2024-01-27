#[derive(Debug)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

#[derive(Debug)]
pub struct Point2<T, U> {
    pub x: T,
    pub y: U,
}


impl<T> Point<T> {
    pub fn coords(&self) -> (&T, &T) {
        (&self.x, &self.y)
    }

    // fn set_x(&mut self, x: T)
}

impl<T,U> Point2<T,U> {
    pub fn coords(&self) -> (&T, &U) {
        (&self.x, &self.y)
    }

    pub fn set_coords(&mut self, x: T, y: U) {
        self.x = x;
        self.y = y;
    }
}


