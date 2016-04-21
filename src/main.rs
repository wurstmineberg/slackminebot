#![warn(trivial_casts)]
#![forbid(unused, unused_extern_crates, unused_import_braces, unused_qualifications)]

extern crate slack;
extern crate serde_json;

mod logtail;

struct SlackHandler {
    will_exit: bool
}

impl Default for SlackHandler {
    fn default() -> SlackHandler {
        return SlackHandler {
            will_exit: false
        }
    }
}

impl slack::EventHandler for SlackHandler {
    fn on_event(&mut self, cli: &mut slack::RtmClient, event_result: Result<&slack::Event, slack::Error>, raw_json: &str) {
        let event = match event_result {
            Ok(event) => event,
            Err(error) => {
                match serde_json::from_str::<serde_json::Value>(raw_json) {
                    Ok(value) => {
                        println!("Slack error: {:?}, JSON: {:?}", error, value);
                    }
                    Err(error) => {
                        println!("Slack error: {:?}, JSON unreadable, raw string: {:?}", error, raw_json);
                    }
                };
                return;
            }
        };
        match *event {
            slack::Event::Hello => {
                println!("Successfully connected to the Slack API server");
                let _ = cli.send_message("#wurstminebot-test", "I'm back!");
            }
            slack::Event::Message(ref message) => {
                match *message {
                    slack::Message::Standard { ts: _, channel: _, user: _, ref text, is_starred: _, pinned_to: _, reactions: _, edited: _, attachments: _ } => {
                        if let Some(ref text) = *text {
                            if text == "!test" {
                                let _ = cli.send_message("#wurstminebot-test", "got test msg");
                            } else if text == "!quit" {
                                let _ = cli.send_message("#wurstminebot-test", "bye");
                                self.will_exit = true;
                            }
                        }
                    }
                    ref m => { println!("Message event not implemented: {:?}", m); } //TODO
                }
            }
            slack::Event::MessageSent { reply_to: _, ts: _, text: _ } => {
                if self.will_exit {
                    std::process::exit(0);
                }
            }
            ref e => { println!("Slack event not implemented: {:?}", e); } //TODO
        }
    }

    fn on_ping(&mut self, _: &mut slack::RtmClient) {}

    fn on_close(&mut self, _: &mut slack::RtmClient) {}

    fn on_connect(&mut self, _: &mut slack::RtmClient) {
        println!("Connection opened");
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
    let mut cli = slack::RtmClient::new(&api_key);
    cli.login_and_run(&mut SlackHandler::default()).unwrap();
    println!("{}", cli.get_name().unwrap());
    println!("{}", cli.get_team().unwrap().name);
}
