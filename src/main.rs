use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let report = read_lines("./input")
        .unwrap()
        .filter_map(|line| line.ok())
        .filter_map(|str| str.parse::<i32>().ok())
        .collect();

    let magic_expense = match args.get(1).map(|s| s.as_str()) {
        None => { find_magic_expense2 }
        Some("2") => { find_magic_expense2 }
        Some("3") => { find_magic_expense3 }
        Some(_) => { panic!("Invalid argument") }
    };

    if let Some(expense) = magic_expense(report, 2020) {
        println!("Your expense is: {}", expense);
    } else {
        println!("Expense not found");
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn find_magic_expense2(expense_report: Vec<i32>, year: i32) -> Option<i32> {
    find_pair_for_sum(expense_report, year).map(|(a, b)| a * b)
}

fn find_pair_for_sum(list: Vec<i32>, sum: i32) -> Option<(i32, i32)> {
    find_sum_in_pairs(create_pairs(list), sum)
}

fn find_sum_in_pairs(list: Vec<(i32, i32)>, sum: i32) -> Option<(i32, i32)> {
    list.iter()
        .filter(|(a, b)| a + b == sum)
        .map(|(a, b)| (*a, *b))
        .next()
}

fn create_pairs(list: Vec<i32>) -> Vec<(i32, i32)> {
    let mut pairs = Vec::new();

    let mut i = 0;
    for first in list.iter() {
        for second in &list[i + 1..] {
            pairs.push((*first, *second));
        }
        i = i + 1;
    }

    pairs
}


fn find_magic_expense3(expense_report: Vec<i32>, year: i32) -> Option<i32> {
    find_triplet_for_sum(expense_report, year).map(|(a, b, c)| a * b * c)
}

fn find_triplet_for_sum(list: Vec<i32>, sum: i32) -> Option<(i32, i32, i32)> {
    find_sum_in_triplets(create_triplets(list), sum)
}

fn find_sum_in_triplets(list: Vec<(i32, i32, i32)>, sum: i32) -> Option<(i32, i32, i32)> {
    list.iter()
        .filter(|(a, b, c)| a + b + c == sum)
        .map(|(a, b, c)| (*a, *b, *c))
        .next()
}

fn create_triplets(list: Vec<i32>) -> Vec<(i32, i32, i32)> {
    let mut triplets = Vec::new();

    let mut i = 0;
    for first in list.iter() {
        let mut j = i + 1;
        for second in &list[i + 1..] {
            for third in &list[j + 1..] {
                triplets.push((*first, *second, *third));
            }
            j = j + 1;
        }
        i = i + 1;
    }

    triplets
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_pairs() {
        assert_eq!(create_pairs(vec![]), vec![]);
        assert_eq!(create_pairs(vec![10]), vec![]);
        assert_eq!(create_pairs(vec![10, 20]), vec![(10, 20)]);
        assert_eq!(create_pairs(vec![10, 20, 30]), vec![(10, 20), (10, 30), (20, 30)]);
    }

    #[test]
    fn test_find_sum_in_pairs() {
        assert_eq!(find_sum_in_pairs(vec![], 30), None);
        assert_eq!(find_sum_in_pairs(vec![(10, 20)], 30), Some((10, 20)));
        assert_eq!(find_sum_in_pairs(vec![(10, 30)], 30), None);
    }

    #[test]
    fn test_find_pair_for_sum() {
        assert_eq!(find_pair_for_sum(vec![], 2020), None);
        assert_eq!(find_pair_for_sum(vec![1721, 979, 366], 2020), None);
        assert_eq!(find_pair_for_sum(vec![1721, 979, 366, 299, 675, 1456], 2020), Some((1721, 299)));
    }

    #[test]
    fn test_find_magic_expense2() {
        assert_eq!(find_magic_expense2(vec![], 2020), None);
        assert_eq!(find_magic_expense2(vec![1721, 979, 366], 2020), None);
        assert_eq!(find_magic_expense2(vec![1721, 979, 366, 299, 675, 1456], 2020), Some(514579));
    }

    #[test]
    fn test_create_triplets() {
        assert_eq!(create_triplets(vec![]), vec![]);
        assert_eq!(create_triplets(vec![10]), vec![]);
        assert_eq!(create_triplets(vec![10, 20]), vec![]);
        assert_eq!(create_triplets(vec![10, 20, 30]), vec![(10, 20, 30)]);
        assert_eq!(create_triplets(vec![10, 20, 30, 40, 50]), vec![
            (10, 20, 30),
            (10, 20, 40),
            (10, 20, 50),
            (10, 30, 40),
            (10, 30, 50),
            (10, 40, 50),
            (20, 30, 40),
            (20, 30, 50),
            (20, 40, 50),
            (30, 40, 50)
        ]);
    }

    #[test]
    fn test_find_sum_in_triplets() {
        assert_eq!(find_sum_in_triplets(vec![], 30), None);
        assert_eq!(find_sum_in_triplets(vec![(10, 20, 30)], 60), Some((10, 20, 30)));
        assert_eq!(find_sum_in_triplets(vec![(10, 20, 40)], 60), None);
    }

    #[test]
    fn test_find_triplet_for_sum() {
        assert_eq!(find_triplet_for_sum(vec![], 2020), None);
        assert_eq!(find_triplet_for_sum(vec![1721, 979, 366], 2020), None);
        assert_eq!(find_triplet_for_sum(vec![1721, 979, 366, 299, 675, 1456], 2020), Some((979, 366, 675)));
    }

    #[test]
    fn test_find_magic_expense3() {
        assert_eq!(find_magic_expense3(vec![], 2020), None);
        assert_eq!(find_magic_expense3(vec![1721, 979, 366], 2020), None);
        assert_eq!(find_magic_expense3(vec![1721, 979, 366, 299, 675, 1456], 2020), Some(241861950));
    }

}
