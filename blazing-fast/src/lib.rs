use std::{
    cell::LazyCell,
    collections::{HashMap, HashSet},
};

use wasm_bindgen::prelude::*;

const SIZE: usize = 9;
const LEVELS_NUMBER: usize = 6;
const SIZE_SQUARED: usize = SIZE * SIZE;
const TOTAL_LEVELS_SIZE: usize = SIZE_SQUARED * LEVELS_NUMBER;

const SUDOKO_IDS: LazyCell<Vec<SodokuId>> = LazyCell::new(|| {
    (0..SIZE_SQUARED)
        .collect::<Vec<_>>()
        .windows(SIZE)
        .step_by(SIZE)
        .enumerate()
        .flat_map(|(y, xs)| xs.into_iter().map(|x| (y, x % SIZE)).collect::<Vec<_>>())
        .map(|(y, x)| SodokuId::from(y, x))
        .collect::<Vec<_>>()
});

const LEVELS: [u8; TOTAL_LEVELS_SIZE] = [
    // level 1
    1, 2, 3, 4, 5, 6, 7, 8, 9, //
    4, 5, 6, 7, 8, 9, 1, 2, 3, //
    7, 8, 9, 1, 2, 3, 4, 5, 6, //
    2, 3, 4, 5, 6, 7, 8, 9, 1, //
    5, 6, 7, 8, 9, 1, 2, 3, 4, //
    8, 9, 1, 2, 3, 4, 5, 6, 7, //
    3, 4, 5, 6, 7, 8, 9, 1, 2, //
    6, 7, 8, 9, 1, 2, 3, 4, 5, //
    9, 1, 2, 3, 4, 5, 6, 7, 0, //
    // level 2
    1, 2, 3, 4, 5, 6, 7, 8, 9, //
    4, 5, 6, 7, 8, 9, 1, 2, 3, //
    7, 8, 0, 0, 2, 3, 4, 5, 6, //
    2, 3, 4, 5, 6, 7, 8, 9, 1, //
    5, 6, 7, 0, 9, 1, 2, 3, 4, //
    8, 9, 1, 2, 3, 4, 0, 0, 0, //
    3, 4, 5, 6, 7, 8, 0, 1, 2, //
    6, 7, 8, 9, 1, 2, 3, 0, 5, //
    9, 1, 2, 3, 4, 5, 6, 0, 0, //
    // level 3
    1, 2, 3, 4, 5, 6, 7, 8, 9, //
    4, 5, 0, 7, 8, 9, 1, 2, 3, //
    7, 8, 0, 0, 2, 3, 4, 5, 6, //
    2, 3, 0, 5, 6, 0, 8, 0, 1, //
    5, 6, 7, 0, 9, 1, 2, 3, 4, //
    8, 9, 1, 2, 3, 4, 0, 0, 0, //
    3, 0, 0, 6, 7, 8, 0, 1, 2, //
    6, 0, 8, 9, 1, 0, 3, 0, 5, //
    9, 1, 2, 3, 4, 5, 6, 0, 0, //
    // level 4
    1, 2, 3, 4, 5, 6, 7, 8, 9, //
    4, 5, 0, 7, 8, 9, 1, 2, 3, //
    7, 8, 0, 0, 2, 3, 4, 5, 6, //
    2, 3, 0, 5, 6, 0, 8, 0, 1, //
    5, 6, 7, 0, 9, 1, 2, 3, 4, //
    8, 9, 0, 0, 3, 4, 0, 0, 0, //
    3, 0, 0, 6, 7, 8, 0, 1, 2, //
    6, 0, 8, 9, 1, 0, 3, 0, 5, //
    9, 1, 2, 3, 4, 5, 6, 0, 0, //
    // level 5
    1, 2, 3, 4, 5, 6, 7, 8, 0, //
    4, 5, 0, 7, 8, 9, 1, 2, 0, //
    7, 8, 0, 0, 2, 0, 0, 5, 6, //
    2, 3, 0, 5, 6, 0, 8, 0, 1, //
    5, 6, 0, 0, 0, 1, 2, 3, 4, //
    8, 9, 1, 2, 0, 4, 0, 0, 0, //
    0, 0, 0, 6, 7, 8, 0, 1, 2, //
    6, 0, 8, 9, 1, 0, 3, 0, 5, //
    9, 1, 2, 3, 4, 5, 6, 0, 0, //
    // level 6
    5, 3, 0, 0, 7, 0, 0, 0, 0, //
    6, 0, 0, 1, 9, 5, 0, 0, 0, //
    0, 9, 8, 0, 0, 0, 0, 6, 0, //
    8, 0, 0, 0, 6, 0, 0, 0, 3, //
    4, 0, 0, 8, 0, 0, 0, 0, 1, //
    7, 0, 0, 0, 2, 0, 0, 0, 0, //
    0, 6, 0, 0, 0, 0, 2, 8, 0, //
    0, 0, 0, 4, 1, 9, 0, 0, 5, //
    0, 0, 0, 0, 8, 0, 0, 7, 9, //
];

