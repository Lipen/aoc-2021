use std::fmt;

use crate::side::{Direction, Side};

#[derive(Debug)]
pub struct Tile {
    pub id: usize,
    pub data: Vec<Vec<char>>,
    side_top_inner: Vec<char>,
    side_bot_inner: Vec<char>,
    side_left_inner: Vec<char>,
    side_right_inner: Vec<char>,
    side_top_outer: Vec<char>,
    side_bot_outer: Vec<char>,
    side_left_outer: Vec<char>,
    side_right_outer: Vec<char>,
}

impl Tile {
    pub fn new(id: usize, data: Vec<Vec<char>>) -> Self {
        // // Note: side_left is outer, side_right is inner
        // let side_left = data.iter().map(|row| *row.first().unwrap()).collect();
        // let side_right = data.iter().map(|row| *row.last().unwrap()).collect();
        let side_top_inner = data.first().unwrap().clone();
        let side_top_outer = side_top_inner.iter().rev().copied().collect();
        let side_bot_outer = data.last().unwrap().clone();
        let side_bot_inner = side_bot_outer.iter().rev().copied().collect();
        let side_left_outer = data
            .iter()
            .map(|row| *row.first().unwrap())
            .collect::<Vec<_>>();
        let side_left_inner = side_left_outer.iter().rev().copied().collect();
        let side_right_inner = data
            .iter()
            .map(|row| *row.last().unwrap())
            .collect::<Vec<_>>();
        let side_right_outer = side_right_inner.iter().rev().copied().collect();
        Tile {
            id,
            data,
            side_top_inner,
            side_bot_inner,
            side_left_inner,
            side_right_inner,
            side_top_outer,
            side_bot_outer,
            side_left_outer,
            side_right_outer,
        }
    }

    pub fn from_multiline_str(id: usize, s: &str) -> Self {
        let data = s.lines().map(|line| line.chars().collect()).collect();
        eprintln!("data = {:?}", data);
        Tile::new(id, data)
    }

    pub fn data_on_side(&self, side: Side, dir: Direction) -> &[char] {
        use Direction::*;
        match dir {
            Inner => self.data_on_inner_side(side),
            Outer => self.data_on_outer_side(side),
        }
    }

    pub fn data_on_inner_side(&self, side: Side) -> &[char] {
        use Side::*;
        match side {
            Top => self.side_top_inner(),
            Bot => self.side_bot_inner(),
            Left => self.side_left_inner(),
            Right => self.side_right_inner(),
        }
    }
    pub fn data_on_outer_side(&self, side: Side) -> &[char] {
        use Side::*;
        match side {
            Top => self.side_top_outer(),
            Bot => self.side_bot_outer(),
            Left => self.side_left_outer(),
            Right => self.side_right_outer(),
        }
    }

    pub fn side_top_inner(&self) -> &[char] {
        &self.side_top_inner
    }
    pub fn side_bot_inner(&self) -> &[char] {
        &self.side_bot_inner
    }
    pub fn side_left_inner(&self) -> &[char] {
        &self.side_left_inner
    }
    pub fn side_right_inner(&self) -> &[char] {
        &self.side_right_inner
    }
    pub fn side_top_outer(&self) -> &[char] {
        &self.side_top_outer
    }
    pub fn side_bot_outer(&self) -> &[char] {
        &self.side_bot_outer
    }
    pub fn side_left_outer(&self) -> &[char] {
        &self.side_left_outer
    }
    pub fn side_right_outer(&self) -> &[char] {
        &self.side_right_outer
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Tile {}:", self.id)?;
        for row in self.data.iter() {
            let s = row.iter().collect::<String>();
            writeln!(f, "{}", s)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use itertools::Itertools;

    use super::*;

    #[test]
    fn test_sides() {
        let s = indoc! {"
            ..##.#..#.
            ##..#.....
            #...##..#.
            ####.#...#
            ##.##.###.
            ##...#.###
            .#.#.#..##
            ..#....#..
            ###...#.#.
            ..###..###
        "};
        let tile = Tile::from_multiline_str(0, s);
        assert_eq!(tile.side_top_inner(), "..##.#..#.".chars().collect_vec());
        assert_eq!(tile.side_bot_outer(), "..###..###".chars().collect_vec());
        assert_eq!(tile.side_left_outer(), ".#####..#.".chars().collect_vec());
        assert_eq!(tile.side_right_inner(), "...#.##..#".chars().collect_vec());
    }

    #[test]
    fn test_display() {
        let s = indoc! {"
            ..##.#..#.
            ##..#.....
            #...##..#.
            ####.#...#
            ##.##.###.
            ##...#.###
            .#.#.#..##
            ..#....#..
            ###...#.#.
            ..###..###
        "};
        let tile = Tile::from_multiline_str(42, s);
        let expected = format!("Tile 42:\n{}", s);
        assert_eq!(format!("{}", tile), expected);
    }
}
