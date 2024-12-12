use embedded_graphics::image;

use crate::raster::{Raster, Point};
use std::convert::From;

pub struct TGA {
  id_length: u8,
  color_map_type: u8,
  image_type: u8,
  color_map_spec: ColorMapSpec,
  image_spec: ImageSpec,
  image_id: Vec<u8>,
  color_map: Vec<u8>,
  image_data: Vec<u16>
}

struct ColorMapSpec {
  first_entry_index: u16,
  color_map_length: u16,
  color_map_size: u8
}

struct ImageSpec {
  x_origin: u16,
  y_origin: u16,
  image_width: u16,
  image_height: u16,
  pixel_depth: u8,
  image_descriptor: u8
}

impl TGA {
  pub fn pack(&self) -> Vec<u8> {
    let mut header: [u8; 18] = [0; 18];
    header[0] =  self.id_length.to_le();
    header[1] =  self.color_map_type.to_le();
    header[2] =  self.image_type.to_le();
    header[3] =  self.color_map_spec.first_entry_index.to_le_bytes()[0];
    header[4] =  self.color_map_spec.first_entry_index.to_le_bytes()[1];
    header[5] =  self.color_map_spec.color_map_length.to_le_bytes()[0];
    header[6] =  self.color_map_spec.color_map_length.to_le_bytes()[1];
    header[7] =  self.color_map_spec.color_map_size.to_le();
    header[8] =  self.image_spec.x_origin.to_le_bytes()[0];
    header[9] = self.image_spec.x_origin.to_le_bytes()[1];
    header[10] =  self.image_spec.y_origin.to_le_bytes()[0];
    header[11] = self.image_spec.y_origin.to_le_bytes()[1];
    header[12] = self.image_spec.image_width.to_le_bytes()[0];
    header[13] = self.image_spec.image_width.to_le_bytes()[1];
    header[14] = self.image_spec.image_height.to_le_bytes()[0];
    header[15] = self.image_spec.image_height.to_le_bytes()[1];
    header[16] = self.image_spec.pixel_depth.to_le();
    header[17] = self.image_spec.image_descriptor.to_le();

    let mut u8_image_data: Vec<u8> = Vec::new();
    for n in &self.image_data {
      u8_image_data.extend(n.to_le_bytes());
    }

    let mut tga_bytes = header.to_vec();
    tga_bytes.append(&mut u8_image_data);

    return tga_bytes;
  }
}

impl From<Raster> for TGA {
  fn from(raster: Raster) -> Self {

    let id_length = 0;
    let color_map_type = 0;
    let image_type = 2;
    let color_map_spec = ColorMapSpec {
      first_entry_index: 0,
      color_map_length: 0,
      color_map_size: 0
    };
    let image_spec = ImageSpec {
      x_origin: 0,
      y_origin: 0,
      image_width: raster.width,
      image_height: raster.height,
      pixel_depth: 16,
      image_descriptor: 0b00000001
    };

    let image_data = raster.raw;

    return TGA {
      id_length: id_length,
      color_map_type: color_map_type,
      image_type: image_type,
      color_map_spec: color_map_spec,
      image_spec: image_spec,
      image_id: [].to_vec(),
      color_map: [].to_vec(),
      image_data: image_data
    };
  }
}

#[cfg(test)]
mod tests {
    use crate::tga::*;

    #[test]
    fn it_works() {
        let raster: Raster = Raster::new(16, 16);
        let tga: TGA = TGA::from(raster);
        let x: u8 = 16;
        println!("{:#02x?}", x.to_le_bytes());
        println!("{:#02x?}", tga.pack());
        println!("{:#02x?}", 0xffff_u16.to_le_bytes());
    }
}