use std::borrow::BorrowMut;
use std::num::Wrapping;

pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        Self {
            rows: create_pascals_triangle(row_count),
        }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.rows.clone()
    }
}

fn create_pascals_triangle(row_count: u32) -> Vec<Vec<u32>> {
    let mut pascals_triangle: Vec<Vec<u32>> = Vec::with_capacity(row_count as usize);

    let mut row_item_count = 1;
    for i in 0..row_count {
        let mut row = Vec::with_capacity(row_item_count);

        let maybe_prev_row = get_previous_row(&pascals_triangle, i);

        match maybe_prev_row {
            Some(prev_row) => {
                for item_index in 0..row_item_count {
                    row.push(1u32);
                }
            },
            None => row.push(1u32)
        }
        pascals_triangle.push(row);

        row_item_count += 1;
    }

    pascals_triangle
}

fn get_previous_row(pascals_triangle: &Vec<Vec<u32>>, curr_row_index: u32) -> Option<&Vec<u32>> {
    let prev_row_index = (curr_row_index as i32) - 1;
    match prev_row_index {
        i if i.is_positive() || i == 0 => pascals_triangle.get(i as usize),
        i if i.is_negative() => None,
        _ => None
    }
}
