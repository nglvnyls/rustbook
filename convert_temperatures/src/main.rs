use std::io;

fn main() {
    println!("Convert temperatures CELSIUS/ FARENHEIT");
    println!("");
    println!("Input temperatures without units.");
    println!("App converts into CELSIUS and FARENHEIT at the same time");
    println!("");

    let mut temperature = String::new();

    io::stdin().read_line(&mut temperature)
        .expect("Failed to read Temperature");

    let temperature: f32 = temperature.trim().parse().unwrap();
             

    let celsius = (temperature - 32.) / 1.8;
    let farenheit = (temperature * 1.8) + 32.;

    println!("{} Farenheit = {} Celsius ", temperature, celsius);
    println!("{} Celsius = {} Farenheit ", temperature, farenheit);


}
