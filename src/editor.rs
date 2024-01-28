use crate::terminal::Terminal;

use termion::event::Key;

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
        Terminal::clear_screen();
        Terminal::cursor_position(0, 0);
        if self.should_quit {
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

    fn draw_rows(&self) {
        for _ in 0..self.terminal.size().height - 1 {
            println!("~\r")
        }
    }
}

// Errors out and exits the program
fn die(e: std::io::Error) {
    Terminal::clear_screen();
    panic!("{}", e);
}
