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

    for row_index in 0..row_count {
        let maybe_prev_row = get_previous_row(&pascals_triangle, row_index);

        let row = create_row(row_index, maybe_prev_row);
        pascals_triangle.push(row);
    }

    pascals_triangle
}

fn get_previous_row(pascals_triangle: &[Vec<u32>], curr_row_index: u32) -> Option<&Vec<u32>> {
    let prev_row_index = (curr_row_index as i32) - 1;
    match prev_row_index {
        i if i >= 0 => pascals_triangle.get(i as usize),
        _ => None,
    }
}

fn create_row(row_index: u32, maybe_prev_row: Option<&Vec<u32>>) -> Vec<u32> {
    let element_count = 1 + row_index as usize;
    let mut row = Vec::with_capacity(element_count);
    match maybe_prev_row {
        Some(prev_row) => {
            for item_index in 0..element_count {
                let entries = get_above_entries(prev_row, item_index);
                let value = extract_value_from_entries(entries);
                row.push(value);
            }
        }
        None => row.push(1u32),
    }
    row
}

fn extract_value_from_entries(entries: (Option<u32>, Option<u32>)) -> u32 {
    match entries {
        (Some(left), Some(right)) => left + right,
        (Some(left), None) => left,
        (None, Some(right)) => right,
        _ => 0,
    }
}

fn get_above_entries(prev_row: &[u32], curr_item_index: usize) -> (Option<u32>, Option<u32>) {
    let left_entry_index = (curr_item_index as i32) - 1;
    let left_entry = match left_entry_index {
        i if i >= 0 => prev_row.get(i as usize).copied(),
        _ => None,
    };

    let right_entry = prev_row.get(curr_item_index).copied();
    (left_entry, right_entry)
}
