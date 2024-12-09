use std::{cmp::min, collections::VecDeque};

use advent2024::{advent_main, all_lines};

fn main() -> anyhow::Result<()> {
    advent_main(|filename, part, _| {
        let file_blocks = FileBlocks::new(all_lines(filename)?.next().unwrap());
        let cmp = file_blocks.compressed();
        println!("{}", cmp.checksum());
        Ok(())
    })
}

#[derive(Clone, Default, Debug)]
struct FileBlocks {
    blocks: VecDeque<BlockEntry>
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
                    blocks.push_back(BlockEntry {id_num, num_blocks, free_space: 0});
                    return Self {blocks};
                }
                Some(free_space) => {
                    blocks.push_back(BlockEntry {id_num, num_blocks, free_space});
                }
            }
        }
    }

    fn compressed(&self) -> Self {
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
                    cmp.blocks.push_back(BlockEntry { id_num: end.id_num, num_blocks: move_count, free_space: 0 });
                }
                if end.num_blocks == 0 {
                    src.blocks.pop_back();
                }
            }
        }
        assert_eq!(self.total_blocks_stored(), cmp.total_blocks_stored());
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

    fn checksum(&self) -> usize {
        let mut total = 0;
        let mut i = 0;
        for block in self.blocks.iter() {
            for j in i..(i + block.num_blocks) {
                total += j * block.id_num;
            }
            i += block.num_blocks;
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
}