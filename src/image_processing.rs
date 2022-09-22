use image::{self, imageops::*};



pub fn convert_to_grayscale() {
  let img = image::open("num.jpeg").unwrap();
  let mut img = img.grayscale();
  let mut img = img.as_mut_luma8().unwrap();
  dither(&mut img, &BiLevel);
  img.save("num.png").unwrap();
}

