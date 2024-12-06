use std::io;
use serde::Deserialize;
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue};
use dotenv::dotenv;
use std::env;
use colored::*;

#[derive(Deserialize, Debug)]
struct WeatherResponse {
    data: Vec<Data>
}

#[derive(Deserialize, Debug)]
struct Data {
    city_name: String,
    country_code: String,
    datetime: String,
    timezone: String,
    weather: Weather,
    temp: f64,
}


#[derive(Deserialize, Debug)]
struct Weather {
    description: String,
    icon: String,
    code: u128
}

fn get_weather_info(longitude: &f64, latitude: &f64, api_key: &str) -> Result<WeatherResponse, reqwest::Error> {
    
    let url = format!(
        "https://weatherbit-v1-mashape.p.rapidapi.com/current?lon={}&lat={}&units=metric&lang=en",
        longitude, latitude
    );

    let client = Client::new();

    let mut headers = HeaderMap::new();
    headers.insert("x-rapidapi-key", HeaderValue::from_str(api_key).unwrap());
    headers.insert("x-rapidapi-host", HeaderValue::from_static("weatherbit-v1-mashape.p.rapidapi.com"));


     let response = client
     .get(&url)
     .headers(headers)
     .send()?
     .json::<WeatherResponse>()?;

    println!("Raw Response: {:?}", response);

    Ok(response)

}


fn main() {
    dotenv().ok();
    println!("{}", "Welcome to our Weather information application - Joshua Uwakwe".bright_red());

    loop {
        println!("{}", "there are basically 2 informations i need from you, longitude and latitude 😤".bright_green());
        println!("{}", "first kindly provide the longitude".bright_white());
        let mut longitude_input = String::new();
        io::stdin()
        .read_line(&mut longitude_input)
        .expect("Failed to read input");

        let longitude: f64 = match longitude_input.trim().parse() {
            Ok(value) => value,
            Err(_) => {
                eprintln!("Invalid input, please enter a valid floating point number");
                continue;
            }
        };

        println!("{}", "kindly provide the latitude".bright_white());
        let mut latitude_input = String::new();
        io::stdin()
        .read_line(&mut latitude_input)
        .expect("Failed to read input");

        let latitude: f64 = match latitude_input.trim().parse() {
            Ok(value) => value,
            Err(_) => {
                eprintln!("Invalid input, please enter a valid floating point number");
                continue;
            }
        };

        let api_key = env::var("API_KEY").expect("API key not found");

        match get_weather_info(&longitude, &latitude, &api_key) {
            Ok(response) => {
                println!("{}", "API Response".bright_yellow());
                for data in response.data {
                    println!(
                        "City: {}, 
                        Temperature: {:.2}°C, 
                        country_code: {}, 
                        datetime: {}, 
                        timezone: {},
                        weather: {}
                        ",
                        data.city_name, data.temp, data.country_code, data.datetime, data.timezone, data.weather.description
                    )
                }
            }
            Err(err) => {
                eprintln!("Error: {}", err);
            }
        }

        println!("Do you want to enter another set of coordinates? (yes/no):");
        let mut again = String::new();
        io::stdin()
            .read_line(&mut again)
            .expect("Failed to read input");
        if again.trim().to_lowercase() != "yes" {
            println!("Thanks for using our Software! Goodbye!");
            break;
        }

    }
}
