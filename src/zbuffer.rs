pub struct ZBuffer {
    pub width: u32,
    pub height: u32,
    pub data: Vec<f64>,
}

impl ZBuffer {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            data: vec![f64::INFINITY; (width * height) as usize],
        }
    }

    #[inline]
    pub fn index(&self, x: u32, y: u32) -> usize {
        (y * self.width + x) as usize
    }
}
