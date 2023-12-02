#[derive(Debug)]
struct GameSet {
    blue: usize,
    red: usize,
    green: usize,
}

impl GameSet {
    fn parse(value: &str) -> Self {
        let mut set = GameSet {
            blue: 0,
            green: 0,
            red: 0,
        };

        value.trim().split(',').map(str::trim).for_each(|s| {
            let split: Vec<&str> = s.split(' ').collect();
            let num = split[0].parse::<usize>().unwrap();
            let color = split[1];

            match color {
                "blue" => set.blue = num,
                "red" => set.red = num,
                "green" => set.green = num,
                _ => panic!("unexpected color {}", color),
            }
        });

        set
    }
}

#[derive(Debug)]
struct Game {
    id: usize,
    sets: Vec<GameSet>,
}

impl Game {
    fn parse(value: &str) -> Self {
        let parts: Vec<&str> = value.trim().split(':').map(str::trim).collect();

        let id: usize = parts[0].split(' ').nth(1).unwrap().parse().unwrap();

        let sets: Vec<GameSet> = parts[1]
            .split(';')
            .map(str::trim)
            .map(GameSet::parse)
            .collect();

        Game { id, sets }
    }

    fn is_possible(&self, max_red: usize, max_green: usize, max_blue: usize) -> bool {
        let is_impossible = self
            .sets
            .iter()
            .any(|set| set.blue > max_blue || set.red > max_red || set.green > max_green);

        !is_impossible
    }

    fn power_of_minimum_set(&self) -> usize {
        let min_red = self.sets.iter().map(|game| game.red).max().unwrap();
        let min_blue = self.sets.iter().map(|game| game.blue).max().unwrap();
        let min_green = self.sets.iter().map(|game| game.green).max().unwrap();

        min_red * min_blue * min_green
    }
}

#[allow(dead_code)]
pub fn solve1() {
    let file_contents = include_str!("input.txt");

    let game_ids: Vec<usize> = file_contents
        .split('\n')
        .map(Game::parse)
        .filter(|game| game.is_possible(12, 13, 14))
        .map(|game| game.id)
        .collect();

    let sum = game_ids.iter().sum::<usize>();

    println!("Answer: {}", sum);
}

#[allow(dead_code)]
pub fn solve2() {
    let file_contents = include_str!("input.txt");

    let powers: Vec<usize> = file_contents
        .split('\n')
        .map(Game::parse)
        .map(|game| game.power_of_minimum_set())
        .collect();

    let sum = powers.iter().sum::<usize>();

    println!("Answer: {}", sum);
}
