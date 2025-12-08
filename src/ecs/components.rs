pub trait Component: Sized + Send + Sync + 'static {}


#[derive(Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

pub struct Size {
    pub r: u32, 
}

impl Component for Position {}
impl Component for Size {}