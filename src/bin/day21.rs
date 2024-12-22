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
    let chain = PadChain::default();
    let mut arms = chain.starting_arms();
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
                    chain.move_arms(c, &mut arms);
                }
                'q' => break,
                _ => {}
            },
            Some(Input::KeyDC) => break,
            _ => {}
        }
        window.addstr(format!("{:?}", arms.outputs));
    }
    endwin();
}

struct PadChain {
    pads: [KeyPad; 4],
}

struct Arms {
    arms: [Position; 4],
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
        }
    }
}

impl PadChain {
    fn starting_arms(&self) -> Arms {
        Arms {
            arms: self.pads.clone().map(|p| p.keys.any_position_for('A')),
            outputs: vec![],
        }
    }

    fn move_arms(&self, key_char: char, arms: &mut Arms) {
        let mut key_char = key_char;
        let mut i = 0;
        loop {
            match self.pads[i].arm_moved(key_char, arms.arms[i]) {
                Some(arm_moved) => {
                    arms.arms[i] = arm_moved;
                    return;
                }
                None => {
                    let c = self.pads[i].char_pressed(key_char, arms.arms[i]).unwrap();
                    i += 1;
                    if i == arms.arms.len() {
                        arms.outputs.push(c);
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

    fn char_pressed(&self, key_char: char, arm: Position) -> Option<char> {
        if key_char.to_ascii_uppercase() == 'A' {
            Some(self.keys.value(arm).unwrap())
        } else {
            None
        }
    }

    fn arm_moved(&self, key_char: char, arm: Position) -> Option<Position> {
        let arm_moved = ManhattanDir::try_from(key_char).unwrap().neighbor(arm);
        self.keys
            .value(arm_moved)
            .filter(|v| *v != ' ')
            .map(|_| arm_moved)
    }
}

impl Display for KeyPad {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for p in self.keys.position_iter() {
            if p[0] == 0 && p[1] > 0 {
                writeln!(f)?;
            }
            let c = if p == self.current {
                '*'
            } else {
                self.keys.value(p).unwrap()
            };
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
