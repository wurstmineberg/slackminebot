extern crate slack;
extern crate serde_json;

use std::process;


struct SlackHandler {
    will_exit: bool,
}


impl SlackHandler {
    fn new() -> SlackHandler {
        return SlackHandler{
            will_exit: false,
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

#[allow(unused_variables)]
impl slack::EventHandler for SlackHandler {
    fn on_receive(&mut self, cli: &mut slack::RtmClient, json_str: &str){
        let data: serde_json::Value = match serde_json::from_str(json_str) {
            Ok(value) => value,
            Err(error) => {
                println!("{:?}", error);
                return;
            },
        };

        match self.parse_msg(cli, data) {
            Ok(()) => (),
            Err(msg) => println!("{}", msg)
        }
    }

    fn on_ping(&mut self, cli: &mut slack::RtmClient){
    }

    fn on_close(&mut self, cli: &mut slack::RtmClient){
    }

    fn on_connect(&mut self, cli: &mut slack::RtmClient){
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
    let mut cli = slack::RtmClient::new();
    let r = cli.login_and_run::<SlackHandler>(&mut handler, &api_key);
    match r {
        Ok(_) => {},
        Err(err) => panic!("Error: {}", err)
    }
    println!("{}", cli.get_name().unwrap());
    println!("{}", cli.get_team().unwrap().name);
}
