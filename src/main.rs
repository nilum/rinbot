use serenity::utils::MessageBuilder;
use serenity::{
    client::validate_token,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

use std::fs;
use std::process::Command;

struct Handler;

impl EventHandler for Handler {
    fn message(&self, context: Context, msg: Message) {
        if msg.content == "!restart" {
            let factorio_pid = fs::read_to_string("save_pid.txt").expect("Failed to read PID.");

            println!("Factorio PID is {}", factorio_pid);
            let _output = Command::new("kill")
                .arg("-2")
                .arg(factorio_pid)
                .output()
                .expect("Failed to kill.");

            let _output = Command::new("launch_factorio.sh")
                .current_dir("~/")
                .spawn()
                .expect("Failed to launch server");

            let response = MessageBuilder::new()
                .push_bold_line("Restarting the Factorio Server!")
                .push("It should be up and running in 90 seconds.")
                .build();

            if let Err(why) = msg.channel_id.say(&context.http, &response) {
                println!("Error sending message: {:?}", why);
            }
        } else if msg.content == "!log" {
            let log = fs::read_to_string("~/my.log").expect("failed to read log.");

            let last_logs = log.lines().rev().take(25);

            let mut true_log = String::new();
            for l in last_logs {
                true_log.push_str(l);
            }

            let message = MessageBuilder::new().push_quote(true_log).build();

            if let Err(why) = msg.channel_id.say(&context.http, &message) {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

fn main() {
    // Configure client with bot token

    let token = "";

    let mut client = Client::new(&token, Handler).expect("Error creating client.");

    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}
