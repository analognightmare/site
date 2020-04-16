extern crate rand;
use rand::Rng;
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
        let mut cells = vec![Cell::Dead; (width * height) as usize];
        for h in 0..self.height {
            if h >= height {
                continue;
            }
            for w in (0..self.width).rev() {
                let dif_abs = (self.width as i32 - width as i32).abs() as u32;
                let dif = width as i32 - self.width as i32;
                if w < dif_abs && self.width > width {
                    continue;
                }
                cells[((h * width + w) as i32 + dif) as usize] =
                    self.cells[(h * self.width + w) as usize];
            }
        }
        self.width = width;
        self.height = height;
        self.cells = cells;
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
        let cells: Vec<Cell> = vec![Cell::Dead; (width * height) as usize];

        Universe {
            width,
            height,
            cells,
        }
    }
}
