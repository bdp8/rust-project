use editor::image_proccessing::Operations;

use image::{GenericImageView};

/*
fn testing_photo()
{
	let mut imgbuff = ImageBuffer::new(255,255);
	
	for y in 0..255 {
			for x in 0..255 {
				imgbuff.put_pixel(x, y, Rgba([x as u8,x as u8,x as u8 ,0 as u8]));
			}
	}	
	let img = DynamicImage::ImageRgba8(imgbuff);
	img.save("test.jpeg").unwrap();
}
*/
fn truncate(value:i32) -> u8
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
	
#[test]
fn crop_new_dimensions() {
	let	mut op = Operations::new("test.jpeg").unwrap();
	op.crop(10,10,100,100);
	let (width, height) = op.image.dimensions();
	
	assert_eq!(width, 100);
	assert_eq!(height, 100);
}

#[test]
fn crop_new_pixels() {
	let img = image::open("test1.jpeg").unwrap();
	let	mut op = Operations::new("test.jpeg").unwrap();
	
	op.crop(10,10,100,100);

	for y in 0..100 {
		for x in 0..100 {
			let px_orig = img.get_pixel(10 + x, 10 + y);
			let px_new = op.image.get_pixel(x, y);
			
			assert_eq!(px_orig, px_new);
		}
	}	
}

#[test]
fn test_grayscale() {
	let img = image::open("test1.jpeg").unwrap();
	let	mut op = Operations::new("test.jpeg").unwrap();
	
	op.grayscale();

	let (width, height) = op.image.dimensions();

	for x in 0..width {
		for y in 0..height {
			let px_new = op.image.get_pixel(x, y);
			let px_orig = img.get_pixel(x, y);
			let res = truncate((0.299 * (px_orig[0] as f32) + 0.587 * (px_orig[1] as f32) + 0.114 * (px_orig[2] as f32)) as i32);
			
			assert_eq!(res as u8, px_new[0]);
			assert_eq!(res as u8, px_new[1]);			
			assert_eq!(res as u8, px_new[2]);
		}
	}
}

#[test]
fn test_brightness_positive() {
	let img = image::open("test1.jpeg").unwrap();
	let	mut op = Operations::new("test.jpeg").unwrap();
	
	op.brightness(50);

	let (width, height) = op.image.dimensions();

	for x in 0..width {
		for y in 0..height {
			let px_new = op.image.get_pixel(x, y);
			let px_orig = img.get_pixel(x, y);
			
			let r = truncate(px_orig[0] as i32 + 50) as u8;
			let g = truncate(px_orig[1] as i32 + 50) as u8;
			let b = truncate(px_orig[2] as i32 + 50) as u8;

			assert_eq!(r, px_new[0]);
			assert_eq!(g, px_new[1]);			
			assert_eq!(b, px_new[2]);
		}
	}
}

#[test]
fn test_brightness_negative() {
	let img = image::open("test1.jpeg").unwrap();
	let	mut op = Operations::new("test.jpeg").unwrap();
	
	op.brightness(-50);

	let (width, height) = op.image.dimensions();

	for x in 0..width {
		for y in 0..height {
			let px_new = op.image.get_pixel(x, y);
			let px_orig = img.get_pixel(x, y);
			
			let r = truncate(px_orig[0] as i32 - 50) as u8;
			let g = truncate(px_orig[1] as i32 - 50) as u8;
			let b = truncate(px_orig[2] as i32 - 50) as u8;

			assert_eq!(r, px_new[0]);
			assert_eq!(g, px_new[1]);			
			assert_eq!(b, px_new[2]);
		}
	}
}

#[test]
fn test_contrast_positive() {
	let img = image::open("test1.jpeg").unwrap();
	let	mut op = Operations::new("test.jpeg").unwrap();
	
	op.contrast(50.0);

	let f :f32 = (259.0 * (50.0 + 255.0)) / (255.0 * (259.0 - 50.0));

	let (width, height) = op.image.dimensions();

	for x in 0..width {
		for y in 0..height {
			let px_new = op.image.get_pixel(x, y);
			let px_orig = img.get_pixel(x, y);
			
			let r = truncate((f * (px_orig[0] as f32 - 128.0) + 128.0) as i32) as u8;
			let g = truncate((f * (px_orig[1] as f32 - 128.0) + 128.0) as i32) as u8;
			let b = truncate((f * (px_orig[2] as f32 - 128.0) + 128.0) as i32) as u8;
			
			assert_eq!(r, px_new[0]);
			assert_eq!(g, px_new[1]);			
			assert_eq!(b, px_new[2]);
		}
	}
}

#[test]
fn test_invert_colors() {
	let img = image::open("test1.jpeg").unwrap();
	let	mut op = Operations::new("test.jpeg").unwrap();
	
	op.invert();

	let (width, height) = op.image.dimensions();

	for x in 0..width {
		for y in 0..height {
			let px_new = op.image.get_pixel(x, y);
			let mut px_orig = img.get_pixel(x, y);
			px_orig[0] = 255 - px_orig[0];
			px_orig[1] = 255 - px_orig[1];
			px_orig[2] = 255 - px_orig[2];
			assert_eq!(px_new, px_orig);
		}
	}
}

#[test]
fn test_adjust_red() {
	let img = image::open("test1.jpeg").unwrap();
	let	mut op = Operations::new("test.jpeg").unwrap();
	
	op.adjust_r(20);

	let (width, height) = op.image.dimensions();

	for x in 0..width {
		for y in 0..height {
			let px_new = op.image.get_pixel(x, y);
			let px_orig = img.get_pixel(x, y);

			let a:i32 = px_orig[0].into();
			let res = truncate(a + a*20/100);
	
			assert_eq!(px_new[0], res as u8);
		}
	}
}

#[test]
fn test_adjust_green() {
	let img = image::open("test1.jpeg").unwrap();
	let	mut op = Operations::new("test.jpeg").unwrap();
	
	op.adjust_g(20);

	let (width, height) = op.image.dimensions();

	for x in 0..width {
		for y in 0..height {
			let px_new = op.image.get_pixel(x, y);
			let px_orig = img.get_pixel(x, y);

			let a:i32 = px_orig[1].into();
			let res = truncate(a + a*20/100);
	
			assert_eq!(px_new[1], res as u8);
		}
	}
}

#[test]
fn test_adjust_blue() {
	let img = image::open("test1.jpeg").unwrap();
	let	mut op = Operations::new("test.jpeg").unwrap();
	
	op.adjust_b(20);

	let (width, height) = op.image.dimensions();

	for x in 0..width {
		for y in 0..height {
			let px_new = op.image.get_pixel(x, y);
			let px_orig = img.get_pixel(x, y);

			let a:i32 = px_orig[2].into();
			let res = truncate(a + a*20/100);
	
			assert_eq!(px_new[2], res as u8);
		}
	}
}