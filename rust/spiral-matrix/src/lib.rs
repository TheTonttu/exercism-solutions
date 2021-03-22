pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let size = size as usize;

    let mut matrix = vec![vec![0u32; size]; size];

    let move_count = size * size;
    let mut nums = 1..=move_count as u32;

    let mut velocity = (1, 0);

    let mut coordinates = (0i32, 0i32);

    let mut i = 0;
    while i < move_count {
        i += 1;

        let (x, y) = coordinates;
        let row = &mut matrix[y as usize];
        if let Some(next) = nums.next() {
            row[x as usize] = next;
        }

        let mut new_coordinates;
        loop {
            new_coordinates = next_coordinates(coordinates, velocity);
            let (nx, ny) = new_coordinates;

            let boundaries = 0..size;
            if boundaries.contains(&(nx as usize))
                && boundaries.contains(&(ny as usize))
                && matrix[ny as usize][nx as usize] == 0
                || i == move_count
            {
                break;
            }
            velocity = turn(velocity);
        }
        coordinates = new_coordinates;
    }
    matrix
}

fn next_coordinates((x, y): (i32, i32), (vx, vy): (i32, i32)) -> (i32, i32) {
    (x + vx, y + vy)
}

fn turn(velocity: (i32, i32)) -> (i32, i32) {
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