#[wasm_bindgen]
pub fn which_section(index: usize) -> usize {
    SUDOKO_IDS[index].section
}

#[wasm_bindgen]
pub fn get_level(index: usize) -> Vec<u8> {
    let index = index % LEVELS_NUMBER;
    let index = index * SIZE_SQUARED;
    LEVELS[index..(index + SIZE_SQUARED)].to_vec()
}

#[wasm_bindgen]
pub fn is_sudoku_board_full(board: Vec<u8>) -> bool {
    assert_eq!(board.len(), SIZE_SQUARED);
    !board.into_iter().any(|x| x == 0)
}

#[wasm_bindgen]
pub fn is_valid_sudoku(board: Vec<u8>) -> bool {
    assert_eq!(board.len(), SIZE_SQUARED);
    let mut map = HashMap::<u8, Sodoku>::with_capacity(SIZE);
    !board
        .into_iter()
        .enumerate()
        .filter(|(_, x)| *x != 0)
        .any(|(index, number)| {
            let id = &SUDOKO_IDS[index];
            match map.get_mut(&number) {
                Some(sodokus) => {
                    if sodokus.exists(id) {
                        return true;
                    };
                }
                None => {
                    map.insert(number, Sodoku::new(id));
                }
            };
            false
        })
}

struct Sodoku {
    rows: HashSet<usize>,
    columns: HashSet<usize>,
    sections: HashSet<usize>,
}

impl Sodoku {
    fn new(SodokuId { y, x, section }: &SodokuId) -> Self {
        let mut rows = HashSet::with_capacity(9);
        let mut columns = HashSet::with_capacity(9);
        let mut sections = HashSet::with_capacity(9);

        rows.insert(*y);
        columns.insert(*x);
        sections.insert(*section);

        Self {
            rows,
            columns,
            sections,
        }
    }

    fn exists(&mut self, SodokuId { y, x, section }: &SodokuId) -> bool {
        !self.rows.insert(*y) || !self.columns.insert(*x) || !self.sections.insert(*section)
    }
}

struct SodokuId {
    y: usize,
    x: usize,
    section: usize,
}

