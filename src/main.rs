use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn main() {
    println!("Advent of Code 2024!");


    let day_1_input_file = "input/day1/input";

    match total_distance(day_1_input_file) {
        Ok((distance, similarity)) => {
            println!("Total distance: {}", distance);
            println!("Similarity: {}", similarity)
        },
        Err(e) => eprintln!("Error encountered in file {}",e)
    }

    let day_2_input_file = "input/day2/input";

    match count_safe_reports(day_2_input_file) {
        Ok(count) => println!("Safe reports: {}", count),
        Err(e) => eprintln!("Error encountered in file {}", e)
    }
}

fn total_distance(file_path: &str) -> io::Result<(i32, i32)> {
    let mut column1 = Vec::new();
    let mut column2 = Vec::new();

    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(record) = line {
                let parts: Vec<&str> = record.split_whitespace().collect();
                if parts.len() == 2 {
                    if let (Ok(num1), Ok(num2)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
                        column1.push(num1);
                        column2.push(num2);
                    }
                }
            } 
        }
    }

    column1.sort_unstable();
    column2.sort_unstable();

    //part 1
    let total_distance: i32 = column1
        .iter()
        .zip(column2.iter())
        .map(|(c1, c2)| (c1 - c2).abs())
        .sum();

    //part 2
    let mut column2_counts = HashMap::new();
    for &num in &column2 {
        *column2_counts.entry(num).or_insert(0) += 1;
    }

    let similarity_score: i32 = column1
        .iter()
        .map(|&c1| column2_counts.get(&c1).cloned().unwrap_or(0)*c1)
        .sum();

    
    Ok((total_distance, similarity_score))
}

//day 2 - part 1
fn count_safe_reports(file_path: &str) -> io::Result<usize> {
    match read_lines(file_path) {
        Ok(lines) => Ok(lines.filter_map(Result::ok)
                        .map(|line| line.split_whitespace()
                        .filter_map(|s| s.parse::<i32>().ok()).collect())
                        .filter(|report| is_safe_report(report)).count()),
        Err(e) => Err(e)

    }
}

fn is_safe_report(report: &Vec<i32>) -> bool {
    report.windows(2)
        .all(|w| w[0] < w[1] && (w[0]+4) > w[1]) ||
        report.windows(2)
            .all(|w| w[0] > w[1] && (w[0]-4) < w[1])
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
