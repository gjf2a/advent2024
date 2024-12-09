use std::{cmp::min, collections::VecDeque, fmt::Display};

use advent2024::{advent_main, all_lines, Part};

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
        while let Some(num_blocks) = char_seq.next() {
            let id_num = blocks.len();
            let free_space = char_seq.next().map_or(0, |c| c);
            blocks.push_back(BlockEntry {
                id_num,
                num_blocks,
                free_space,
            });
        }
        Self { blocks }
    }

    fn checksum(&self) -> usize {
        let mut total = 0;
        let mut i = 0;
        for block in self.blocks.iter() {
            total += (i..(i + block.num_blocks))
                .map(|j| j * block.id_num)
                .sum::<usize>();
            i += block.footprint();
        }
        total
    }

    fn total_blocks_stored(&self) -> usize {
        self.blocks.iter().map(|b| b.num_blocks).sum()
    }

    fn total_footprint(&self) -> usize {
        self.blocks.iter().map(|b| b.footprint()).sum()
    }

    fn compressed_fragmented(&self) -> Self {
        let mut src = self.clone();
        let mut cmp = Self::default();
        while let Some(mut front_block) = src.blocks.pop_front() {
            let extra = front_block.clear_free();
            cmp.append_block_entry(front_block);
            src.distribute(extra, &mut cmp);
        }
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

    fn distribute(&mut self, mut extra: usize, cmp: &mut Self) {
        while extra > 0 && self.blocks.len() > 0 {
            let end = self.blocks.back_mut().unwrap();
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
                self.blocks.pop_back();
            }
        }
    }

    fn compressed_contiguous(&self) -> Self {
        let mut cmp = self.clone();
        for candidate_id in (1..self.blocks.len()).rev() {
            let right = cmp.first_location_of(candidate_id);
            if let Some(left) = cmp.find_free_space_for(right) {
                cmp.swap_block_left(left, right);
            }
        }
        assert_eq!(self.total_footprint(), cmp.total_footprint());
        cmp
    }

    fn find_free_space_for(&self, right: usize) -> Option<usize> {
        (0..right).find(|left| self.blocks[*left].free_space >= self.blocks[right].num_blocks)
    }

    fn swap_block_left(&mut self, left: usize, right: usize) {
        if left == right - 1 {
            self.blocks[left].free_space -= self.blocks[right].num_blocks;
            self.blocks[right].free_space += self.blocks[right].num_blocks;
        } else {
            self.blocks[right - 1].free_space += self.blocks[right].footprint();
            let mut movee: BlockEntry = self.blocks.remove(right).unwrap();
            movee.free_space = self.blocks[left].free_space - movee.num_blocks;
            self.blocks[left].free_space = 0;
            self.blocks.insert(left + 1, movee);
        }
    }

    fn first_location_of(&self, id_num: usize) -> usize {
        self.blocks
            .iter()
            .enumerate()
            .find(|(_, b)| b.id_num == id_num)
            .map(|(i, _)| i)
            .unwrap()
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
