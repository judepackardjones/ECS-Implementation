pub trait Component: Sized + Send + Sync + 'static {}


#[derive(Debug, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}
#[derive(Debug, PartialEq)]
pub struct Size {
    pub r: u32, 
}

#[derive(Debug, PartialEq)]
pub struct Player {}


impl Component for Position {}
impl Component for Size {}
impl Component for Player {}