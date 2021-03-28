use std::cmp::Ordering;

pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut saddle_points = Vec::new();

    for (row_index, row) in input.iter().enumerate().filter(|(_, row)| !row.is_empty()) {
        let (row_max, max_col_indexes) = row.iter().enumerate().skip(1).fold(
            (row[0], vec![0]),
            |(mut row_max, mut indexes), (col_index, curr_value)| {
                match row_max.cmp(&curr_value) {
                    Ordering::Less => {
                        indexes.clear();
                        row_max = *curr_value;
                        indexes.push(col_index);
                    }
                    Ordering::Equal => {
                        indexes.push(col_index);
                    }
                    Ordering::Greater => {}
                }
                (row_max, indexes)
            },
        );

        for col_index in &max_col_indexes {
            if is_min_column_value(&row_max, input, col_index) {
                saddle_points.push((row_index, *col_index));
            }
        }
    }

    saddle_points
}

fn is_min_column_value(value: &u64, matrix: &[Vec<u64>], column_index: &usize) -> bool {
    !matrix.iter().any(|row| *value > row[*column_index])
}
