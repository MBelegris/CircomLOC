# Circom LOC Analyzer

A Rust tool that recursively analyzes all `.circom` files in a given directory and reports their lines of code (LOC).

## Overview

This program scans a specified file or directory, identifies all `.circom` files, and computes line statistics for each.  
It prints either a single-file summary or a table summarizing all files found.

## Usage

### Run the program

```bash
cargo run -- <path>
```
