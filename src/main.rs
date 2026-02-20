pub mod file_analyzer;
pub mod args;

use file_analyzer::{FileSizeAnalyzer};
use walkdir::{DirEntry, WalkDir};

use crate::file_analyzer::*;
use crate::args::Args;

fn main() {
    // cargo run -- <file-or-directory-path>
    // default is current dir

    let args: Args = Args::parse();
    let path = &args.circom_dir;

    let walker = WalkDir::new(path).into_iter();

    let mut analyzers: Vec<FileSizeAnalyzer> = vec![];

    walker.filter_map(|e| e.ok()).filter(only_noir_files).enumerate().for_each(
        |(_i, entry)| {
            let path = entry.path();
            let path_str = path.to_str().unwrap();

            let mut analyzer = FileSizeAnalyzer::new(path_str.to_string());
            analyzer.analyze();

            analyzers.push(analyzer);
        }
    );

    if analyzers.len() == 0 {
        println!("No .circom files found in the specified path.");
        return;
    } else if analyzers.len() == 1 {
        analyzers[0].print_single_summary();
        return;
    } else {
        let analysis = Analysis {
            file_size_analyzers: analyzers,
        };
        analysis.print_summary_as_table();
    }
}

fn only_circom_files(entry: &DirEntry) -> bool {
    entry.path().is_file() && entry.path().extension().map_or(false, |ext| ext == "circom")
}

fn only_noir_files(entry: &DirEntry) -> bool {
    entry.path().is_file() && entry.path().extension().map_or(false, |ext| ext == "nr")
}