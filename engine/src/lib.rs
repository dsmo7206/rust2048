use itertools::Itertools;
use rand::{seq::SliceRandom, thread_rng};

pub struct Board {
    blocks: [Option<u16>; 16],
    // max_value: u16,
    // score: u32,
}

impl Board {
    pub fn new() -> Self {
        let mut result = Self {
            blocks: [None; 16],
            // max_value: 2,
            // score: 0,
        };

        // Add the initial two
        result.next_turn();

        result
    }

    pub fn get_blocks(&self) -> &[Option<u16>; 16] {
        &self.blocks
    }

    fn rand_free_index(&self) -> usize {
        let free_indices = self
            .blocks
            .iter()
            .enumerate()
            .filter_map(|(i, block)| match block {
                Some(_) => None,
                None => Some(i),
            })
            .collect::<Vec<_>>();

        *free_indices
            .choose(&mut thread_rng())
            .expect("No free indices")
    }

    pub fn next_turn(&mut self) -> BoardState {
        if self.blocks.iter().all(|block| block.is_some()) {
            return BoardState::GameOverLose;
        }

        if self.blocks.iter().any(|block| match block {
            Some(2048) => true,
            _ => false,
        }) {
            return BoardState::GameOverWin;
        }

        self.blocks[self.rand_free_index()] = Some(2);

        BoardState::Ok
    }

    pub fn compress(&mut self, dir: Direction) {
        let (row_or_col, reverse) = match dir {
            Direction::Up => (RowOrCol::Col, false),
            Direction::Down => (RowOrCol::Col, true),
            Direction::Left => (RowOrCol::Row, false),
            Direction::Right => (RowOrCol::Row, true),
        };

        (0..4).for_each(|index| {
            self.compress_indexes(self.indexes(row_or_col, reverse, index));
        });
    }

    fn indexes(&self, row_or_col: RowOrCol, reverse: bool, index: usize) -> [usize; 4] {
        let mut indexes = match (row_or_col, index) {
            (RowOrCol::Row, 0) => [0, 1, 2, 3],
            (RowOrCol::Row, 1) => [4, 5, 6, 7],
            (RowOrCol::Row, 2) => [8, 9, 10, 11],
            (RowOrCol::Row, 3) => [12, 13, 14, 15],
            (RowOrCol::Col, 0) => [0, 4, 8, 12],
            (RowOrCol::Col, 1) => [1, 5, 9, 13],
            (RowOrCol::Col, 2) => [2, 6, 10, 14],
            (RowOrCol::Col, 3) => [3, 7, 11, 15],
            _ => unreachable!("Invalid index"),
        };
        if reverse {
            indexes.reverse();
        }
        indexes
    }

    fn compress_indexes(&mut self, indexes: [usize; 4]) {
        let values = indexes
            .iter()
            .filter_map(|&index| self.blocks[index])
            .dedup_with_count()
            .map(|(count, value)| match count {
                1 => vec![Some(value)],
                2 => vec![Some(value * 2)],
                3 => vec![Some(value * 2), Some(value)],
                4 => vec![Some(value * 2), Some(value * 2)],
                _ => unreachable!(),
            })
            .flatten()
            .chain(std::iter::repeat(None))
            .take(4)
            .collect::<Vec<_>>();

        for (&index, value) in indexes.iter().zip(values.into_iter()) {
            self.blocks[index] = value;
        }
    }
}

#[derive(Clone, Copy)]
enum RowOrCol {
    Row,
    Col,
}

pub enum BoardState {
    Ok,
    GameOverLose,
    GameOverWin,
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
