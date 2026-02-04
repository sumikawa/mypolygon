pub struct Transform {
    height: i32,
}

impl Transform {
    pub fn new(height: i32) -> Transform {
        Transform { height }
    }

    pub fn to_screen(&self, x: i32, y: i32) -> (i32, i32) {
        (x, self.height - 1 - y)
    }
}
