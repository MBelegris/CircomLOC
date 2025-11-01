pub mod file_analyzer;

use file_analyzer::{FileSizeAnalyzer};
use std::env;
use std::{fs, io};

use crate::file_analyzer::*;

fn main() {
    // cargo run -- <file-or-directory-path>

    // Specify and read lines from file path
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please provide a file path as an argument.");
    }  
    let path = &args[1];

    let directory = read_dir(path);
    match directory {
        Ok(dir) => {
            for entry in dir {
                let entry = entry.unwrap();
                let path = entry.path();
                
                let mut counts: Vec<Count> = vec![];
                let mut paths = vec![];

                if path.is_file() {
                    let path_str = path.to_str().unwrap();
                    
                    if path_str.ends_with(".circom") == false {
                        continue;
                    }

                    let mut analyzer = FileSizeAnalyzer::new(path_str.to_string());
                    analyzer.analyze();

                    assert!(
                        analyzer.count.total_lines == 
                        analyzer.count.white_lines + analyzer.count.comment_lines + 
                        analyzer.count.long_comment_lines + analyzer.count.code_lines);

                    counts.push(analyzer.count);
                    paths.push(path_str.to_string());
                }
                print_summary(&paths, &counts);
            }
        }

        Err(_) => {
            // Not a directory, treat as a single file
            let mut analyzer = FileSizeAnalyzer::new(path.to_string());
            analyzer.analyze();

            assert!(
                analyzer.count.total_lines == 
                analyzer.count.white_lines + analyzer.count.comment_lines + 
                analyzer.count.long_comment_lines + analyzer.count.code_lines);

            analyzer.print_single_summary();
        }
    }

}

fn read_dir(path: &str) -> Result<fs::ReadDir, io::Error> {
    fs::read_dir(path)
}

pub fn print_summary(paths: &Vec<String>, counts: &Vec<Count>) {
    // Define table column widths
    let file_width = 30;
    let num_width = 12;
    let total_width = file_width + 5 * (num_width + 3) + 1; // for separators

    // Print header separator
    println!("{}", "-".repeat(total_width));

    // Print header row
    println!(
        "| {:<file_width$} | {:>num_width$} | {:>num_width$} | {:>num_width$} | {:>num_width$} | {:>num_width$} |",
        "File",
        "Total",
        "White",
        "Comments",
        "Long Cmt",
        "Code",
        file_width = file_width,
        num_width = num_width
    );

    // Print header separator
    println!("{}", "-".repeat(total_width));

    // Print each file’s counts
    for (path, count) in paths.iter().zip(counts.iter()) {
        println!(
            "| {:<file_width$} | {:>num_width$} | {:>num_width$} | {:>num_width$} | {:>num_width$} | {:>num_width$} |",
            path,
            count.total_lines,
            count.white_lines,
            count.comment_lines,
            count.long_comment_lines,
            count.code_lines,
            file_width = file_width,
            num_width = num_width
        );
    }

    // Print row separator
    println!("{}", "-".repeat(total_width));

    // Calculate totals
    let total = counts.iter().fold(
        Count {
            total_lines: 0,
            white_lines: 0,
            comment_lines: 0,
            long_comment_lines: 0,
            code_lines: 0,
        },
        |mut acc, c| {
            acc.total_lines += c.total_lines;
            acc.white_lines += c.white_lines;
            acc.comment_lines += c.comment_lines;
            acc.long_comment_lines += c.long_comment_lines;
            acc.code_lines += c.code_lines;
            acc
        },
    );

    // Print total summary row
    println!(
        "| {:<file_width$} | {:>num_width$} | {:>num_width$} | {:>num_width$} | {:>num_width$} | {:>num_width$} |",
        "TOTAL",
        total.total_lines,
        total.white_lines,
        total.comment_lines,
        total.long_comment_lines,
        total.code_lines,
        file_width = file_width,
        num_width = num_width
    );

    // Final separator
    println!("{}", "-".repeat(total_width));
}
