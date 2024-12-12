
pub struct Raster {
    pub height: u16,
    pub width: u16,
    pub raw: Vec<u16>,
}

pub struct Point {
    pub x: u16,
    pub y: u16,
}

impl Raster {
    pub fn new(height: u16, width: u16) -> Self {
        let mut raw = vec![0xffff; (height * width) as usize];
        Raster {
            height: height,
            width: width,
            raw: raw
        }
    }

    pub fn set(&mut self, point: Point, to: u16) -> bool {

        if point.x >= self.width {
            return false;
        }
        if point.y >= self.height {
            return false;
        }

        let idx = self.width * point.y + point.x;

        self.raw[idx as usize] = to;
        return true;
    }
}