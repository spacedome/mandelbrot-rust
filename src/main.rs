extern crate num;
extern crate image;

use std::fs::File;
use std::path::Path;

use image::Rgb;

use num::complex::Complex;

fn main() {
    const M: f32 = 2.0;

    let max_n: u16 = 256*2;

    // imgx should be bigger than imgy
    let imgx: u32 = 2560*4;
    let imgy: u32 = 1440*4;

    let xpos = -0.69;
    let ypos = 0.307;
    let zoom = 128.0*8.0;

    let ratio = (imgx as f32) / (imgy as f32);

    let dx = (4.0 / zoom) / imgx as f32;
    let dy = ((4.0 / ratio) / zoom) / imgy as f32;

    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {

        let cx = xpos + x as f32 * dx - 2.0 / zoom;
        let cy = -ypos + y as f32 * dy - (2.0 / ratio) / zoom;
        let c = Complex::new(cx, cy);
        let mut z = Complex::new(0.0, 0.0);
        let mut prev = Complex::new(0.0, 0.0);
        let mut iter = 0;


        // Cardioid and period-2 bulb check, not helpful for zooms?
        let q = (c.re - 0.25) * (c.re - 0.25) + c.im * c.im;
        if q * (q + (c.re - 0.25)) < 0.25 * c.im * c.im { iter = max_n; }
        else if (c.re + 1.0) * (c.re + 1.0) + c.im * c.im < 1.0/16.0 { iter = max_n; }
        else {
            for t in 0..(max_n) {
                if z.norm() > M * M {
                    break
                }
                z = z * z + c;

                // check for periodicity (last iter only)
                if z == prev {
                    iter = max_n;
                    break
                }
                prev = z;
                iter = t;
            }
        }

        // http://stackoverflow.com/questions/369438/smooth-spectrum-for-mandelbrot-set-rendering
        let smooth_iter = if iter >= (max_n - 1) { max_n as f32 }
            else if iter < 2 { 1f32 }
            else { (iter as f32) + 1.0 - z.norm_sqr().ln().ln() / (2.0f32).ln() };

        *pixel = smooth_colormap(smooth_iter, max_n as f32);

    }


    let ref mut fout = File::create(&Path::new("fractal.png")).unwrap();

    // We must indicate the image’s color type and what format to save as
    let _ = image::ImageRgb8(imgbuf).save(fout, image::PNG);
}

fn smooth_colormap(iter: f32, max: f32 ) -> Rgb<u8> {
    let mix = iter/max*1.0;
    let r: u8 = if mix < 1.0/3.0 { (mix * 3.0 * 255.0) as u8 } else { 255u8 };
    let g: u8 = if mix > 2.0/3.0 { 255u8 }
                else if mix > 1.0/3.0 { ((mix - 1.0/3.0)* 3.0 * 255.0) as u8 } 
                else { 0u8 }; 
    let b: u8 = if mix > 1.0 { 255u8 }
                else if mix > 2.0/3.0 { ((mix - 2.0/3.0) * 3.0 * 255.0) as u8 }
                else { 0u8 };
    Rgb([r, g, b])
}
