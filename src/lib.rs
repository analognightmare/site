extern crate rand;
use rand::Rng;
use std::fmt;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

#[wasm_bindgen]
impl Universe {
    pub fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        self.cells = (0..self.width * self.height).map(|_i| Cell::Dead).collect();
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    fn live_neighbor_count(&self, idx: u32) -> u8 {
        [
            (idx - self.width - 1),
            (idx - self.width),
            (idx - self.width + 1),
            (idx - 1),
            (idx + 1),
            (idx + self.width - 1),
            (idx + self.width),
            (idx + self.width + 1),
        ]
        .iter()
        .fold(0, |acc, i| {
            acc + *self.cells.get(*i as usize).unwrap_or(&Cell::Dead) as u8
        })
    }

    pub fn tick(&mut self) {
        self.random();
        self.cells = (0..self.width * self.height)
            .enumerate()
            .map(|(idx, _)| {
                let live_neighbors = self.live_neighbor_count(idx as u32);
                match (self.cells[idx as usize], live_neighbors) {
                    (Cell::Alive, 2) | (Cell::Alive, 3) | (Cell::Dead, 3) => Cell::Alive,
                    (_, _) => Cell::Dead,
                }
            })
            .collect();
        for idx in 0..(self.height - 1) {
            self.cells[(idx * self.width) as usize] = Cell::Dead;
        }
    }

    pub fn random(&mut self) {
        let mut rng = rand::thread_rng();
        for idx in 1..(self.height - 1) {
            self.cells[((idx - 1) * self.width) as usize] = Cell::Dead;
            self.cells[(idx * self.width - 1) as usize] = match rng.gen::<bool>() {
                true => Cell::Alive,
                false => Cell::Dead,
            };
            if rng.gen::<bool>() {
                self.cells[(idx * self.width - 2) as usize] = match rng.gen::<bool>() {
                    true => Cell::Alive,
                    false => Cell::Dead,
                };
            }
            if rng.gen::<bool>() && rng.gen::<bool>() {
                self.cells[(idx * self.width - 3) as usize] = match rng.gen::<bool>() {
                    true => Cell::Alive,
                    false => Cell::Dead,
                };
            }
        }
    }

    pub fn new(width: u32, height: u32) -> Universe {
        // let cells = (0..width * height).map(|_| Cell::Dead).collect();
        let cells: Vec<Cell> = vec![Cell::Dead; (width * height) as usize];

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }
}

impl Universe {
    pub fn get_cells(&self) -> &[Cell] {
        &self.cells
    }

    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter() {
            self.cells[(row * self.width + col) as usize] = Cell::Alive;
        }
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
