use framebuffer::{Framebuffer, KdMode};

fn main() {
    println!("Hello from Rust on PinePhone!");

    let mut fb = Framebuffer::new("/dev/fb0").unwrap();

    let w = fb.var_screen_info.xres;
    let h = fb.var_screen_info.yres;
    let ll = fb.fix_screen_info.line_length;
    let bpp = fb.var_screen_info.bits_per_pixel;

    println!("xres is {}", w);
    println!("yres is {}", h);
    println!("line length is {}", ll);
    println!("bits per pixel is {}", bpp);

    let mut frame = vec![0u8; (ll * h) as usize];

    println!("Setting byte 1...");
    for (idx, pixel) in frame.iter_mut().enumerate() {
        if (idx % 4 == 1) {
            *pixel = 255u8;
        } else {
            *pixel = 0u8;
        }
    }

    let _ = fb.write_frame(&frame);

    println!("Setting byte 0...");
    for (idx, pixel) in frame.iter_mut().enumerate() {
        if (idx % 4 == 0) {
            *pixel = 255u8;
        } else {
            *pixel = 0u8;
        }
    }

    let _ = fb.write_frame(&frame);

    println!("Setting byte 2...");
    for (idx, pixel) in frame.iter_mut().enumerate() {
        if (idx % 4 == 2) {
            *pixel = 255u8;
        } else {
            *pixel = 0u8;
        }
    }
    
    let _ = fb.write_frame(&frame);

    println!("Setting byte 3...");
    for (idx, pixel) in frame.iter_mut().enumerate() {
        if (idx % 4 == 3) {
            *pixel = 255u8;
        } else {
            *pixel = 0u8;
        }
    }
    
    let _ = fb.write_frame(&frame);

}
