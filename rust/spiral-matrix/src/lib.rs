pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let size = size as usize;

    let mut matrix = vec![vec![0u32; size]; size];

    let max_steps = size * size;

    let mut number_source = 1..=max_steps as u32;

    let mut coordinates = (0, 0);
    let mut velocity = (1, 0);

    let mut step = 0;

    while step < max_steps {
        step += 1;

        let (x, y) = coordinates;
        if let Some(next) = number_source.next() {
            matrix[y][x] = next;
        }

        let mut new_coordinates_found = false;
        while !new_coordinates_found
            // On last step numbers are already assigned so calculating new coordinates is unnecessary.
            && step != max_steps
        {
            match calculate_new_valid_coordinates(&matrix, coordinates, velocity) {
                Some(new_coordinates) => {
                    coordinates = new_coordinates;
                    new_coordinates_found = true;
                }
                None => {
                    velocity = change_direction_clockwise(velocity);
                }
            }
        }
    }
    matrix
}

fn calculate_new_valid_coordinates(
    matrix: &[Vec<u32>],
    (x, y): (usize, usize),
    (vx, vy): (i32, i32),
) -> Option<(usize, usize)> {
    let coordinates_candidate = (x as i32 + vx, y as i32 + vy);
    let (nx, ny) = coordinates_candidate;

    match get_position_value(matrix, nx, ny) {
        Some(0) => Some((nx as usize, ny as usize)),
        _ => None,
    }
}

fn get_position_value(matrix: &[Vec<u32>], x: i32, y: i32) -> Option<u32> {
    if x.is_negative() || y.is_negative() {
        return None;
    }

    matrix
        .get(y as usize)
        .and_then(|row| row.get(x as usize).copied())
}

fn change_direction_clockwise(velocity: (i32, i32)) -> (i32, i32) {
    match velocity {
        // right -> down
        (vx, _) if vx > 0 => (0, 1),
        // down -> left
        (_, vy) if vy > 0 => (-1, 0),
        // left -> up
        (vx, _) if vx < 0 => (0, -1),
        // up -> right
        (_, vy) if vy < 0 => (1, 0),
        (_, _) => unimplemented!("unsupported velocity: {:?}", velocity),
    }
}
