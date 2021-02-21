extern crate image;
use image::{GenericImage, GenericImageView, ImageBuffer, DynamicImage, Pixel, Rgba};
use crate::errors::EditorError;

use image::imageops::FilterType::Lanczos3 as filterType;

pub struct Operations
{
	pub image: DynamicImage
}

impl Operations
{
	pub fn new(img_path: &str) -> Result<Self, EditorError>
	{
	    if img_path.is_empty() {
            return Err(EditorError::InvalidFile(String::new()))
        }

		let img = image::open(img_path).unwrap();
		Ok( Operations { image: img})
	}
	
	pub fn thumbnail(&mut self, x:u32, y:u32)
	{
		self.image = self.image.resize(x, y, filterType);
	}
	
	pub fn crop(&mut self, point_x:u32, point_y:u32, sz_x:u32, sz_y:u32)
	{
		let mut imgbuff = ImageBuffer::new(sz_x, sz_y);
		
		for y in 0..sz_y {
			for x in 0..sz_x {
				let px = self.image.get_pixel(point_x + x, point_y + y);
				imgbuff.put_pixel(x, y, px);
			}
		}	

		self.image = DynamicImage::ImageRgba8(imgbuff);
	}
	
	pub fn grayscale(&mut self)
	{
		let (width, height) = self.image.dimensions();

		for x in 0..width {
			for y in 0..height {
				let px = self.image.get_pixel(x, y);

				let res = self.truncate((0.299 * (px[0] as f32) + 0.587 * (px[1] as f32) + 0.114 * (px[2] as f32)) as i32);

				self.image.put_pixel(x, y, Rgba([res as u8, res as u8, res as u8, res as u8]));
			}
		}
	}
	
	pub fn rotate(&mut self, deg:u32)
	{
		if deg == 90 {
			self.image = self.image.rotate90();
		}
		if deg == 270 {
			self.image = self.image.rotate270();
		}
	}
	
	fn truncate(&mut self, value:i32) -> u8
	{
		if value > 255
		{
			return 255;
		} 
		else if value < 0
		{
			return 0;
		}
		else 
		{
			return value as u8;
		}
	}
	
	pub fn brightness(&mut self, value: i32)
	{
		let (width, height) = self.image.dimensions();

		for y in 0..height {
			for x in 0..width {
				let mut px = self.image.get_pixel(x, y);
				px[0] = self.truncate(px[0] as i32 + value);
				px[1] = self.truncate(px[1] as i32 + value);
				px[2] = self.truncate(px[2] as i32 + value);
				
				self.image.put_pixel(x, y, px);
			}
		}	
	}
	
	pub fn contrast(&mut self, value: f32)
	{
		let f :f32 = (259.0 * (value + 255.0)) / (255.0 * (259.0 - value));
		let (width, height) = self.image.dimensions();

		for y in 0..height {
			for x in 0..width {
				let mut px = self.image.get_pixel(x, y);
				px[0] = self.truncate((f * (px[0] as f32 - 128.0) + 128.0) as i32);
				px[1] = self.truncate((f * (px[1] as f32 - 128.0) + 128.0) as i32);
				px[2] = self.truncate((f * (px[2] as f32 - 128.0) + 128.0) as i32);
				
				self.image.put_pixel(x, y, px);
			}
		}
	}
	
	pub fn sharpen(&mut self, value: i32)
	{
		self.image = self.image.unsharpen(10.0, value);
	}
		
	pub fn blur(&mut self, value: f32)
	{
		self.image = self.image.blur(value);
	}
	
	pub fn invert(&mut self)
	{
		let (width, height) = self.image.dimensions();

		for y in 0..height {
			for x in 0..width {
				let mut px = self.image.get_pixel(x, y);
				px[0] = 255 - px[0];
				px[1] = 255 - px[1];
				px[2] = 255 - px[2];
				self.image.put_pixel(x, y, px);
			}
		}
	}

	pub fn adjust_r(&mut self, value:i32)
	{
		let (width, height) = self.image.dimensions();
		
		for y in 0..height {
			for x in 0..width {
				let mut px = self.image.get_pixel(x, y);
				let a:i32 = px[0].into();
				let res = self.truncate(a + a*value/100);

				px[0] = res as u8;
				self.image.put_pixel(x, y, px);
			}
		}
	}	
	
	pub fn adjust_g(&mut self, value:i32)
	{
		let (width, height) = self.image.dimensions();
			
		for y in 0..height {
			for x in 0..width {
				let mut px = self.image.get_pixel(x, y);
				let a:i32 = px[1].into();
				let res = self.truncate(a + a*value/100);
				
				px[1] = res as u8;
				self.image.put_pixel(x, y, px);
			}
		}
	}	
	
	pub fn adjust_b(&mut self, value:i32)
	{
		let (width, height) = self.image.dimensions();
		
		for y in 0..height {
			for x in 0..width {
				let mut px = self.image.get_pixel(x, y);
				let a:i32 = px[2].into();
				let res = self.truncate(a + a*value/100);
	
				px[2] = res as u8;
				self.image.put_pixel(x, y, px);
			}
		}
	}	
	
	pub fn sobel(&mut self)
	{
	    self.image = DynamicImage::ImageLuma8(self.image.to_luma8());
		let width: u32 = self.image.width() - 2;
		let height: u32 = self.image.height() - 2;

		for x in 0..width {
			for y in 0..height {
				let val0 = self.image.get_pixel(x, y)[0] as i32;
				let val1 = self.image.get_pixel(x + 1 , y)[0] as i32;
				let val2 = self.image.get_pixel(x + 2, y)[0] as i32;
				let val3 = self.image.get_pixel(x, y + 1)[0] as i32;
				let val5 = self.image.get_pixel(x + 2, y + 1)[0] as i32;
				let val6 = self.image.get_pixel(x, y + 2)[0] as i32;
				let val7 = self.image.get_pixel(x + 1, y + 2)[0] as i32;
				let val8 = self.image.get_pixel(x + 2, y + 2)[0] as i32;

				let gx = (-1 * val0) + (-2 * val3) + (-1 * val6) + val2 + (2 * val5) + val8;
				let gy = (-1 * val0) + (-2 * val1) + (-1 * val2) + val6 + (2 * val7) + val8;
				let res = self.truncate(((gx as f64).powi(2) + (gy as f64).powi(2)).sqrt() as i32);

				self.image.put_pixel(x, y, Rgba([res as u8, res as u8, res as u8, res as u8]));
			}
		}
	}
}