extern crate lua_parser;
extern crate serde_json;

use std::fs::{File, read_dir};
use std::io::{Read, Write};

use lua_parser::{tokenize, parse_from_tokens, Token, ast::Chunk};

#[test]
fn tokenizer() {
    for entry in read_dir("parse_examples/tokenizer_tests").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path().to_str().unwrap().to_string();

        if !path.ends_with(".lua") {
            continue;
        }

        let expected_tokens_path = path.replace(".lua", ".tokens.json");
        let expected_ast_path = path.replace(".lua", ".ast.json");

        let contents = {
            let mut file = File::open(entry.path())
                .expect("Unable to open file!");

            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .expect("Unable to read from file!");

            contents
        };

        let mut expected_token_contents = String::new();
        let expected_tokens = match File::open(&expected_tokens_path) {
            Ok(mut file) => {
                file.read_to_string(&mut expected_token_contents)
                    .expect("Unable to read from file!");

                let result: Vec<Token> = match serde_json::from_str(&expected_token_contents) {
                    Ok(value) => value,
                    Err(error) => {
                        panic!("Unable to deserialize JSON file {}:\n{}", expected_tokens_path, error);
                    }
                };

                Some(result)
            },
            Err(_) => None,
        };

        let mut expected_ast_contents = String::new();
        let expected_ast = match File::open(&expected_ast_path) {
            Ok(mut file) => {
                file.read_to_string(&mut expected_ast_contents)
                    .expect("Unable to read from file!");

                let result: Chunk = match serde_json::from_str(&expected_ast_contents) {
                    Ok(value) => value,
                    Err(error) => {
                        panic!("Unable to deserialize JSON file {}:\n{}", expected_ast_path, error);
                    }
                };

                Some(result)
            },
            Err(_) => None,
        };

        let tokens = match tokenize(&contents) {
            Ok(tokens) => tokens,
            Err(err) => {
                panic!("Failed to tokenize file {}: {:?}", entry.path().display(), err);
            },
        };

        let ast = match parse_from_tokens(&tokens) {
            Ok(ast) => ast,
            Err(err) => {
                panic!("Failed to parse file {}: {:?}", entry.path().display(), err);
            },
        };

        match expected_tokens {
            Some(expected_tokens) => {
                if tokens != expected_tokens {
                    panic!("Received: {:#?}\n\nExpected: {:#?}\n\nFrom expected tokens file {}", tokens, expected_tokens, expected_tokens_path);
                }
            },
            None => {
                println!("Creating expectated tokens file {}", expected_tokens_path);

                let mut file = File::create(&expected_tokens_path)
                    .expect("Unable to create file!");

                let contents = serde_json::to_string_pretty(&tokens).unwrap();

                file.write_all(contents.as_bytes()).unwrap();
            }
        }

        match expected_ast {
            Some(expected_ast) => {
                if ast != expected_ast {
                    panic!("Received: {:#?}\n\nExpected: {:#?}\n\nFrom expected AST file {}", ast, expected_ast, expected_ast_path);
                }
            },
            None => {
                println!("Creating expectated AST file {}", expected_ast_path);

                let mut file = File::create(&expected_ast_path)
                    .expect("Unable to create file!");

                let contents = serde_json::to_string_pretty(&ast).unwrap();

                file.write_all(contents.as_bytes()).unwrap();
            }
        }
    }
}