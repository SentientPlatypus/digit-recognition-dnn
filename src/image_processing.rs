use image::{self, imageops::*};



pub fn convert_to_grayscale() {
  let img = image::open("num.jpeg").unwrap();
  let mut img = img.grayscale();
  let mut img = img.as_mut_luma8().unwrap();
  dither(&mut img, &BiLevel);
  img.save("num.png").unwrap();
}

pub fn get_grayscale_vector(img: image) -> Vec<i8> {
  *img = img.grayscale();
  let to_return: Vec<i8> = Vec::new();
  for pixel in img.pixels {
    to_return.push(pixel);
  }
  to_return
}
