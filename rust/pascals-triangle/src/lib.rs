use std::borrow::BorrowMut;

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
        for _row_index in 0..row_item_count {
            row.push(1u32);
        }

        pascals_triangle.push(row);

        row_item_count += 1;
    }

    pascals_triangle
}
