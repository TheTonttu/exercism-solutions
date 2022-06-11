const MINE: char = '*';

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let height = minefield.len();
    let width = minefield.first().map_or(0, |row| row.len());

    let mut annotated_minefield = vec![vec![' '; width]; height];
    for (x, y, cell_content) in cells(minefield) {
        if cell_content == MINE {
            annotated_minefield[y][x] = MINE;
        } else {
            let surrounding_mines_count = neighbors(x, y, width, height)
                .map(|(nx, ny)| minefield[ny].chars().nth(nx))
                .filter(|c| *c == Some(MINE))
                .count();

            if surrounding_mines_count > 0 {
                let annotation = char::from_digit(surrounding_mines_count as u32, 10).unwrap();
                annotated_minefield[y][x] = annotation;
            }
        }
    }

    annotated_minefield
        .iter()
        .map(|row| row.iter().collect::<String>())
        .collect()
}

fn cells<'a>(minefield: &'a [&'a str]) -> impl Iterator<Item = (usize, usize, char)> + 'a {
    minefield.iter().enumerate().flat_map(move |(y, row)| {
        row.chars()
            .enumerate()
            .map(move |(x, cell_content)| (x, y, cell_content))
    })
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
