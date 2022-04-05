use discord::model::Event as DiscordEvent;
use discord::{Discord, State};
use std::{io, env};
use tui::{
    backend::{CrosstermBackend, Backend},
    widgets::{Widget, Block, Borders},
    layout::{Layout, Constraint, Direction},
    text::Text,
    Terminal,
    Frame,
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, KeyCode, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

enum InputMode {
    Normal,
    Editing,
}

struct App {
    input: String,
    input_mode: InputMode,
    messages: Vec<String>,
}

impl Default for App {
    fn default() -> App {
        App {
            input: String::new(),
            input_mode: InputMode::Normal,
            messages: Vec::new(),
        }
    }
}

fn main() -> Result<(), io::Error>{
    // Setup Terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App::default();
    let res = run_app(&mut terminal, app);

    // restore terminal
    disable_raw_mode();
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    );
    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    // Log in to Discord using a bot token from the environment
    let token = &env::var("DISCORD_TOKEN").expect("Expected token");
    let discord = Discord::from_user_token(token)
        .expect("login failed");
    // Establish and use a websocket connection
    let (mut connection, ready) = discord.connect().expect("connect failed");
    println!("Ready.");

    let mut state = State::new(ready);
    loop {
        println!("1");
        terminal.draw(|f| ui(f, &app))?;
        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                println!("manas");
                return Ok(());
            }
        }
        println!("2");
        let event = match connection.recv_event() {
            Ok(event) => event,
            Err(err) => {
                println!("[Warning] Receive error: {:?}", err);
                println!("bust");
                if let discord::Error::WebSocket(..) = err {
                    // Handle the websocket connection being dropped
                    let (new_connection, ready) = discord.connect().expect("connection failed");
                    connection = new_connection;
                    println!("[Ready] Reconnected successfully.");
                }
                if let discord::Error::Closed(..) = err {
                    break;
                }
                println!("bust 2");
                continue;
            },
        };

        println!("3");
        state.update(&event); 
        println!("4");
        match event {
            DiscordEvent::MessageCreate(message) => {
                println!("bustisus");
                if message.author.id == state.user().id {
                    continue;
                }
                println!("{}: {}", message.author.name, message.content);
            }
            _ => {}
        }
    }
    return Ok(());
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let size = f.size();
    let block = Block::default()
        .title("Messages")
        .borders(Borders::ALL);
    f.render_widget(block, size);

}