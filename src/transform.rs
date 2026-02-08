pub struct Transform {
    width: u32,
    height: u32,
}

impl Transform {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    pub fn to_screen(&self, x: i32, y: i32) -> Option<(u32, u32)> {
        if y < 0 || x < 0 || x >= self.width as i32 || y >= self.height as i32 {
            None
        } else {
            let sy: u32 = self.height - 1 - y as u32;
            Some((x as u32, sy))
        }
    }
}
