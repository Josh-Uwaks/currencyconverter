use std::io::{self, stdin};
use reqwest::{blocking::Client, StatusCode};
use serde::Deserialize;
use dotenv::dotenv;
use std::env;
use colored::*;


#[derive(Deserialize, Debug)]
struct ConverterResponse {
    result: String,
    time_last_update_utc: String,
    time_next_update_utc: String,
    base_code: String,
    target_code: String,
    conversion_rate: f64
}


fn get_currency(apikey: &str, base_currency: &str, target_currency: &str) -> Result<ConverterResponse, reqwest::Error> {
    let url = format!(
        "https://v6.exchangerate-api.com/v6/{}/pair/{}/{}",
        apikey,
        base_currency.trim().to_uppercase(),
        target_currency.trim().to_uppercase()
    );

    let client = Client::new();
    let response = client.get(&url).send()?.error_for_status()?;

    // Deserialize the response into the `ConverterResponse` struct
    let parsed_response: ConverterResponse = response.json()?;
    Ok(parsed_response)
}


fn main() {
    dotenv().ok();
    println!("{}", "Welcome to Currency converter CLI".bright_white());


    loop {
        println!("kindly provide the base currency");

        let mut base_currency = String::new();
        
        io::stdin()
        .read_line(&mut base_currency)
        .expect("failed to read provided value");


        println!("kindly provide the target currency");

        let mut target_currency = String::new();

        io::stdin()
        .read_line(&mut target_currency)
        .expect("failed to read provided value");

        let api_key = env::var("API_KEY").expect("API key not found");

        match get_currency(&api_key, &base_currency, &target_currency) {
            Ok(response) => {
                println!("{}", "API Response".bright_blue());
                println!("{:?}", response);
            }
            Err(err) => {
                eprintln!("Error: {}", err);
            }
        }


        println!("Do you want to get another converter info? (yes/no):");
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
