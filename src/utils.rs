use crossterm::cursor::RestorePosition;
use crossterm::event::{Event, KeyCode, read};
use crossterm::execute;
use crossterm::terminal::ClearType::FromCursorDown;
use crossterm::terminal::{Clear, disable_raw_mode, enable_raw_mode};
use std::io;
use std::str::FromStr;

pub fn wait_for_press_of(key_codes: Vec<KeyCode>) -> io::Result<KeyCode> {
    enable_raw_mode()?;
    loop {
        match read()? {
            Event::FocusGained => {}
            Event::FocusLost => {}
            Event::Key(key) => {
                if key_codes.contains(&key.code) {
                    disable_raw_mode()?;
                    return Ok(key.code);
                }
            }
            Event::Mouse(_) => {}
            Event::Paste(_) => {}
            Event::Resize(_, _) => {}
        }
    }
}

pub fn get_int_from_input<T>() -> T
where
    T: FromStr,
{
    loop {
        let mut string = String::new();
        io::stdin().read_line(&mut string).unwrap();
        let Ok(n) = string.trim().parse::<T>() else {
            execute!(io::stdout(), RestorePosition, Clear(FromCursorDown)).unwrap();
            println!("Please enter a number! ");
            continue;
        };
        return n;
    }
}
