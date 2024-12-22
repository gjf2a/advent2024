use advent2024::{
    advent_main, all_lines,
    grid::GridCharWorld,
    multidim::{DirType, ManhattanDir, Position},
    search_iter::BfsIter,
};
use pancurses::{endwin, initscr, noecho, Input};

fn main() -> anyhow::Result<()> {
    advent_main(|filename, part, options| {
        println!("{filename} {part:?}");
        if options.contains(&"-view") {
            view();
        } else {
            let chain = PadChain::default();
            let mut total_complexity = 0;
            for code in all_lines(filename)? {
                println!("Checking {code}");
                let arms = chain.starting_arms();
                let mut searcher = BfsIter::new(arms.clone(), |current| {
                    let current = current.clone();
                    let chain = chain.clone();
                    println!("{current:?}");
                    ['<', '>', 'v', '^', 'A']
                        .iter()
                        .filter_map(move |c| chain.moved_arms(*c, &current))
                });
                match searcher.find(|arms| arms.output_matches(code.as_str())) {
                    Some(found) => {
                        let length = searcher.depth_for(&found);
                        let encoded = (&code[..(code.len() - 1)]).parse::<usize>().unwrap();
                        println!(
                            "length: {length} encoded: {encoded} complexity: {}",
                            length * encoded
                        );
                        total_complexity += length * encoded;
                    }
                    None => {
                        panic!("Unreachable: {code}");
                    }
                }
            }
            println!("{total_complexity}");
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
        for (i, pad) in chain.pads.iter().enumerate() {
            window.addstr(format!("{}\n", pad.show(arms.arms[i])));
        }
        match window.getch() {
            Some(Input::Character(c)) => match c {
                '^' | 'v' | '<' | '>' | 'A' | 'a' => {
                    if let Some(moved) = chain.moved_arms(c, &arms) {
                        arms = moved;
                    }
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

#[derive(Clone)]
struct PadChain {
    pads: [KeyPad; 4],
}

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
struct Arms {
    arms: [Position; 4],
    outputs: Vec<char>,
}

impl Arms {
    fn output_matches(&self, target: &str) -> bool {
        self.outputs.len() == target.len()
            && target
                .chars()
                .zip(self.outputs.iter())
                .all(|(c1, c2)| c1 == *c2)
    }
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

    fn moved_arms(&self, key_char: char, arms: &Arms) -> Option<Arms> {
        let mut arms = arms.clone();
        let mut key_char = key_char;
        let mut i = 0;
        loop {
            match self.pads[i].arm_moved(key_char, arms.arms[i]) {
                Some(arm_moved) => {
                    arms.arms[i] = arm_moved;
                    return Some(arms);
                }
                None => match self.pads[i].char_pressed(key_char, arms.arms[i]) {
                    Some(c) => {
                        i += 1;
                        if i == arms.arms.len() {
                            arms.outputs.push(c);
                            return Some(arms);
                        } else {
                            key_char = c;
                        }
                    }
                    None => return None,
                },
            }
        }
    }
}

#[derive(Clone)]
struct KeyPad {
    keys: GridCharWorld,
}

impl KeyPad {
    fn new(key_str: &str) -> Self {
        let keys = key_str.parse::<GridCharWorld>().unwrap();
        Self { keys }
    }

    fn char_pressed(&self, key_char: char, arm: Position) -> Option<char> {
        if key_char.to_ascii_uppercase() == 'A' {
            Some(self.keys.value(arm).unwrap())
        } else {
            None
        }
    }

    fn arm_moved(&self, key_char: char, arm: Position) -> Option<Position> {
        match ManhattanDir::try_from(key_char) {
            Err(_) => None,
            Ok(dir) => {
                let arm_moved = dir.neighbor(arm);
                self.keys
                    .value(arm_moved)
                    .filter(|v| *v != ' ')
                    .map(|_| arm_moved)
            }
        }
    }

    fn show(&self, arm: Position) -> String {
        let mut result = String::new();
        for p in self.keys.position_iter() {
            if p[0] == 0 && p[1] > 0 {
                result.push('\n');
            }
            let c = if p == arm {
                '*'
            } else {
                self.keys.value(p).unwrap()
            };
            result.push(c);
        }
        result.push('\n');
        result
    }
}

const KEYPAD: &str = "789
456
123
 0A";

const DIRECTION_PAD: &str = " ^A
<v>";
