use crossterm::cursor::RestorePosition;
use crossterm::event::{Event, KeyCode, read};
use crossterm::execute;
use crossterm::terminal::ClearType::FromCursorDown;
use crossterm::terminal::{Clear, disable_raw_mode, enable_raw_mode};
use std::io;
use std::str::FromStr;

struct RawModeGuard;

impl RawModeGuard {
    fn new() -> io::Result<Self> {
        enable_raw_mode()?;
        Ok(RawModeGuard)
    }
}

impl Drop for RawModeGuard {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
    }
}

pub fn wait_for_press_of(key_codes: Vec<KeyCode>, activate_raw: bool) -> io::Result<KeyCode> {
    let _guard: RawModeGuard;
    if activate_raw {
        _guard = RawModeGuard::new()?;
    }
    loop {
        match read()? {
            Event::FocusGained => {}
            Event::FocusLost => {}
            Event::Key(key) => {
                if key_codes.contains(&key.code) {
                    return Ok(key.code);
                }
            }
            _ => {}
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
