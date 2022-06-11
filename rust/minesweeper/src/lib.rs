const MINE: char = '*';

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let height = minefield.len();
    let width = minefield.first().map_or(0, |row| row.len());

    let mut annotated_minefield = vec![vec![' '; width]; height];
    for (x, y, cell_content) in cells(minefield) {
        if cell_content == MINE {
            annotated_minefield[y][x] = MINE;
        } else {
            let surrounding_mines_count = neighboring_mine_count(x, y, minefield);
            if surrounding_mines_count > 0 {
                let mine_count_annotation =
                    char::from_digit(surrounding_mines_count as u32, 10).unwrap();
                annotated_minefield[y][x] = mine_count_annotation;
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

fn neighboring_mine_count<'a>(x: usize, y: usize, minefield: &'a [&'a str]) -> usize {
    neighbors(x, y, minefield)
        .filter(|(_nx, _ny, cell_content)| *cell_content == MINE)
        .count()
}

fn neighbors<'a>(
    x: usize,
    y: usize,
    minefield: &'a [&'a str],
) -> impl Iterator<Item = (usize, usize, char)> + 'a {
    let height = minefield.len();
    let width = minefield.first().map_or(0, |row| row.len());

    neighbor_positions(x, y, width, height)
        .map(|(nx, ny)| (nx, ny, minefield[ny].chars().nth(nx).unwrap()))
}

fn neighbor_positions(
    x: usize,
    y: usize,
    width: usize,
    height: usize,
) -> impl Iterator<Item = (usize, usize)> {
    (x.max(1) - 1..=(x + 1).min(width - 1))
        .flat_map(move |nx| (y.max(1) - 1..=(y + 1).min(height - 1)).map(move |y| (nx, y)))
        .filter(move |&pos| pos != (x, y))
}
