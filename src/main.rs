pub mod raster;
pub mod tga;

use raster::Raster;
use raster::Point;
use tga::TGA;

use std::fs::write;

fn main() -> std::io::Result<()> {
  let mut image: Raster = Raster::new(16, 16);
  image.set(Point {x: 5, y: 5}, 0x070E);

  let mut file = write("test.tga", TGA::from(image).pack())?;
  Ok(())
}
