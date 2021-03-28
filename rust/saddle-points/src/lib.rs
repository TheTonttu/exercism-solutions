pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut saddle_points = Vec::new();

    for row_index in 0..input.len() {
        if input[row_index].is_empty() {
            continue;
        }

        let mut row_max = input[row_index][0];
        let mut max_col_indexes = vec![0];
        for col_index in 1..input[row_index].len() {
            let curr_value = input[row_index][col_index];
            if row_max < curr_value {
                max_col_indexes.clear();
                row_max = curr_value;
                max_col_indexes.push(col_index);
            } else if row_max == curr_value {
                max_col_indexes.push(col_index);
            }
        }

        'index_check: for max_col_index in &max_col_indexes {
            let mut check_row_index = 0;
            for derp_row_index in 0..input.len() {
                check_row_index = derp_row_index;
                let curr_value = input[derp_row_index][*max_col_index];
                if row_max > curr_value {
                    continue 'index_check;
                }
            }

            saddle_points.push((row_index, *max_col_index));
        }
    }

    saddle_points
}
