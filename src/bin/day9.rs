use std::{cmp::min, collections::VecDeque, fmt::Display};

use advent2024::{advent_main, all_lines, Part};

// 9963020502985 is too high on Part 2.

fn main() -> anyhow::Result<()> {
    advent_main(|filename, part, options| {
        let file_blocks = FileBlocks::new(all_lines(filename)?.next().unwrap());
        let cmp = match part {
            Part::One => file_blocks.compressed_fragmented(),
            Part::Two => file_blocks.compressed_contiguous(),
        };
        assert_eq!(file_blocks.total_blocks_stored(), cmp.total_blocks_stored());
        if options.contains(&"-show") {
            println!("{cmp}");
        }
        println!("{}", cmp.checksum());
        Ok(())
    })
}

#[derive(Clone, Default, Debug)]
struct FileBlocks {
    blocks: VecDeque<BlockEntry>,
}

#[derive(Copy, Clone, Debug)]
struct BlockEntry {
    id_num: usize,
    num_blocks: usize,
    free_space: usize,
}

impl FileBlocks {
    fn new(input_line: String) -> Self {
        let mut blocks = VecDeque::new();
        let mut char_seq = input_line.chars().map(|c| c.to_digit(10).unwrap() as usize);
        loop {
            let num_blocks = char_seq.next().unwrap();
            let id_num = blocks.len();
            match char_seq.next() {
                None => {
                    blocks.push_back(BlockEntry {
                        id_num,
                        num_blocks,
                        free_space: 0,
                    });
                    return Self { blocks };
                }
                Some(free_space) => {
                    blocks.push_back(BlockEntry {
                        id_num,
                        num_blocks,
                        free_space,
                    });
                }
            }
        }
    }

    fn compressed_fragmented(&self) -> Self {
        let mut src = self.clone();
        let mut cmp = Self::default();
        while let Some(mut front_block) = src.blocks.pop_front() {
            let mut extra = front_block.clear_free();
            cmp.append_block_entry(front_block);
            while extra > 0 && src.blocks.len() > 0 {
                let end = src.blocks.back_mut().unwrap();
                let move_count = min(extra, end.num_blocks);
                if move_count > 0 {
                    extra -= move_count;
                    end.num_blocks -= move_count;
                    cmp.blocks.push_back(BlockEntry {
                        id_num: end.id_num,
                        num_blocks: move_count,
                        free_space: 0,
                    });
                }
                if end.num_blocks == 0 {
                    src.blocks.pop_back();
                }
            }
        }
        cmp
    }

    fn compressed_contiguous(&self) -> Self {
        let mut cmp = self.clone();
        for down in (1..self.blocks.len()).rev() {
            let down = self.first_location_of(down);
            for up in 0..down {
                if cmp.blocks[up].free_space >= cmp.blocks[down].num_blocks {
                    assert_ne!(up, down - 1);
                    cmp.blocks[down - 1].free_space += cmp.blocks[down].footprint();
                    let mut movee = cmp.blocks.remove(down).unwrap();
                    movee.free_space = cmp.blocks[up].free_space - movee.num_blocks;
                    cmp.blocks[up].free_space = 0;
                    cmp.blocks.insert(up + 1, movee);
                    break;
                }
            }
        }
        assert_eq!(self.total_footprint(), cmp.total_footprint());
        cmp
    }

    fn append_block_entry(&mut self, entry: BlockEntry) {
        if let Some(end) = self.blocks.back_mut() {
            if end.id_num == entry.id_num {
                end.num_blocks += entry.num_blocks;
                return;
            }
        }
        self.blocks.push_back(entry);
    }

    fn total_blocks_stored(&self) -> usize {
        self.blocks.iter().map(|b| b.num_blocks).sum()
    }

    fn total_footprint(&self) -> usize {
        self.blocks.iter().map(|b| b.footprint()).sum()
    }

    fn first_location_of(&self, id_num: usize) -> usize {
        self.blocks
            .iter()
            .enumerate()
            .find(|(_, b)| b.id_num == id_num)
            .map(|(i, _)| i)
            .unwrap()
    }

    fn checksum(&self) -> usize {
        let mut total = 0;
        let mut i = 0;
        for block in self.blocks.iter() {
            for j in i..(i + block.num_blocks) {
                total += j * block.id_num;
            }
            i += block.footprint();
        }
        total
    }
}

impl BlockEntry {
    fn clear_free(&mut self) -> usize {
        let free = self.free_space;
        self.free_space = 0;
        free
    }

    fn footprint(&self) -> usize {
        self.num_blocks + self.free_space
    }
}

impl Display for FileBlocks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for block in self.blocks.iter() {
            for _ in 0..block.num_blocks {
                write!(f, "{}", block.id_num)?;
            }
            for _ in 0..block.free_space {
                write!(f, ".")?;
            }
        }
        Ok(())
    }
}
