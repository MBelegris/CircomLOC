use core::panic;
use std::fs::{File};
use std::io::{self, BufRead, BufReader};
use std::cmp::max;

pub enum Line {
    WhiteSpace(),
    Comment(),
    LongComment(),
    Code(),
}

#[derive(Clone, Copy, Debug)]
pub struct Count {
    pub total_lines: usize,
    pub white_lines: usize,
    pub comment_lines: usize,
    pub long_comment_lines: usize,
    pub code_lines: usize,
}

impl Count {
    fn print_summary(&self) {
        println!("+--------------------+-------+");
        println!("| Metric             | Count |");
        println!("+--------------------+-------+");
        println!("| Total lines        | {:>5} |", self.total_lines);
        println!("| White lines        | {:>5} |", self.white_lines);
        println!("| Comment lines      | {:>5} |", self.comment_lines);
        println!("| Long comment lines | {:>5} |", self.long_comment_lines);
        println!("| Code lines         | {:>5} |", self.code_lines);
        println!("+--------------------+-------+");
    }
}

#[derive(Clone, Debug)]
pub struct Analysis {
    pub file_size_analyzers: Vec<FileSizeAnalyzer>,
}

impl Analysis {
    pub fn new() -> Self {
        Analysis {
            file_size_analyzers: vec![],
        }
    }

    pub fn add_analyzer(&mut self, analyzer: FileSizeAnalyzer) {
        self.file_size_analyzers.push(analyzer);
    }

    pub fn set_analyzers(&mut self, analyzers: Vec<FileSizeAnalyzer>) {
        self.file_size_analyzers = analyzers;
    }

    pub fn print_summary_as_singles(&self) {
        for analyzer in &self.file_size_analyzers {
            analyzer.print_single_summary();
        }
    }

    pub fn print_summary_as_table(&self) {
        let paths: Vec<String> = self.file_size_analyzers.iter().map(|a| a.path.clone()).collect();
        let counts: Vec<Count> = self.file_size_analyzers.iter().map(|a| a.count).collect();
        Self::print_summary(&paths, &counts);
    }

fn print_summary(paths: &Vec<String>, counts: &Vec<Count>) {
        // Compute file column width dynamically based on the longest file path
        // (ensure it's at least wide enough for the "File" header and a practical minimum)
        let longest_path = paths.iter().map(|p| p.len()).max().unwrap_or(0);
        let file_width = max(18, max("File".len(), longest_path));

        // Keep numeric columns fixed (can also be made dynamic if you want)
        let num_width = 12;

        // Total width for the dashed separators
        // Pattern per numeric column is " | {:>num_width$} " => num_width + 3 chars
        // File column is " | {:<file_width$} " => file_width + 3 chars
        // Plus the leading '|' and trailing '|' => +2
        let total_width = (file_width + 3) + 5 * (num_width + 3) + 2;

        // Header separator
        println!("{}", "-".repeat(total_width));

        // Header row
        println!(
            "| {:<file_width$} | {:>num_width$} | {:>num_width$} | {:>num_width$} | {:>num_width$} | {:>num_width$} |",
            "File", "Total", "White", "Comments", "Long Cmt", "Code",
            file_width = file_width, num_width = num_width
        );

        // Header separator
        println!("{}", "-".repeat(total_width));

        // Rows
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

        // Row separator
        println!("{}", "-".repeat(total_width));

        // Totals
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

        // Totals row
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
}

#[derive(Clone, Debug)]
pub struct FileSizeAnalyzer {
    pub path: String,
    pub count: Count,
}

impl Default for FileSizeAnalyzer {
    fn default() -> Self {
        FileSizeAnalyzer {
            path: String::new(),
            count: Count {
                total_lines: 0,
                white_lines: 0,
                comment_lines: 0,
                long_comment_lines: 0,
                code_lines: 0,
            },
        }
    }
}

impl FileSizeAnalyzer {
    pub fn new(path: String) -> Self {
        FileSizeAnalyzer {
            path,
            ..Default::default()
        }
    }

    pub fn analyze(&mut self) {
        // Implementation for analyzing file size and counting lines
        let lines: io::Lines<BufReader<File>> =  match Self::read_lines(&self.path.as_str()) {
            Ok(x) => x,
            Err(e) => {
                panic!("Error reading file: {} at path {}", e, self.path);
            }
        };
        self.count = Self::count_lines(lines);
    }

    pub fn print_single_summary(&self) {
        println!("File Summary for {}:", self.path);
        self.count.print_summary();
    }

    fn count_lines(lines: io::Lines<BufReader<File>>) -> Count {
        let mut total_lines = 0;
        let mut white_lines = 0;
        let mut comment_lines = 0;
        let mut long_comment_lines = 0;
        let mut code_lines = 0;
        let mut trailing_comment = false;

        lines.into_iter().for_each(
            |line| {
                match line {
                    Ok(content) => {
                        let (line_type, is_trailing) = Self::line_processor(&content, trailing_comment);
                        trailing_comment = is_trailing;
                        match line_type {
                            Line::WhiteSpace() => white_lines += 1,
                            Line::Comment() => comment_lines += 1,
                            Line::LongComment() => long_comment_lines += 1,
                            Line::Code() => code_lines += 1,
                        }
                        total_lines += 1;
                    }
                    Err(e) => {
                        panic!("Error reading line: {}", e);
                    }
                }
            }
        );

        Count {
            total_lines,
            white_lines,
            comment_lines,
            long_comment_lines,
            code_lines,
        }
    }

    fn read_lines(path: &str) -> Result<std::io::Lines<BufReader<File>>, io::Error> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        Ok(reader.lines())
    }

    fn line_processor(line: &str, trailing_comment: bool) -> (Line, bool) { 
        if line.trim().starts_with("**/") || line.trim().starts_with("*/"){
            (Line::LongComment(), false)
        } else if line.trim().is_empty(){
            (Line::WhiteSpace(), trailing_comment)
        } else if line.trim_start().starts_with("/**"){
            (Line::LongComment(), true)
        } else if trailing_comment{
            (Line::LongComment(), true)
        } else {
            if line.trim_start().starts_with("//") {
                (Line::Comment(), false)
            } else {
                (Line::Code(), false)
            }
        }
    }

}
