use image::{self, imageops::*};



pub fn convert_to_grayscale() {
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
  gray_img.save("gray.jpeg").unwrap();
}




