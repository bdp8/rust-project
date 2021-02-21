
extern crate image;
use editor::input::{get_input_info};
use editor::image_proccessing::{Operations};

fn main() {
    let matches = get_input_info();
	let filename = matches.value_of("input").unwrap();
	
	let mut output_file = "output.jpeg";
	if matches.is_present("output")
	{
		output_file = matches.value_of("output").unwrap();
	}
	
	if output_file.is_empty() {
		println!("Output file name is empty. View --help for more information.");
		std::process::exit(1);
    }
	
	let	mut op = match Operations::new(filename) {
		Ok(w) => w, 
		Err(e) => {
			println!("\n{:?}", e);
            std::process::exit(1);
		}
	};
	
	if matches.is_present("thumbnail") 
	{
		let vals: Vec<&str> = matches.values_of("thumbnail").unwrap().collect();

		op.thumbnail(vals[0].parse::<u32>().unwrap(), vals[1].parse::<u32>().unwrap());
	} 
	if matches.is_present("crop") 
	{
		let vals: Vec<&str> = matches.values_of("crop").unwrap().collect();

		op.crop(vals[0].parse::<u32>().unwrap(), vals[1].parse::<u32>().unwrap(), vals[2].parse::<u32>().unwrap(), vals[3].parse::<u32>().unwrap());
	}
	if matches.is_present("grayscale") 
	{
		op.grayscale();
	}
	if matches.is_present("rotate-left") 
	{
		op.rotate(270);
	}
	if matches.is_present("rotate-right") 
	{
		op.rotate(90);
	}
	if matches.is_present("brightness") 
	{
		let value = matches.value_of("brightness").unwrap().parse::<i32>().unwrap();
		op.brightness(value);
	}
	if matches.is_present("contrast") 
	{
		let value = matches.value_of("contrast").unwrap().parse::<f32>().unwrap();
		op.contrast(value);
	}
	if matches.is_present("sharpen") 
	{
		let value = matches.value_of("sharpen").unwrap().parse::<i32>().unwrap();
		op.sharpen(value);
	}
	if matches.is_present("blur") 
	{
		let value = matches.value_of("blur").unwrap().parse::<f32>().unwrap();
		op.blur(value);
	}
	
	if matches.is_present("invert") 
	{
		op.invert();
	}

	if matches.is_present("adjust-r") 
	{
		let value = matches.value_of("adjust-r").unwrap().parse::<i32>().unwrap();
		op.adjust_r(value);
	}
	
	if matches.is_present("adjust-g") 
	{
		let value = matches.value_of("adjust-g").unwrap().parse::<i32>().unwrap();
		op.adjust_g(value);
	}	
	
	if matches.is_present("adjust-b") 
	{
		let value = matches.value_of("adjust-b").unwrap().parse::<i32>().unwrap();
		op.adjust_b(value);
	}	

	if matches.is_present("sobel") 
	{
		op.sobel();
	}	
		
	match op.image.save(output_file) {
		Ok(file) => file,
		Err(e) => { 
			println!("\n{}", e);
            std::process::exit(1);
		},
	};
}
