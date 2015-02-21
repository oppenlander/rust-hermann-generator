use std::old_io::File;

extern crate image;
use image::{ ImageBuffer, ImageRgb8, Rgb, JPEG };

pub fn gen(dest: String, imgx: u32, imgy: u32, inverted: bool, centered: bool) {
    let mut imgbuf = ImageBuffer::new(imgx, imgy);

    let box_size = 35;
    let margin_low = 3;
    let margin_high = box_size - margin_low;

    let outter_pixel = if inverted {
        Rgb([0, 0, 0])
    } else {
        Rgb([255, 255, 255])
    };
    let inner_pixel = if inverted {
        Rgb([255, 255, 255])
    } else {
        Rgb([0, 0, 0])
    };

    let offsetx = if centered {
       ((imgx % box_size) / 2) as u32
    } else {
        0
    };
    let offsety = if centered {
       ((imgy % box_size) / 2) as u32
    } else {
        0
    };

    for y in (0..imgy) {
        let yy = (y + offsetx) % box_size;
        for x in (0..imgx) {
            let xx = (x + offsety) % box_size;

            let pixel = if xx > margin_low && xx < margin_high && yy > margin_low && yy < margin_high {
                inner_pixel
            } else {
                outter_pixel
            };
            imgbuf.put_pixel(x, y, pixel);
        }
    }

    let ref mut fout = File::create(&Path::new(dest)).unwrap();
    
    let _ = ImageRgb8(imgbuf).save(fout, JPEG);
}
