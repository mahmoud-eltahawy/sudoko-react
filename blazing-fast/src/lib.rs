use std::{
    cell::LazyCell,
    collections::{HashMap, HashSet},
};

use wasm_bindgen::prelude::*;

const SIZE: usize = 9;
const LEVELS_NUMBER: usize = 6;
const SIZE_SQUARED: usize = SIZE * SIZE;
const TOTAL_LEVELS_SIZE: usize = SIZE_SQUARED * LEVELS_NUMBER;

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
pub fn get_level(index: usize) -> Vec<u8> {
    let index = index % LEVELS_NUMBER;
    let begin = index * SIZE_SQUARED;
    LEVELS[begin..(begin + SIZE_SQUARED)].to_vec()
}

#[wasm_bindgen]
pub fn is_sudoku_board_full(board: Vec<u8>) -> bool {
    assert_eq!(board.len(), SIZE_SQUARED);
    !board.into_iter().any(|x| x == 0)
}

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
    areas: HashSet<usize>,
}

impl Sodoku {
    fn new(SodokuId { row, column, area }: &SodokuId) -> Self {
        let mut rows = HashSet::with_capacity(9);
        let mut columns = HashSet::with_capacity(9);
        let mut areas = HashSet::with_capacity(9);

        rows.insert(*row);
        columns.insert(*column);
        areas.insert(*area);

        Self {
            rows,
            columns,
            areas,
        }
    }

    fn exists(&mut self, SodokuId { row, column, area }: &SodokuId) -> bool {
        !self.rows.insert(*row) || !self.columns.insert(*column) || !self.areas.insert(*area)
    }
}

struct SodokuId {
    row: usize,
    column: usize,
    area: usize,
}

impl SodokuId {
    fn from(row: usize, column: usize) -> Self {
        let area = (row / 3) * 3 + (column / 3);
        Self { row, column, area }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_sudoko_ids() {
        assert_eq!(SUDOKO_IDS[0].row, 0);
        assert_eq!(SUDOKO_IDS[0].column, 0);
        assert_eq!(SUDOKO_IDS[0].area, 0);

        assert_eq!(SUDOKO_IDS[1].row, 0);
        assert_eq!(SUDOKO_IDS[1].column, 1);
        assert_eq!(SUDOKO_IDS[1].area, 0);

        assert_eq!(SUDOKO_IDS[4].row, 0);
        assert_eq!(SUDOKO_IDS[4].column, 4);
        assert_eq!(SUDOKO_IDS[4].area, 1);

        assert_eq!(SUDOKO_IDS[5].row, 0);
        assert_eq!(SUDOKO_IDS[5].column, 5);
        assert_eq!(SUDOKO_IDS[5].area, 1);

        assert_eq!(SUDOKO_IDS[7].row, 0);
        assert_eq!(SUDOKO_IDS[7].column, 7);
        assert_eq!(SUDOKO_IDS[7].area, 2);

        assert_eq!(SUDOKO_IDS[8].row, 0);
        assert_eq!(SUDOKO_IDS[8].column, 8);
        assert_eq!(SUDOKO_IDS[8].area, 2);

        assert_eq!(SUDOKO_IDS[9].row, 1);
        assert_eq!(SUDOKO_IDS[9].column, 0);
        assert_eq!(SUDOKO_IDS[9].area, 0);

        assert_eq!(SUDOKO_IDS[10].row, 1);
        assert_eq!(SUDOKO_IDS[10].column, 1);
        assert_eq!(SUDOKO_IDS[10].area, 0);

        assert_eq!(SUDOKO_IDS[12].row, 1);
        assert_eq!(SUDOKO_IDS[12].column, 3);
        assert_eq!(SUDOKO_IDS[12].area, 1);

        assert_eq!(SUDOKO_IDS[15].row, 1);
        assert_eq!(SUDOKO_IDS[15].column, 6);
        assert_eq!(SUDOKO_IDS[15].area, 2);

        assert_eq!(SUDOKO_IDS[17].row, 1);
        assert_eq!(SUDOKO_IDS[17].column, 8);
        assert_eq!(SUDOKO_IDS[17].area, 2);

        assert_eq!(SUDOKO_IDS[27].row, 3);
        assert_eq!(SUDOKO_IDS[27].column, 0);
        assert_eq!(SUDOKO_IDS[27].area, 3);
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
