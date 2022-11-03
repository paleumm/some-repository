use std::env::args;
use std::fs;
use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{color, style};

struct Doc {
    lines: Vec<String>,
}

#[derive(Debug)]
struct Coordinates {
    pub x: usize,
    pub y: usize,
}

struct TextViewer {
    doc: Doc,
    doc_len: usize,
    cur_pos: Coordinates,
    terminal_size: Coordinates,
    file_name: String,
}

impl TextViewer {
    fn init(filename: &str) -> Self {
        let mut doc_file = Doc { lines: vec![] };
        let file_handle = fs::read_to_string(filename).unwrap();

        for doc_line in file_handle.lines() {
            doc_file.lines.push(doc_line.to_string());
        }
        let doc_len = file_handle.lines().count();
        let size = termion::terminal_size().unwrap();

        Self {
            doc: doc_file,
            cur_pos: Coordinates { x: 1, y: doc_len },
            doc_len: doc_len,
            terminal_size: Coordinates {
                x: size.0 as usize,
                y: size.1 as usize,
            },
            file_name: filename.into(),
        }
    }

    fn show_document(&mut self) {
        let pos = &self.cur_pos;
        let (old_x, old_y) = (pos.x, pos.y);
        print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
        println!(
            "{}{}Paleumm TUI\r{}",
            color::Bg(color::Black),
            color::Fg(color::White),
            style::Reset
        );
        if self.doc_len < self.terminal_size.y {
            for line in 0..self.doc_len {
                println!("{}\r", self.doc.lines[line as usize]);
            }
        } else {
            if pos.y <= self.terminal_size.y {
                for line in 0..self.terminal_size.y - 3 {
                    println!("{}\r", self.doc.lines[line as usize]);
                }
            } else {
                for line in pos.y - (self.terminal_size.y - 3)..pos.y {
                    println!("{}\r", self.doc.lines[line as usize]);
                }
            }
        }

        println!(
            "{}",
            termion::cursor::Goto(0, (self.terminal_size.y - 2) as u16)
        );

        println!(
            "{}{} line-count={} Filename: {}{}",
            color::Fg(color::Red),
            style::Bold,
            self.doc_len,
            self.file_name,
            style::Reset
        );

        self.set_pos(old_x, old_y);
    }

    fn set_pos(&mut self, x: usize, y: usize) {
        self.cur_pos.x = x;
        self.cur_pos.y = y;

        println!(
            "{}",
            termion::cursor::Goto((self.cur_pos.x) as u16, (self.cur_pos.y) as u16)
        );
    }

    fn terminal_print(&self) {
        println!(
            "{}",
            termion::cursor::Goto(self.cur_pos.x as u16, self.cur_pos.y as u16)
        );
    }

    fn inc_x(&mut self) {
        if self.cur_pos.x < self.terminal_size.x {
            self.cur_pos.x += 1;
        }
        self.terminal_print();
    }

    fn dec_x(&mut self) {
        if self.cur_pos.x > 1 {
            self.cur_pos.x -= 1;
        }
        self.terminal_print();
    }

    fn inc_y(&mut self) {
        if self.cur_pos.y < self.doc_len {
            self.cur_pos.y += 1;
        }
        self.terminal_print();
    }

    fn dec_y(&mut self) {
        if self.cur_pos.y > 1 {
            self.cur_pos.y -= 1;
        }
        self.terminal_print();
    }

    fn run(&mut self) {
        let mut stdout = stdout().into_raw_mode().unwrap();
        let stdin = stdin();

        for c in stdin.keys() {
            match c.unwrap() {
                Key::Ctrl('q') => {
                    break;
                }
                Key::Left => {
                    self.dec_x();
                    self.show_document();
                }
                Key::Right => {
                    self.inc_x();
                    self.show_document();
                }
                Key::Up => {
                    self.dec_y();
                    self.show_document();
                }
                Key::Down => {
                    self.inc_y();
                    self.show_document();
                }
                Key::Backspace => {
                    self.dec_x();
                }
                _ => {}
            }
            stdout.flush().unwrap();
        }
    }
}

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() < 2 {
        println!("Please provide file name as an argument");
        std::process::exit(0);
    }

    if !std::path::Path::new(&args[1]).exists() {
        println!("File does not exist");
        std::process::exit(0);
    }

    println!("{}", termion::cursor::Show);

    let mut viewer = TextViewer::init(&args[1]);
    viewer.show_document();
    viewer.run();
}
