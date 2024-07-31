use utf8parse::{Parser, Receiver};
use polars::prelude::*;

struct Utf8Receiver {
    result: String,
    error: bool,
}

impl Utf8Receiver {
    fn new() -> Self {
        Self {
            result: String::new(),
            error: false,
        }
    }
}

impl Receiver for Utf8Receiver {
    fn codepoint(&mut self, c: char) {
        self.result.push(c);
    }

    fn invalid_sequence(&mut self) {
        self.error = true;
    }
}

pub fn parse_utf8(value: &[u8]) -> Result<String, &'static str> {
    let mut receiver = Utf8Receiver::new();
    let mut parser = Parser::new();

    for byte in value {
        parser.advance(&mut receiver, *byte);
        if receiver.error {
            return Err("Invalid UTF-8 sequence");
        }
    }

    Ok(receiver.result)
}

// Função para obter o valor UTF-8 de uma coluna Polars
pub fn get_utf8_value(column: &Series, index: usize) -> String {
    if let Ok(value) = column.str_value(index){
        match parse_utf8(value.as_bytes()) {
            Ok(parsed_value) => parsed_value,
            Err(_) => String::new(),
        }
    } else {
        String::new()
    }
}