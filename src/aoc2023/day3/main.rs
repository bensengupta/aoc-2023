use std::collections::{HashMap, HashSet};

fn is_adjacent_to_symbol(grid: &Vec<Vec<char>>, r: usize, c: usize) -> bool {
    for (dr, dc) in [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
    ] {
        let (row, col) = (r as i32 + dr, c as i32 + dc);

        if row < 0 || row >= grid.len() as i32 || col < 0 || col >= grid[0].len() as i32 {
            continue;
        }
        let ch = grid[row as usize][col as usize];

        if ch.is_ascii_digit() || ch == '.' {
            continue;
        }

        return true;
    }
    return false;
}

#[allow(dead_code)]
pub fn solve1() {
    let file_contents = include_str!("input.txt");

    let mut grid: Vec<Vec<char>> = file_contents
        .split('\n')
        .map(|line| line.chars().collect())
        .collect();

    // Used later on for parsing integers
    for r in 0..grid.len() {
        grid[r].push('.');
    }

    let mut parsed_nums: Vec<u32> = Vec::new();
    let mut current_num = 0;
    let mut is_part = false;

    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c].is_ascii_digit() {
                current_num = current_num * 10 + grid[r][c].to_digit(10).unwrap();
                is_part = is_part || is_adjacent_to_symbol(&grid, r, c);
                continue;
            }
            if current_num == 0 {
                continue;
            }

            if is_part {
                parsed_nums.push(current_num);
            }

            is_part = false;
            current_num = 0;
        }
    }

    let sum = parsed_nums.iter().sum::<u32>();

    println!("Answer: {}", sum);
}

#[allow(dead_code)]
pub fn solve2() {
    let file_contents = include_str!("input.txt");

    let mut grid: Vec<Vec<char>> = file_contents
        .split('\n')
        .map(|line| line.chars().collect())
        .collect();

    // Used later on for parsing integers
    for r in 0..grid.len() {
        grid[r].push('.');
    }

    let mut id_num_map: HashMap<usize, usize> = HashMap::new();
    let mut id_grid: Vec<Vec<Option<usize>>> = vec![vec![None; grid[0].len()]; grid.len()];

    let mut id = 0;
    let mut current_num = 0;

    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c].is_ascii_digit() {
                id_grid[r][c] = Some(id);
                current_num = current_num * 10 + grid[r][c].to_digit(10).unwrap() as usize;
                continue;
            }
            if current_num > 0 {
                id_num_map.insert(id, current_num);
                id += 1;
            }
            current_num = 0;
        }
    }

    let mut sum = 0;

    for (r, row) in grid.iter().enumerate() {
        for (c, ch) in row.iter().enumerate() {
            if *ch != '*' {
                continue;
            }

            let mut adjacent_ids: HashSet<usize> = HashSet::new();
            for (dr, dc) in [
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, 1),
                (1, 1),
                (1, 0),
                (1, -1),
                (0, -1),
            ] {
                let row = r as i32 + dr;
                let col = c as i32 + dc;
                if row < 0 || row >= grid.len() as i32 || col < 0 || col >= grid[0].len() as i32 {
                    continue;
                }

                if let Some(id) = id_grid[row as usize][col as usize] {
                    adjacent_ids.insert(id);
                }
            }

            if adjacent_ids.len() == 2 {
                sum += adjacent_ids
                    .iter()
                    .map(|id| id_num_map.get(id).unwrap())
                    .product::<usize>();
            }
        }
    }

    println!("Answer: {}", sum);
}
