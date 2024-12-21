use std::fmt::Display;

use advent2024::{
    advent_main,
    grid::GridCharWorld,
    multidim::{DirType, ManhattanDir, Position},
};
use pancurses::{endwin, initscr, noecho, Input};

fn main() -> anyhow::Result<()> {
    advent_main(|filename, part, options| {
        println!("{filename} {part:?}");
        if options.contains(&"-view") {
            view();
        }
        Ok(())
    })
}

fn view() {
    let mut chain = PadChain::default();
    let window = initscr();
    window.keypad(true);
    noecho();
    loop {
        window.clear();
        for pad in chain.pads.iter() {
            window.addstr(format!("{pad}\n"));
        }
        match window.getch() {
            Some(Input::Character(c)) => match c {
                '^' | 'v' | '<' | '>' | 'A' | 'a' => {
                    chain.move_arm(c);
                }
                'q' => break,
                _ => {}
            },
            Some(Input::KeyDC) => break,
            _ => {}
        }
        window.addstr(format!("{:?}", chain.outputs));
    }
    endwin();
}

struct PadChain {
    pads: [KeyPad; 4],
    outputs: Vec<char>,
}

impl Default for PadChain {
    fn default() -> Self {
        Self {
            pads: [
                KeyPad::new(DIRECTION_PAD),
                KeyPad::new(DIRECTION_PAD),
                KeyPad::new(DIRECTION_PAD),
                KeyPad::new(KEYPAD),
            ],
            outputs: vec![],
        }
    }
}

impl PadChain {
    fn move_arm(&mut self, mut key_char: char) {
        let mut i = 0;
        loop {
            match self.pads[i].move_arm(key_char) {
                None => return,
                Some(c) => {
                    i += 1;
                    if i == self.pads.len() {
                        self.outputs.push(c);
                        return;
                    } else {
                        key_char = c;
                    }
                }
            }
        }
    }
}

#[derive(Clone)]
struct KeyPad {
    keys: GridCharWorld,
    current: Position,
}

impl KeyPad {
    fn new(key_str: &str) -> Self {
        let keys = key_str.parse::<GridCharWorld>().unwrap();
        let current = keys.any_position_for('A');
        Self { keys, current }
    }

    fn move_arm(&mut self, key_char: char) -> Option<char> {
        if key_char.to_ascii_uppercase() == 'A' {
            Some(self.keys.value(self.current).unwrap())
        } else {
            let arm_moved = ManhattanDir::try_from(key_char)
                .unwrap()
                .neighbor(self.current);
            assert!(self.keys.in_bounds(arm_moved));
            self.current = arm_moved;
            None
        }
    }
}

impl Display for KeyPad {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for p in self.keys.position_iter() {
            if p[0] == 0 && p[1] > 0 {
                writeln!(f)?;
            }
            let c = if p == self.current {'*'} else { self.keys.value(p).unwrap()};
            write!(f, "{c}")?;
        }
        writeln!(f)
    }
}

const KEYPAD: &str = "789
456
123
 0A";

const DIRECTION_PAD: &str = " ^A
<v>";
