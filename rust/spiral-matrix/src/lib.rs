pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let size = size as usize;

    let mut matrix = vec![vec![0u32; size]; size];

    let max_steps = size * size;
    let mut number_source = 1..=max_steps as u32;

    let mut velocity = (1, 0);

    let mut coordinates = (0i32, 0i32);

    let mut step = 0;
    while step < max_steps {
        step += 1;

        let (x, y) = coordinates;
        if let Some(next) = number_source.next() {
            matrix[y as usize][x as usize] = next;
        }

        let mut new_coordinates;
        let boundaries = 0..size;

        loop {
            new_coordinates = calculate_new_coordinates(coordinates, velocity);
            let (nx, ny) = new_coordinates;

            if boundaries.contains(&(nx as usize))
                && boundaries.contains(&(ny as usize))
                && matrix[ny as usize][nx as usize] == 0
                // On last step numbers are already assigned so turning is unnecessary.
                || step == max_steps
            {
                break;
            }
            velocity = change_direction_clockwise(velocity);
        }
        coordinates = new_coordinates;
    }
    matrix
}

fn calculate_new_coordinates((x, y): (i32, i32), (vx, vy): (i32, i32)) -> (i32, i32) {
    (x + vx, y + vy)
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