impl SodokuId {
    fn from(y: usize, x: usize) -> Self {
        let section = (y / 3) * 3 + (x / 3);
        Self { y, x, section }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_sudoko_ids() {
        assert_eq!(SUDOKO_IDS[0].y, 0);
        assert_eq!(SUDOKO_IDS[0].x, 0);
        assert_eq!(SUDOKO_IDS[0].section, 0);

        assert_eq!(SUDOKO_IDS[1].y, 0);
        assert_eq!(SUDOKO_IDS[1].x, 1);
        assert_eq!(SUDOKO_IDS[1].section, 0);

        assert_eq!(SUDOKO_IDS[4].y, 0);
        assert_eq!(SUDOKO_IDS[4].x, 4);
        assert_eq!(SUDOKO_IDS[4].section, 1);

        assert_eq!(SUDOKO_IDS[5].y, 0);
        assert_eq!(SUDOKO_IDS[5].x, 5);
        assert_eq!(SUDOKO_IDS[5].section, 1);

        assert_eq!(SUDOKO_IDS[7].y, 0);
        assert_eq!(SUDOKO_IDS[7].x, 7);
        assert_eq!(SUDOKO_IDS[7].section, 2);

        assert_eq!(SUDOKO_IDS[8].y, 0);
        assert_eq!(SUDOKO_IDS[8].x, 8);
        assert_eq!(SUDOKO_IDS[8].section, 2);

        assert_eq!(SUDOKO_IDS[9].y, 1);
        assert_eq!(SUDOKO_IDS[9].x, 0);
        assert_eq!(SUDOKO_IDS[9].section, 0);

        assert_eq!(SUDOKO_IDS[10].y, 1);
        assert_eq!(SUDOKO_IDS[10].x, 1);
        assert_eq!(SUDOKO_IDS[10].section, 0);

        assert_eq!(SUDOKO_IDS[12].y, 1);
        assert_eq!(SUDOKO_IDS[12].x, 3);
        assert_eq!(SUDOKO_IDS[12].section, 1);

        assert_eq!(SUDOKO_IDS[15].y, 1);
        assert_eq!(SUDOKO_IDS[15].x, 6);
        assert_eq!(SUDOKO_IDS[15].section, 2);

        assert_eq!(SUDOKO_IDS[17].y, 1);
        assert_eq!(SUDOKO_IDS[17].x, 8);
        assert_eq!(SUDOKO_IDS[17].section, 2);

        assert_eq!(SUDOKO_IDS[27].y, 3);
        assert_eq!(SUDOKO_IDS[27].x, 0);
        assert_eq!(SUDOKO_IDS[27].section, 3);

        assert_eq!(SUDOKO_IDS[80].y, 8);
        assert_eq!(SUDOKO_IDS[80].x, 8);
        assert_eq!(SUDOKO_IDS[80].section, 8);
    }

    #[test]
    pub fn non_valid_levels() {
        assert!(!is_valid_sudoku(vec![
            8, 3, 0, 0, 7, 0, 0, 0, 0, //
            6, 0, 0, 1, 9, 5, 0, 0, 0, //
            0, 9, 8, 0, 0, 0, 0, 6, 0, //
            8, 0, 0, 0, 6, 0, 0, 0, 3, //
            4, 0, 0, 8, 0, 3, 0, 0, 1, //
            7, 0, 0, 0, 2, 0, 0, 0, 6, //
            0, 6, 0, 0, 0, 0, 2, 8, 0, //
            0, 0, 0, 4, 1, 9, 0, 0, 5, //
            0, 0, 0, 0, 8, 0, 0, 7, 9
        ]));

        assert!(!is_valid_sudoku(vec![
            0, 0, 4, 0, 0, 0, 6, 3, 0, //
            0, 0, 0, 0, 0, 0, 0, 0, 0, //
            5, 0, 0, 0, 0, 0, 0, 9, 0, //
            0, 0, 0, 5, 6, 0, 0, 0, 0, //
            4, 0, 3, 0, 0, 0, 0, 0, 1, //
            0, 0, 0, 7, 0, 0, 0, 0, 0, //
            0, 0, 0, 5, 0, 0, 0, 0, 0, //
            0, 0, 0, 0, 0, 0, 0, 0, 0, //
            0, 0, 0, 0, 0, 0, 0, 0, 0
        ]));
    }

    #[test]
    fn valid_levels() {
        for i in 0..LEVELS_NUMBER {
            debug_assert!(is_valid_sudoku(get_level(i)), "valid level number {}", i);
        }
    }

    #[test]
    fn full_board() {
        assert!(is_sudoku_board_full(vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, //
            4, 5, 6, 7, 8, 9, 1, 2, 3, //
            7, 8, 9, 1, 2, 3, 4, 5, 6, //
            2, 3, 4, 5, 6, 7, 8, 9, 1, //
            5, 6, 7, 8, 9, 1, 2, 3, 4, //
            8, 9, 1, 2, 3, 4, 5, 6, 7, //
            3, 4, 5, 6, 7, 8, 9, 1, 2, //
            6, 7, 8, 9, 1, 2, 3, 4, 5, //
            9, 1, 2, 3, 4, 5, 6, 7, 8, //
        ]));

        assert!(!is_sudoku_board_full(vec![
            0, 2, 3, 4, 5, 6, 7, 8, 9, //
            4, 5, 6, 7, 8, 9, 1, 2, 3, //
            7, 8, 9, 1, 2, 3, 4, 5, 6, //
            2, 3, 4, 5, 6, 7, 8, 9, 1, //
            5, 6, 7, 8, 9, 1, 2, 3, 4, //
            8, 9, 1, 2, 3, 4, 5, 6, 7, //
            3, 4, 5, 6, 7, 8, 9, 1, 2, //
            6, 7, 8, 9, 1, 2, 3, 4, 5, //
            9, 1, 2, 3, 4, 5, 6, 7, 8, //
        ]));

        assert!(!is_sudoku_board_full(vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, //
            4, 5, 6, 7, 8, 9, 1, 2, 3, //
            7, 8, 9, 1, 2, 3, 4, 5, 6, //
            2, 3, 4, 5, 6, 7, 8, 9, 1, //
            5, 6, 7, 8, 9, 1, 2, 3, 4, //
            8, 9, 1, 2, 3, 4, 5, 6, 7, //
            3, 4, 5, 6, 7, 8, 9, 1, 2, //
            6, 7, 8, 9, 1, 2, 3, 4, 5, //
            9, 1, 2, 3, 4, 5, 6, 7, 0, //
        ]));
    }
}
