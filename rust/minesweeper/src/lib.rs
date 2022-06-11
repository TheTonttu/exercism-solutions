pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let height = minefield.len();
    let width = if let Some(row) = minefield.first() {
        row.len()
    } else {
        return vec![];
    };

    let mut annotated_minefield = vec![vec![' '; width]; height];

    for (y, row) in minefield.iter().enumerate() {
        for (x, element) in row.chars().enumerate() {
            if element == '*' {
                annotated_minefield[y][x] = '*';
            } else {
                let surrounding_mines_count = neighbors(x, y, width, height)
                    .map(|(nx, ny)| minefield[ny].chars().nth(nx))
                    .filter(|c| *c == Some('*'))
                    .count();

                let annotation = if surrounding_mines_count == 0 {
                    ' '
                } else {
                    char::from_digit(surrounding_mines_count as u32, 10).unwrap()
                };

                annotated_minefield[y][x] = annotation;
            }
        }
    }

    annotated_minefield
        .iter()
        .map(|row| row.iter().collect::<String>())
        .collect()
}

fn neighbors(
    x: usize,
    y: usize,
    width: usize,
    height: usize,
) -> impl Iterator<Item = (usize, usize)> {
    (x.max(1) - 1..=(x + 1).min(width - 1))
        .flat_map(move |nx| (y.max(1) - 1..=(y + 1).min(height - 1)).map(move |y| (nx, y)))
        .filter(move |&pos| pos != (x, y))
}
