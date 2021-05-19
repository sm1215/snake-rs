use direction::Direction;

pub enum Command {
    Quit,
    Turn(Direction),
}
