use discord::model::Event;
use discord::{Discord, State};
use std::env;

fn main() {
    // Log in to Discord using a bot token from the environment
    let token = &env::var("DISCORD_TOKEN").expect("Expected token");
    let discord = Discord::from_user_token(token)
        .expect("login failed");

    // Establish and use a websocket connection
    let (mut connection, ready) = discord.connect().expect("connect failed");
    println!("Ready.");

    let mut state = State::new(ready);
    loop {
        let event = match connection.recv_event() {
            Ok(event) => event,
            Err(err) => {
                println!("[Warning] Receive error: {:?}", err);
                if let discord::Error::WebSocket(..) = err {
                    // Handle the websocket connection being dropped
                    let (new_connection, ready) = discord.connect().expect("connect failed");
                    connection = new_connection;
                    println!("[Ready] Reconnected successfully.");
                }
                if let discord::Error::Closed(..) = err {
                    break;
                }
                continue;
            },
        };

        state.update(&event);

        match event {
            Event::MessageCreate(message) => {
                if message.author.id == state.user().id {
                    continue;
                }
                println!("{}: {}", message.author.name, message.content);
            }
            _ => {}
        }
    }
}