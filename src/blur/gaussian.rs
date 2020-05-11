use ::image::{ImageBuffer, RgbImage, DynamicImage};
use std::f64::consts::{E, PI};

pub fn gaussian_blur(img: DynamicImage, filter_dim: u32, sigma: f64, output: &str){
    let rgbimg: RgbImage = img.into_rgb();
    let mut filter = vec!();
    let mut sum: f64 = 0.0;
    for i in 0..filter_dim {
        let mut filter_row = vec!();
        for j in 0..filter_dim {
            let val = E.powf(-(i as f64 * i as f64 + j as f64 * j as f64)/(2.0 * sigma * sigma)) / (2.0 * PI * sigma * sigma);
            filter_row.push(val);
            sum += val;
        }
        filter.push(filter_row);
    }
    for i in 0..filter_dim {
        for j in 0..filter_dim {
            filter[i as usize][j as usize] /= sum;
        }
    }
    let mut blurred_img: RgbImage = ImageBuffer::new(rgbimg.dimensions().0, rgbimg.dimensions().1);
    
    let mut ctr = 0;
    //For each row
    for i in 0..(rgbimg.dimensions().0) {
        //For each col
        for j in 0..(rgbimg.dimensions().1) {
            let mut val = vec![0 as u32; 3];
            ctr += 1;
            print!("\r{:.2}%", (ctr as f64 / (rgbimg.dimensions().0 * rgbimg.dimensions().1) as f64 * 100.0));
            
            //For filter dimension in row
            for k in (-(filter_dim as i32) / 2)..=(filter_dim as i32 / 2) {
                //For filter dimension in each col
                for l in (-(filter_dim as i32) / 2)..=(filter_dim as i32 / 2) {
                    if (i as i32 + k) < 0 || (i as i32 + k) >= rgbimg.dimensions().0 as i32 || j as i32 + l < 0 || (j as i32 + l)  >= rgbimg.dimensions().1 as i32 {

                        let pixel = rgbimg[(i as u32, j as u32)].0;
                        //For each pixel channel (RGB)
                        for m in 0..=2 {
                            val[m as usize] += (pixel[m as usize] as f64 * filter[(k + (filter_dim as i32) / 2) as usize][(l + (filter_dim as i32) / 2) as usize]) as u32;
                        }
                    
                        continue;
                    }

                    let pixel = rgbimg[((i as i32 + k) as u32, (j as i32 + l) as u32)].0;
                    //For each pixel channel (RGB)
                    for m in 0..=2 {
                        val[m as usize] += (pixel[m as usize] as f64 * filter[(k + (filter_dim as i32) / 2) as usize][(l + (filter_dim as i32) / 2) as usize]) as u32;
                    }
                }   
            }
            let mut temp_val: Vec<u8> = vec!();
            
            for n in 0..=2 {
                temp_val.push(val[n] as u8);
            }
            let p = image::Pixel::from_channels(temp_val[0], temp_val[1], temp_val[2], 1);
            blurred_img.put_pixel(i, j, p);
        }
    }
    println!("");
    blurred_img.save(output).unwrap();
}