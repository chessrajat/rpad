use crate::terminal::Terminal;

use termion::event::Key;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
}

impl Editor {
    pub fn default() -> Self {
        Self {
            should_quit: false,
            terminal: Terminal::new().expect("Failed to initialize terminal!"),
        }
    }

    pub fn run(&mut self) {
        loop {
            if let Err(error) = self.refresh_screen() {
                die(error)
            }

            if self.should_quit {
                break;
            }

            if let Err(error) = self.process_keypress() {
                die(error)
            }
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        //clear and reposition cursor
        Terminal::cursor_hide();
        Terminal::cursor_position(0, 0);
        if self.should_quit {
            Terminal::clear_screen();
            println!("GoodBye !! \r")
        } else {
            self.draw_rows();
            Terminal::cursor_position(0, 0)
        }
        Terminal::cursor_show();
        Terminal::flush()
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Ctrl('q') => self.should_quit = true,
            _ => (),
        }

        return Ok(());
    }

    fn draw_welcome_message(&self) {
        let mut welcome_message = format!("RPAD Editor -- version {}\r", VERSION);
        let width = self.terminal.size().width as usize;
        let message_length = welcome_message.len();
        let padding = width.saturating_sub(message_length) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        welcome_message = format!("~{}{}", spaces, welcome_message);
        welcome_message.truncate(width);
        println!("{}\r", welcome_message);
    }

    fn draw_rows(&self) {
        let height = self.terminal.size().height;
        for row in 0..height - 1 {
            Terminal::clear_current_line();
            if row == 0 {
                self.draw_welcome_message();
            } else {
                println!("~\r")
            }
        }
    }
}

// Errors out and exits the program
fn die(e: std::io::Error) {
    Terminal::clear_screen();
    panic!("{}", e);
}
