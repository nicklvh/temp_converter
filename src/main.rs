use std::io;

fn main() {
    println!("Please input your temperature scale: (Celsius, Fahrenheit)");

    let mut temp_scale = String::new();

    io::stdin()
        .read_line(&mut temp_scale)
        .expect("Failed to read line");

    temp_scale = temp_scale.trim().to_lowercase();

    let other_temp_scale;

    let scale = if temp_scale == "celsius" {
        other_temp_scale = "fahrenheit".to_string();
        TemperatureScale::Celsius
    } else if temp_scale == "fahrenheit" {
        other_temp_scale = "celsius".to_string();
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

    println!(
        "From {degrees} degrees in {temp_scale}, to {} in {other_temp_scale}",
        convert(scale, degrees)
    );
}

fn convert(scale: TemperatureScale, degrees: i32) -> i32 {
    match scale {
        TemperatureScale::Celsius => (degrees * 9 / 5) + 32,
        TemperatureScale::Fahrenheit => (degrees - 32) * 5 / 9,
    }
}

enum TemperatureScale {
    Celsius,
    Fahrenheit,
}
