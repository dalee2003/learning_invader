use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use rusty_audio::Audio;
use std::{
    error::Error,
    sync::mpsc::{self, Receiver},
    time::{Duration, Instant},
    {io, thread},
};

fn main() -> Result<(), Box<dyn Error>>{
    let mut audio = Audio::new();

    audio.add("explode", "audio/original/explode.wav");
    audio.add("lose", "audio/original/lose.wav");
    audio.add("move", "audio/original/move.wav");
    audio.add("pew", "audio/original/pew.wav");
    audio.add("startup", "audio/original/startup.wav");
    audio.add("win", "audio/original/win.wav");
    audio.play("startup");

    //Terminal
    let mut stdout = io::stdout(); //gets access to stdout
    terminal::enable_raw_mode()?; //gets keyboard input as it occurs; ?: crash if error
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    //Game Loop
    'gameloop: loop{
        //Input
        while event::poll(Duration::default())?{
            if let Event::Key(key_event) = event::read()?{
                match key_event.code{
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play("lose");
                        break 'gameloop;
                    }
                    _ => {}
                }
            }
        }
    }


    audio.wait(); //block until audio is done playing
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}
