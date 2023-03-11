mod utils;

extern crate console_error_panic_hook;

use std::panic;
use wasm_bindgen::prelude::*;
// use std::fmt;
use rand::Rng;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

// #[wasm_bindgen]
// #[repr(u8)]
// #[derive(Clone, Copy, Debug, PartialEq, Eq)]
// pub enum Cell {
//     Dead = 0,
//     Alive = 1,
// }

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<u8>,
}

#[wasm_bindgen]
impl Universe {
    pub fn tick(&mut self) {

        let mut cells_copy = self.cells.clone();
        let mut changed: Vec<(u32, u32, u8)> = Vec::new();

        for row in 0..self.height {
            for col in 0..self.width {
                let cell = self.get_bit(row, col);
                let live_neighbors = self.live_neighbor_count(row, col);

                match (cell, live_neighbors) {
                    // Rule 1: Any live cell with fewer than two live neighbours
                    // dies, as if caused by underpopulation.
                    (1, x) if x < 2 => {
                      changed.push((row, col, 0));
                      self.set_bit(row, col, 0, &mut cells_copy)
                    }

                    // Rule 2: Any live cell with two or three live neighbours
                    // lives on to the next generation.
                    (1, 2) | (1, 3) => {
                      changed.push((row, col, 1));
                      self.set_bit(row, col, 1, &mut cells_copy)
                    }

                    // Rule 3: Any live cell with more than three live
                    // neighbours dies, as if by overpopulation.
                    (1, x) if x > 3 => {
                      changed.push((row, col, 0));
                      self.set_bit(row, col, 0, &mut cells_copy)
                    }

                    // Rule 4: Any dead cell with exactly three live neighbours
                    // becomes a live cell, as if by reproduction.
                    (0, 3) => {
                      changed.push((row, col, 1));
                      self.set_bit(row, col, 1, &mut cells_copy)
                    }

                    // All other cells remain in the same state.
                    (otherwise, _) => otherwise,
                };
            }
        }

        for (changed_row, changed_col, value) in changed {
          self.set_cell_bit(changed_row, changed_col, value);
        }
    }

    pub fn new(width: u32, height: u32) -> Universe {
        utils::set_logger();
        utils::set_panic_hook();

        panic::set_hook(Box::new(console_error_panic_hook::hook));

        let mut rng = rand::thread_rng();

        let cells = (0..width * (height / 8))
            .map(|_i| {
                rng.gen_range(0..255)
            }).collect::<Vec<u8>>();

        // create a glider
        // cells[13] = 0b00001000;
        // cells[21] = 0b00010000;
        // cells[29] = 0b00011100;

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn toggle_cell(&mut self, row: u32, col: u32) {
      let cell = self.get_bit(row, col);
      self.set_cell_bit(row, col, 1 - cell);
    }

    pub fn set_glider(&mut self, row: u32, col: u32) {
      [(-1, 1), (0, -1), (0, 1), (1, 0), (1, 1)]
        .iter()
        .for_each(|(row_offset, col_offset)| {
          self.set_relative_bit(row, col, *row_offset, *col_offset, 1);
        });
    }

    pub fn set_pulsar(&mut self, row: u32, col: u32) {
      [(-6, 4), (-6, 3), (-6, 2), (-6, -2), (-6, -3), (-6, -4), (-4, 6), (-3, 6), (-2, 6), (2, 6),
        (3, 6), (4, 6), (6, 4), (6, 3), (6, 2), (6, -2), (6, -3), (6, -4), (4, -6), (3, -6),
        (2, -6), (-2, -6), (-3, -6), (-4, -6), (-4, -1), (-3, -1), (-2, -1), (-4, 1), (-3, 1),
        (-2, 1), (4, -1), (3, -1), (2, -1), (4, 1), (3, 1), (2, 1), (1, -4), (1, -3), (1, -2),
        (-1, -4), (-1, -3), (-1, -2), (1, 4), (1, 3), (1, 2), (-1, 4), (-1, 3), (-1, 2)]
        .iter()
        .for_each(|(row_offset, col_offset)| {
          self.set_relative_bit(row, col, *row_offset, *col_offset, 1);
        });
    }

    pub fn width(&self) -> u32 {
      self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    /// Set the width of the universe.
    ///
    /// Resets all cells to the dead state.
    pub fn set_width(&mut self, width: u32) {
      self.width = width;
      self.cells = (0..width * self.height).map(|_i| 0).collect();
  }

    /// Set the height of the universe.
    ///
    /// Resets all cells to the dead state.
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells = (0..self.width * height).map(|_i| 0).collect();
    }

    pub fn reset(&mut self) {
      let mut rng = rand::thread_rng();

      self.cells = (0..self.width * (self.height / 8))
          .map(|_i| {
              rng.gen_range(0..255)
          }).collect::<Vec<u8>>();
    }

    pub fn clear(&mut self) {
      self.cells = (0..self.width * (self.height / 8))
          .map(|_i| {
              0
          }).collect::<Vec<u8>>();
    }

    pub fn cells(&self) -> *const u8 {
        self.cells.as_ptr()
    }
}

impl Universe {
  fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
    let mut count = 0;

