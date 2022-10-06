use std::{
    io::{BufRead, BufReader},
    net::TcpStream,
};

use rouille::Response;

use crate::password_generator::generate;

#[derive(Serialize)]
struct JSONResponse {
    success: bool,
    password: String,
}

pub fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let mut _include_capitals = true;
    let mut _include_letters = true;
    let mut _include_symbols = true;
    let mut _includes_numbers = true;
    let mut _length = 16_u8;
    let mut _not_allowed_chars: Vec<char> = Vec::new();

    let params: Vec<&str> = http_request[0].split(" ").collect();
    // println!("{:?}", params[1]);

    let params: Vec<&str> = params[1].split("?").collect();
    if params.len() > 1 {
        let qparams: Vec<&str> = params[1].split(",").collect();

        for param in qparams {
            let key_value: Vec<&str> = param.split("=").collect();

            match key_value[0] {
                "include_numbers" => {
                    if key_value[1].eq("true") {
                        _includes_numbers = true
                    } else {
                        _includes_numbers = false
                    }
                }
                "include_symbols" => {
                    if key_value[1].eq("true") {
                        _include_symbols = true
                    } else {
                        _include_symbols = false
                    }
                }
                "include_capitals" => {
                    if key_value[1].eq("true") {
                        _include_capitals = true
                    } else {
                        _include_capitals = false
                    }
                }
                "include_letters" => {
                    if key_value[1].eq("true") {
                        _include_letters = true
                    } else {
                        _include_letters = false
                    }
                }
                "length" => _length = key_value[1].parse::<u8>().unwrap(),
                _ => println!("anan"),
            }
        }
    }

    let pass = generate(
        Some(_length),
        Some(_includes_numbers),
        Some(_include_letters),
        Some(_include_capitals),
        Some(_include_symbols),
        None,
    );
    println!("{}", pass.unwrap());
    //println!("Request: {:#?}", http_request.);
    Response::Response::json(&JSONResponse {
        success: true,
        password: pass.to_owned(),
    });
}
