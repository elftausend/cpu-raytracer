
pub struct Rect<T> {
    pub width: T,
    pub height: T,
    pub x: T,
    pub y: T,
}

impl<T> Rect<T> {
    pub fn new(x: T, y: T, width: T, height: T) -> Self {
        Self {
            x,y,width,height
        }
    }
}