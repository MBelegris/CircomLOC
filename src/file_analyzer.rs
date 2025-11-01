use core::panic;
use std::fs::{File};
use std::io::{self, BufRead, BufReader};

pub enum Line {
    WhiteSpace(),
    Comment(),
    LongComment(),
    Code(),
}

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
        if line.trim().starts_with("**/"){
            (Line::LongComment(), false)
        } else if line.trim().is_empty(){
            (Line::WhiteSpace(), trailing_comment)
        } else if line.trim_start().starts_with("/**") {
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
