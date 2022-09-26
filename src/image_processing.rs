use image::{self, imageops::*, Pixel, Pixels, Luma};



pub fn convert_to_grayscale() -> Vec<&'static Luma<u8>> {
  let img = match image::open("num.jpeg")
  {
    Ok(t) => {
      t
    }
    Err(_) => {
      panic!("Failed in opening");
    }
  };
  let gray_img = img.to_luma8();

  let mut pixel_vector = Vec::new();
  gray_img.pixels().for_each(|pixel| {
    pixel_vector.push(pixel);
  });
  pixel_vector
}




