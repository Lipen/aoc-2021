use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

use anyhow::Result;
use itertools::Itertools;
use vec_map::VecMap;

#[derive(Debug)]
struct SpokenNumbers {
    storage: VecMap<usize>,
    step: usize,
    last: usize,
}

impl SpokenNumbers {
    fn new(data: &[usize]) -> Self {
        SpokenNumbers::with_capacity(&data, data.len())
    }

    fn with_capacity(data: &[usize], capacity: usize) -> Self {
        assert!(data.len() > 0);
        let mut storage = VecMap::with_capacity(capacity.max(data.len()));
        let mut last = 0;

        for (i, &x) in data.iter().enumerate() {
            if i > 0 {
                storage.insert(last, i - 1);
            }
            last = x;
        }

        SpokenNumbers {
            storage,
            step: data.len(),
            last,
        }
    }

    fn push(&mut self, value: usize) {
        self.storage.insert(self.last, self.step - 1);
        self.last = value;
        self.step += 1;
    }
}

impl Iterator for SpokenNumbers {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        assert!(self.step > 0);

        let age = if let Some(index) = self.storage.get(self.last) {
            self.step - 1 - index
        } else {
            0
        };
        self.push(age);

        Some(self.last)
    }
}

fn spoken_number(data: &[usize], n: usize) -> usize {
    // Note: `n` is 1-based
    assert!(data.len() > 0);
    assert!(n > data.len());

    let mut s = SpokenNumbers::with_capacity(&data, n);
    s.nth(n - 1 - data.len()).unwrap()
}

fn solve(data: &[usize], steps: usize) {
    let last = spoken_number(&data, steps);
    println!(
        "Last spoken number for {:?} after {} steps is {}",
        data, steps, last
    );
}

fn main() -> Result<()> {
    let start_time = Instant::now();

    let path = "data/input.txt";
    // let path = "data/sample.txt"; // 436
    let mut lines = BufReader::new(File::open(path)?).lines();
    let data: Vec<_> = lines
        .next()
        .unwrap()?
        .split(',')
        .map(|x| x.parse::<usize>())
        .try_collect()?;
    println!("[debug] {:?}", data);

    println!("Solving part 1...");
    solve(&data, 2020);
    println!();
    println!("Solving part 2...");
    solve(&data, 30_000_000);

    println!("All done in {:.2} s", start_time.elapsed().as_secs_f32());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_036() {
        let data = vec![0, 3, 6];
        let res10 = spoken_number(&data, 10);
        assert_eq!(res10, 0);
        let res2020 = spoken_number(&data, 2020);
        assert_eq!(res2020, 436);
    }

    #[test]
    fn test_sample_132() {
        let data = vec![1, 3, 2]; // 1
        let res = spoken_number(&data, 2020);
        assert_eq!(res, 1);
    }

    #[test]
    fn test_sample_213() {
        let data = vec![2, 1, 3]; // 10
        let res = spoken_number(&data, 2020);
        assert_eq!(res, 10);
    }

    #[test]
    fn test_sample_123() {
        let data = vec![1, 2, 3]; // 27
        let res = spoken_number(&data, 2020);
        assert_eq!(res, 27);
    }

    #[test]
    fn test_sample_231() {
        let data = vec![2, 3, 1]; // 78
        let res = spoken_number(&data, 2020);
        assert_eq!(res, 78);
    }

    #[test]
    fn test_sample_321() {
        let data = vec![3, 2, 1]; // 438
        let res = spoken_number(&data, 2020);
        assert_eq!(res, 438);
    }

    #[test]
    fn test_sample_312() {
        let data = vec![3, 1, 2]; // 1836
        let res = spoken_number(&data, 2020);
        assert_eq!(res, 1836);
    }

    #[test]
    fn test_my_2020() {
        let data = vec![2, 20, 0, 4, 1, 17];
        let res = spoken_number(&data, 2020);
        assert_eq!(res, 758);
    }

    #[test]
    fn test_my_30kk() {
        let data = vec![2, 20, 0, 4, 1, 17];
        let res = spoken_number(&data, 30_000_000);
        assert_eq!(res, 814);
    }

    #[test]
    fn test_danvk_2020() {
        let data = vec![0, 20, 7, 16, 1, 18, 15];
        let res = spoken_number(&data, 2020);
        assert_eq!(res, 1025);
    }

    #[test]
    fn test_danvk_30kk() {
        let data = vec![0, 20, 7, 16, 1, 18, 15];
        let res = spoken_number(&data, 30_000_000);
        assert_eq!(res, 129262);
    }
}
