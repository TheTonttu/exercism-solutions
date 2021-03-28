use std::cmp::Ordering;

pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut saddle_points = Vec::new();

    for (row_index, row) in input.iter().enumerate().filter(|(_, row)| !row.is_empty()) {
        let (row_max, max_col_indexes) = row.iter().enumerate().skip(1).fold(
            (row[0], vec![0]),
            |(mut max_value, mut max_value_indexes), (col_index, col_value)| {
                match max_value.cmp(&col_value) {
                    Ordering::Less => {
                        max_value_indexes.clear();
                        max_value = *col_value;
                        max_value_indexes.push(col_index);
                    }
                    Ordering::Equal => {
                        max_value_indexes.push(col_index);
                    }
                    Ordering::Greater => {}
                }
                (max_value, max_value_indexes)
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
