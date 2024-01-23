#[derive(Debug)]
pub enum Event {
    CarArrived(Floor),
    CarDoorOpened,
    CarDoorClosed,
    ButtonPressed(Button),
}

/// Type representing floor
pub type Floor = i32;

/// A direction for travel
#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
}

/// A user button
#[derive(Debug)]
pub enum Button {
    LobbyCall(Direction, Floor),
    CarFloor(Floor),
}


/// The car has arrived on the given floor
pub fn car_arrived(floor: i32) -> Event {
    Event::CarArrived(floor)
}


/// The car doors have opened
pub fn car_door_opened() -> Event {
    Event::CarDoorOpened
}

/// The car doors have closed
pub fn car_door_closed() -> Event {
    Event::CarDoorClosed
}

/// A directional button was pressed in an elevator lobby on the given floor
pub fn lobby_call_button_pressed(floor: i32, direction: Direction) -> Event {
    Event::ButtonPressed(Button::LobbyCall(direction, floor))
}

/// A floor button was pressed in the elevator car
pub fn car_floor_button_pressed(floor: i32) -> Event {
    Event::ButtonPressed(Button::CarFloor(floor))
}