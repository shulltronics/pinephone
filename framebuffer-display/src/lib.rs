use embedded_graphics::{
    prelude::*,
    pixelcolor::Bgr888,
    geometry::{OriginDimensions, Size},
    draw_target::DrawTarget,
};

use linuxfb::Framebuffer;
use memmap;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Orientation {
    PORTRAIT,
    LANDSCAPE,
}

pub struct FramebufferDisplay {
    pixels: memmap::MmapMut,
    width: u32,
    height: u32,
    bpp: u32,
    orientation: Orientation,
}

impl FramebufferDisplay {

    pub fn new() -> FramebufferDisplay {
        let fb_raw = Framebuffer::new("/dev/fb0").unwrap();
        let buffer = fb_raw.map().unwrap();
        let (w, h) = fb_raw.get_size();
        let bpp = fb_raw.get_bytes_per_pixel();
        Self {
            pixels: buffer,
            width: w,
            height: h,
            bpp: bpp,
            orientation: Orientation::LANDSCAPE,
        }
    }

    pub fn get_orientation(&self) -> Orientation {
        return self.orientation;
    }

    pub fn set_orientation(&mut self, o: Orientation) {
        self.orientation = o;
    }

    pub fn get_size(&self) -> (u32, u32) {
        if self.orientation == Orientation::PORTRAIT {
            return (self.width, self.height);
        } else {
            return (self.height, self.width);
        }
    }

    pub fn get_bytes_per_pixel(&self) -> u32 {
        return self.bpp;
    }

    pub fn clear(&mut self) {
        for i in 0..self.pixels.len() {
            self.pixels[i] = 0u8;
        }
    }

}

impl DrawTarget for FramebufferDisplay {
    type Color = Bgr888;
    type Error = ();

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(coord, color) in pixels.into_iter() {
            let (x, y) = (coord.x as u32, coord.y as u32);
            // constrain pixels to screen area
            if x < 0 || x > self.width-1 || y < 0 || y > self.height-1 {
                continue;
            }
            let idx: usize;
            if self.orientation == Orientation::PORTRAIT {
                idx = (x + y*self.width) as usize;
            } else {
                idx = ((self.width - 1 - y) + x*self.width) as usize;
            }
            // we don't multiply the second term by bits-per-pixels because we cast the buffer here
            let (_prefix, pixels, _suffix) = unsafe { self.pixels.align_to_mut::<u32>() };
            pixels[idx] = (color.into_storage()) as u32;
        }

        return Ok(());
    }

}

impl OriginDimensions for FramebufferDisplay {
    fn size(&self) -> Size {
        return Size::new(self.width, self.height);
    }
}
