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
    description: String
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

    Ok(response)

}


fn display_weather_info(response: &WeatherResponse) {
    let description = &response.data[0].weather.description;
    let city_name = &response.data[0].city_name;
    let country_code = &response.data[0].country_code;
    let temp = response.data[0].temp;
    let timezone = &response.data[0].timezone;
    let datetime = &response.data[0].datetime;

    let weather_text = format!(
        "description of weather: {}, {},
        > Temperature: {:.2}°C, 
        > country code: {}, 
        > city name: {}, 
        > timezone: {},
        > datetime: {}",
        description,
        get_temperature_emoji(temp),
        temp,
        country_code,
        city_name,
        timezone,
        datetime
        );


        // Coloring the weather text based on weather conditions
    let weather_text_colored = match description.as_str() {
        "clear sky" => weather_text.bright_yellow(),
        "few clouds" | "scattered clouds" | "broken clouds" => weather_text.bright_blue(),
        "overcast clouds" | "mist" | "haze" | "smoke" | "sand" | "dust" | "fog" | "squalls" => weather_text.dimmed(),
        "shower rain" | "rain" | "thunderstorm" | "snow" => weather_text.bright_cyan(),
        _ => weather_text.normal(),
    };

    println!("{}", weather_text_colored);

}

fn get_temperature_emoji(temperature: f64) -> &'static str {
    if temperature < 0.0 {
        "❄️"
    } else if temperature >= 0.0 && temperature < 10.0 {
        "☁️"
    } else if temperature >= 10.0 && temperature < 20.0 {
        "⛅"
    } else if temperature >= 20.0 && temperature < 30.0 {
        "🌤️"
    } else {
        "🔥"
    }
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
                display_weather_info(&response);
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