    for delta_row in [self.height - 1, 0, 1] {
      for delta_col in [self.width - 1, 0, 1] {
        if delta_row == 0 && delta_col == 0 {
          continue;
        }

        let neighbor_row = (row + delta_row) % self.height;
        let neighbor_col = (column + delta_col) % self.width;
        // let idx = self.get_index(neighbor_row, neighbor_col);
        count += self.get_bit(neighbor_row, neighbor_col);
      }
    }
    count
  }

  fn _get_relative_bit(&self, row: u32, col: u32, delta_row: i32, delta_col: i32) -> u8 {
    let delta_col = delta_col + self.width as i32;
    let delta_row = delta_row + self.height as i32;

    let neighbor_row = ((row as i32 + delta_row) % self.height as i32) as u32;
    let neighbor_col = ((col as i32 + delta_col) % self.width as i32) as u32;
    self.get_bit(neighbor_row, neighbor_col)
  }

  // gets the value of a bit at a given row and column
  fn get_bit(&self, row: u32, column: u32) -> u8 {
    // divide by 8 to find byte
    let idx = self.get_index(row, column);
    let byte = self.cells[idx/8];

    // modulo 8 to find index
    let bit = idx % 8;
    let mask = 1 << bit;

    // return
    (byte & mask) >> bit
  }

  // unused, but I want to keep it around in case I need it later
  fn _get_bit_by_index(&self, idx: u32) -> u8 {
    // divide by 8 to find byte
    let byte = self.cells[(idx/8) as usize];

    // modulo 8 to find index
    let bit = idx % 8;
    let mask = 1 << bit;

    // return
    (byte & mask) >> bit
  }

  fn set_relative_bit(&mut self, row: u32, col: u32, delta_row: i32, delta_col: i32, value: u8) -> u8 {
    let delta_col = delta_col + self.width as i32;
    let delta_row = delta_row + self.height as i32;

    let neighbor_row = ((row as i32 + delta_row) % self.height as i32) as u32;
    let neighbor_col = ((col as i32 + delta_col) % self.width as i32) as u32;
    self.set_cell_bit(neighbor_row, neighbor_col, value)
  }

  fn set_bit(&self, row: u32, column: u32, value: u8, array: &mut Vec<u8>) -> u8 {
    // divide by 8 to find byte
    let idx = self.get_index(row, column);
    let byte = array[idx/8];

    // modulo 8 to find index
    let bit = idx % 8;

    if value == 0 {
      let mask: u8 = !(1 << bit);
      array[idx/8] = byte & mask;
    }
    else {
      let mask: u8 = 1 << bit;
      array[idx/8] = byte | mask;
    }

    value
  }

  fn set_cell_bit(&mut self, row: u32, column: u32, value: u8) -> u8 {
    // divide by 8 to find byte
    let idx = self.get_index(row, column);
    let byte = self.cells[idx/8];

    // modulo 8 to find index
    let bit = idx % 8;

    if value == 0 {
      let mask: u8 = !(1 << bit);
      self.cells[idx/8] = byte & mask;
    }
    else {
      let mask: u8 = 1 << bit;
      self.cells[idx/8] = byte | mask;
    }

    value
  }

  fn get_index(&self, row: u32, column: u32) -> usize {
    (row * self.width + column) as usize
  }

  /// Get the dead and alive values of the entire universe.
  pub fn get_cells(&self) -> &[u8] {
      &self.cells
  }

  /// Set cells to be alive in a universe by passing the row and column
  /// of each cell as an array.
  pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
      for (row, col) in cells.iter().cloned() {
          self.set_cell_bit(row, col, 1);
      }
  }
}
