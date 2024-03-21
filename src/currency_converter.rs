use std::collections::HashMap;
use std::io::{self, BufRead};

use crate::api::{fetch_json, ApiError};
use crate::config::ConfigApi;
use crate::parser::{parse_json, CurrenciesRatesData};

struct CurrenciesToConvert {
    base: String,
    target: String,
    amount: f64,
}

fn fetch_currencies_rate(url: String) -> Result<HashMap<String, f64>, ApiError> {
    let json_body: String = match fetch_json(url) {
        Ok(json) => json,
        Err(error) => return Err(error),
    };

    let rates_data: CurrenciesRatesData = match parse_json(json_body) {
        Ok(data) => data,
        Err(error) => return Err(error),
    };

    Ok(rates_data.data.rate)
}

fn convert_currencies(cur: CurrenciesToConvert, cfg: &impl ConfigApi) {
    let url: String = format!(
        "{}latest?base_currency={}&currencies={}&apikey={}",
        cfg.get_url(),
        cur.base,
        cur.target,
        cfg.get_key()
    );
    match fetch_currencies_rate(url) {
        Ok(currencies_rate) => {
            println!();
            if currencies_rate.len() == 1 {
                println!(
                    "{} {} = {:.2} {}\nRate: {:.2}",
                    cur.amount,
                    cur.base,
                    cur.amount * currencies_rate[&cur.target],
                    cur.target,
                    currencies_rate[&cur.target]
                );
            } else {
                for (currency, rate) in currencies_rate.iter() {
                    println!("{}: {:.2}", currency, rate);
                }
            }
        }
        Err(error) => println!("{}", error),
    };
}

fn read_3_args_user_input() -> Result<CurrenciesToConvert, Box<dyn std::error::Error>> {
    let mut input: String = String::new();
    io::stdin().lock().read_line(&mut input)?;

    input = input.trim().to_string().to_uppercase();

    if input.is_empty() {
        return Err("Empty input".into());
    }

    let input_vec: Vec<String> = input.split_whitespace().map(|s| s.to_string()).collect();

    if input_vec.len() != 3 {
        return Err("Insufficient input".into());
    }

    let amount: f64 = match input_vec[2].parse() {
        Ok(amount) => amount,
        Err(_) => return Err("Invalid amount".into()),
    };

    Ok(CurrenciesToConvert {
        base: input_vec[0].clone(),
        target: input_vec[1].clone(),
        amount: amount,
    })
}

fn read_1_arg_user_input() -> Result<CurrenciesToConvert, Box<dyn std::error::Error>> {
    let mut input: String = String::new();
    io::stdin().lock().read_line(&mut input)?;

    input = input.trim().to_string().to_uppercase();

    if input.is_empty() {
        return Err("Empty input".into());
    }

    let input_vec: Vec<String> = input.split_whitespace().map(|s| s.to_string()).collect();

    if input_vec.len() != 1 {
        return Err("Insufficient input".into());
    }

    Ok(CurrenciesToConvert {
        base: input_vec[0].clone(),
        target: "".to_string(),
        amount: 1.0,
    })
}

fn convert_all_currencies(cfg: &impl ConfigApi) {
    match read_1_arg_user_input() {
        Ok(cur) => convert_currencies(cur, cfg),
        Err(error) => println!("{}", error),
    };
}

fn convert_specific_currencies(cfg: &impl ConfigApi) {
    match read_3_args_user_input() {
        Ok(cur) => convert_currencies(cur, cfg),
        Err(error) => println!("{}", error),
    };
}

fn exit() {
    std::process::exit(0);
}

fn print_menu() {
    println!("\n1) All currencies rates");
    println!("2) Convert currency");
    println!("3) Exit\n");
}

pub fn client_code(cfg: &impl ConfigApi) {
    loop {
        print_menu();
        let mut input: String = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input = input.trim().to_string();

        match input.as_str() {
            "1" => {
                println!("Example of usage: USD");
                println!("Type currency code:");
                convert_all_currencies(cfg);
            }
            "2" => {
                println!("Example of usage: USD EUR 72.34");
                println!("Type base currency, target currency and amount:");
                convert_specific_currencies(cfg);
            }
            "3" => exit(),
            _ => println!("Invalid input"),
        }
    }
}
