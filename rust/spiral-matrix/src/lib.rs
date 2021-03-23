use std::ops::Range;

pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let size = size as usize;

    let mut matrix = vec![vec![0u32; size]; size];

    let max_steps = size * size;

    let mut number_source = 1..=max_steps as u32;

    let mut coordinates = (0, 0);
    let mut velocity = (1, 0);

    let mut step = 0;
    let boundaries = 0..size;

    while step < max_steps {
        step += 1;

        let (x, y) = coordinates;
        if let Some(next) = number_source.next() {
            matrix[y][x] = next;
        }

        let mut maybe_new_coordinates: Option<(usize, usize)> = None;

        while maybe_new_coordinates.is_none()
            // On last step numbers are already assigned so turning is unnecessary.
            && step != max_steps
        {
            maybe_new_coordinates =
                calculate_new_valid_coordinates(&matrix, coordinates, velocity, &boundaries);
            match maybe_new_coordinates {
                Some(new_coordinates) => {
                    coordinates = new_coordinates;
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
    boundaries: &Range<usize>,
) -> Option<(usize, usize)> {
    let coordinates_candidate = (x as i32 + vx, y as i32 + vy);
    let (nx, ny) = coordinates_candidate;

    if boundaries.contains(&(nx as usize))
        && boundaries.contains(&(ny as usize))
        && matrix[ny as usize][nx as usize] == 0
    {
        Some((nx as usize, ny as usize))
    } else {
        None
    }
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
