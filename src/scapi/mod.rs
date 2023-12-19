use std::io::{Read, Write};
use std::net::TcpStream;
use std::process::exit;
use std::string::ToString;
use chrono::prelude::*;
use serde_json::Value;

mod addons;

use crate::utilities;
use crate::colors::*;


const VERSION: &str = "1.0.0";
const FULL_VERSION: &str = "_dev-vacakes-stblib::rs_stmbv2";

pub struct Bot {
    pub username: String,
    pub token: String,
    pub address: String,
    pub port: u16,
    pub stream: TcpStream,
    pub send_stream: TcpStream,
    pub enable_user_input: bool,
    pub log_recv_msg: bool,
    pub log_msg: String,
    pub json_fmt: bool,
}

pub struct Command<'a> {
    pub name: &'a str,
    pub handler: fn(Vec<String>) -> Vec<String>,
}

pub enum LogLevel {
    INFO = 1,
    ERROR = 2,
    MESSAGE = 3,
    WARNING = 4,
}

impl Bot {
    pub fn new(username: &str, token: &str, address: &str, port: u16, json_fmt: bool) -> Bot {
        pub fn connect(address: &str, port: u16) -> TcpStream {
            let host = format!("{}:{}", address, port);

            TcpStream::connect(host).expect("Error opening stream")
        }

        Bot {
            username: username.to_string(),
            token: token.to_string(),
            address: address.to_string(),
            port: port,
            stream: connect(address, port),
            send_stream: connect(address, port),
            enable_user_input: false,
            log_recv_msg: false,
            log_msg: format!("{}{}{}  {}scapi  -->  {}{}", CYAN, BOLD, "", "", C_RESET, ""),
            json_fmt: json_fmt,
        }
    }

    fn current_time(&self, ) -> String {
        let local: DateTime<Local> = Local::now();
        local.format("%Y-%m-%d %H:%M:%S").to_string()
    }

    pub fn flag_handler(&mut self, enable_user_input: bool, log_recv_msg: bool) {
        self.enable_user_input = enable_user_input;
        self.log_recv_msg = log_recv_msg;
    }

    pub fn logger(&mut self, message: &str, log_type: LogLevel) {
        match log_type {
            LogLevel::INFO => {
                self.log_msg = format!("{}{}{}  {}scapi  -->  {}{}", CYAN, BOLD, self.current_time(), format!("{}INFO    ", BLUE), C_RESET, message);
                println!("{}", self.log_msg)
            },
            LogLevel::ERROR => {
                self.log_msg = format!("{}{}{}  {}scapi  -->  {}{}", CYAN, BOLD, self.current_time(), format!("{}ERROR   ", RED), C_RESET, message);
                println!("{}", self.log_msg)
            },
            LogLevel::MESSAGE => {
                self.log_msg = format!("{}{}{}  {}scapi  -->  {}{}", CYAN, BOLD, self.current_time(), format!("{}MESSAGE ", GREEN), C_RESET, message);
                println!("{}", self.log_msg)
            },
            LogLevel::WARNING => {
                self.log_msg = format!("{}{}{}  {}scapi  -->  {}{}", CYAN, BOLD, self.current_time(), format!("{}WARNING ", YELLOW), C_RESET, message);
                println!("{}", self.log_msg)
            }
        }
    }

    pub fn log_fmt(&mut self, message: &str, log_type: LogLevel) -> String {
        match log_type {
            LogLevel::INFO => {
                let log_msg = format!("{}{}{}  {}scapi  -->  {}{}", CYAN, BOLD, self.current_time(), format!("{}INFO    ", BLUE), C_RESET, message);
                log_msg
            },
            LogLevel::ERROR => {
                let log_msg = format!("{}{}{}  {}scapi  -->  {}{}", CYAN, BOLD, self.current_time(), format!("{}ERROR   ", RED), C_RESET, message);
                log_msg
            },
            LogLevel::MESSAGE => {
                let log_msg = format!("{}{}{}  {}scapi  -->  {}{}", CYAN, BOLD, self.current_time(), format!("{}MESSAGE ", GREEN), C_RESET, message);
                log_msg
            },
            LogLevel::WARNING => {
                let log_msg = format!("{}{}{}  {}scapi  -->  {}{}", CYAN, BOLD, self.current_time(), format!("{}WARNING ", YELLOW), C_RESET, message);
                log_msg
            }
        }
    }

    pub fn login(&mut self) {
        self.stream.write(self.username.as_bytes()).expect("Error writing stream");
        utilities::ms_sleep(250);
        self.stream.write(self.token.as_bytes()).expect("Error writing stream");
    }

    /* fn send(&mut self) {
        if self.enable_user_input {
            let mut line_reader = rustyline::DefaultEditor::new().unwrap();

            loop {
                let message = line_reader.readline("").unwrap();
                line_reader.add_history_entry(&message).unwrap();
                self.stream.write(message.as_bytes()).expect("Error writing stream");
            }
        }
    } */

    fn recv(&mut self) {
        if self.json_fmt {
            let mut count: i8 = 0;

            loop {
                let mut buffer = [0u8; 1];
                let mut str_buffer = String::new();
                let mut wraps = 0;

                loop {
                    let stream_reader = self.stream.read(&mut buffer).expect(self.log_fmt("Error while reading from stream", LogLevel::ERROR).as_str());

                    if stream_reader == 0 {
                        self.logger("Server connection closed", LogLevel::ERROR);
                        exit(1)
                    }

                    match buffer[0] as char {
                        '{' => {
                            wraps += 1;
                            str_buffer.push('{');
                        }
                        '}' => {
                            wraps -= 1;
                            str_buffer.push('}');
                        }
                        c => str_buffer.push(c)
                    }

                    if wraps == 0 {
                        break
                    }
                }

                count = count + 1;
                
                let msg: Value = match serde_json::from_str(&str_buffer) {
                    Ok(ok) => ok,
                    Err(e) => {
                        self.logger(format!("Error desering packet ({str_buffer}): {e}").as_str(), LogLevel::ERROR);
                        continue;
                    }
                };

                if count > 8 {
                    match msg["message_type"].as_str() {
                        Some("system_message") => self.logger(msg["message"]["content"].as_str().unwrap(), LogLevel::MESSAGE),
                        Some("user_message") => self.logger(
                            format!(
                                "{}{} (@{}){}{C_RESET} {}",
                                msg["role_color"].as_str().unwrap(),
                                msg["nickname"].as_str().unwrap(),
                                msg["username"].as_str().unwrap().to_lowercase(),
                                addons::badge_handler(msg["badge"].as_str().unwrap()).as_str(),
                                msg["message"]["content"].as_str().unwrap()
                            ).as_str(), LogLevel::MESSAGE),

                        None => unreachable!(),
                        m => self.logger(
                            format!(
                                "{YELLOW}Unimplemented packet {} - full packet: {}", m.unwrap(), str_buffer
                            ).as_str(), LogLevel::WARNING),
                    }
                }
            }
        }
    }

    pub fn run(&mut self,) {
        self.logger(format!("{GREEN}Starting scapi {VERSION} (v{VERSION}{FULL_VERSION})").as_str(), LogLevel::INFO);
        if self.enable_user_input {
            self.logger(format!("{YELLOW}Flag {GREEN}{BOLD}'enabled_user_input'{C_RESET}{YELLOW} is enabled").as_str(), LogLevel::INFO);
        }
        if self.log_recv_msg {
            self.logger(format!("{YELLOW}Flag {GREEN}{BOLD}'log_recv_msg'{C_RESET}{YELLOW} is enabled").as_str(), LogLevel::INFO);
        }

        self.recv();
    }
}

