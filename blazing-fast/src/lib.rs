use std::collections::{HashMap, HashSet};

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
pub fn level(index: usize) -> Vec<u8> {
    let index = index % LEVELS_NUMBER;
    let begin = index * SIZE_SQUARED;
    LEVELS[begin..(begin + SIZE_SQUARED)].to_vec()
}

#[wasm_bindgen]
pub fn is_valid_sudoku(board: Vec<u8>) -> bool {
    let mut map = HashMap::<u8, Sodoku>::with_capacity(SIZE);
    !board
        .windows(SIZE)
        .step_by(SIZE)
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, x)| **x != 0)
                .map(|(x, number)| (number, SodokuElement::from(y, x)))
                .collect::<Vec<_>>()
        })
        .any(|(number, element)| {
            match map.get_mut(number) {
                Some(sodokus) => {
                    if sodokus.exists(element) {
                        return true;
                    };
                }
                None => {
                    map.insert(*number, Sodoku::new(element));
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
    fn new(SodokuElement { row, column, area }: SodokuElement) -> Self {
        let mut rows = HashSet::with_capacity(9);
        let mut columns = HashSet::with_capacity(9);
        let mut areas = HashSet::with_capacity(9);

        rows.insert(row);
        columns.insert(column);
        areas.insert(area);

        Self {
            rows,
            columns,
            areas,
        }
    }

    fn exists(&mut self, SodokuElement { row, column, area }: SodokuElement) -> bool {
        let rows = self.rows.insert(row);
        let columns = self.columns.insert(column);
        let areas = self.areas.insert(area);
        !rows || !columns || !areas
    }
}

struct SodokuElement {
    row: usize,
    column: usize,
    area: usize,
}

impl SodokuElement {
    fn from(row: usize, column: usize) -> Self {
        let area = match (row, column) {
            (0..3, 0..3) => 0,
            (0..3, 3..6) => 1,
            (0..3, 6..9) => 2,
            (3..6, 0..3) => 3,
            (3..6, 3..6) => 4,
            (3..6, 6..9) => 5,
            (6..9, 0..3) => 6,
            (6..9, 3..6) => 7,
            (6..9, 6..9) => 8,
            _ => unreachable!(),
        };

        Self { row, column, area }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
            assert!(is_valid_sudoku(level(i)));
        }
    }
}
