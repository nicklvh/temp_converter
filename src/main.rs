use std::io;

fn main() {
    println!("Please input your temperature scale: (Celsius, Fahrenheit)");

    let mut temp_scale = String::new();

    io::stdin()
        .read_line(&mut temp_scale)
        .expect("Failed to read line");

    temp_scale = temp_scale.trim().to_lowercase();

    let scale = if temp_scale == "celsius" {
        TemperatureScale::Celsius
    } else if temp_scale == "fahrenheit" {
        TemperatureScale::Fahrenheit
    } else {
        return println!("Please input either Celsius or Fahrenheit!");
    };

    println!("Please input the degrees:");

    let mut degrees = String::new();

    io::stdin()
        .read_line(&mut degrees)
        .expect("Failed to read line");

    let degrees = match degrees.trim().parse::<i32>() {
        Ok(num) => num,
        Err(err) => return println!("{err}"),
    };

    let conversion = TemperatureScale::convert(&scale, degrees);

    println!(
        "From {degrees} degrees in {temp_scale}, to {} degrees in {}",
        conversion.0, conversion.1
    );
}

enum TemperatureScale {
    Celsius,
    Fahrenheit,
}

impl TemperatureScale {
    fn convert(&self, degrees: i32) -> (i32, String) {
        match self {
            TemperatureScale::Celsius => ((degrees * 9 / 5) + 32, String::from("fahrenheit")),
            TemperatureScale::Fahrenheit => ((degrees - 32) * 5 / 9, String::from("celsius")),
        }
    }
}
