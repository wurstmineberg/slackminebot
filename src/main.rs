#![warn(trivial_casts)]
#![forbid(unused, unused_extern_crates, unused_import_braces, unused_qualifications)]

extern crate slack;
extern crate serde_json;

mod logtail;

use std::process;

struct SlackHandler {
    will_exit: bool
}

impl SlackHandler {
    fn new() -> SlackHandler {
        return SlackHandler {
            will_exit: false
        }
    }

    fn parse_msg(&mut self, cli: &mut slack::RtmClient, value: serde_json::Value) -> Result<(), String> {
        let error = format!("Invalid Message Received {:?}", value);

        let map = try!(value.as_object().ok_or(error.clone()));
        let ty = match map.get("type") {
            Some(val) => try!(val.as_string().ok_or(error.clone())),
            None => {
                if map.get("ok").is_some() {
                    "result"
                } else {
                    return Err(error);
                }
            }
        };
        if ty == "message" {
            match map.get("text") {
                Some(val) => {
                    let text = try!(val.as_string().ok_or(error.clone()));
                    if text == "!test" {
                        let _ = cli.send_message("#wurstminebot-test", "got test msg");
                    } else if text == "!quit" {
                        let _ = cli.send_message("#wurstminebot-test", "bye");
                        self.will_exit = true;
                    }
                }
                None => return Ok(())
            }
        } else if ty == "result" {
            if self.will_exit {
                process::exit(0);
            }

            let ok_val = try!(map.get("ok").ok_or(error.clone()));
            let ok = try!(ok_val.as_boolean().ok_or(error.clone()));
            if !ok {
                return Err(format!("Got not okay response: {:?}", value));
            }
        }

        Ok(())
    }
}

impl slack::EventHandler for SlackHandler {
    fn on_event(&mut self, cli: &mut slack::RtmClient, _: Result<&slack::Event, slack::Error>, raw_json: &str) {
        let data: serde_json::Value = match serde_json::from_str(raw_json) {
            Ok(value) => value,
            Err(error) => {
                println!("{:?}", error);
                return;
            }
        };

        match self.parse_msg(cli, data) {
            Ok(()) => (),
            Err(msg) => println!("{}", msg)
        }
    }

    fn on_ping(&mut self, _: &mut slack::RtmClient) {}

    fn on_close(&mut self, _: &mut slack::RtmClient) {}

    fn on_connect(&mut self, cli: &mut slack::RtmClient) {
        println!("Successfully connected to the Slack API server");
        let _ = cli.send_message("#wurstminebot-test", "I'm back!");
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let api_key = match args.len() {
        0 | 1 => panic!("No api-key in args! Usage: ./slackminebot <api-key>"),
        x => {
            args[x-1].clone()
        }
    };
    let mut handler = SlackHandler::new();
    let mut cli = slack::RtmClient::new(&api_key);
    cli.login_and_run::<SlackHandler>(&mut handler).unwrap();
    println!("{}", cli.get_name().unwrap());
    println!("{}", cli.get_team().unwrap().name);
}
