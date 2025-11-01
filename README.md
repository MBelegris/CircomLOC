# CircomLOC
Count circom lines of code directly from github repos

## Background

Walk Process -> File Reader -> File Processor -> Summeriser

### How cloc works

First, create a list of files to consider. Next, attempt to determine whether or not found files contain recognized computer language source code. Finally, for files identified as source files, invoke language-specific routines to count the number of source lines.

A more detailed description:

1. If the input file is an archive (such as a .tar.gz or .zip file), create a temporary directory and expand the archive there using a system call to an appropriate underlying utility (tar, bzip2, unzip, etc) then add this temporary directory as one of the inputs. (This works more reliably on Unix than on Windows.)
2. Use File::Find to recursively descend the input directories and make a list of candidate file names. Ignore binary and zero-sized files.
3. Make sure the files in the candidate list have unique contents (first by comparing file sizes, then, for similarly sized files, compare MD5 hashes of the file contents with Digest::MD5). For each set of identical files, remove all but the first copy, as determined by a lexical sort, of identical files from the set. The removed files are not included in the report. (The --skip-uniqueness switch disables the uniqueness tests and forces all copies of files to be included in the report.) See also the --ignored= switch to see which files were ignored and why.
4. Scan the candidate file list for file extensions which cloc associates with programming languages (see the `--show-lang` and `--show-ext` options). Files which match are classified as containing source code for that language. Each file without an extension is opened and its first line read to see if it is a Unix shell script (anything that begins with `#!`). If it is shell script, the file is classified by that scripting language (if the language is recognized). If the file does not have a recognized extension or is not a recognized scripting language, the file is ignored.
5. All remaining files in the candidate list should now be source files
    for known programming languages. For each of these files:
    1.  Read the entire file into memory.
    2.  Count the number of lines (= L<sub>original</sub>).
    3.  Remove blank lines, then count again (= L<sub>non_blank</sub>).
    4.  Loop over the comment filters defined for this language. (For
        example, C++ has two filters: (1) remove lines that start with
        optional whitespace followed by // and (2) remove text between
        /* and */) Apply each filter to the code to remove comments.
        Count the left over lines (= L<sub>code</sub>).
    5.  Save the counts for this language:
        * blank lines = L<sub>original</sub> - L<sub>non_blank</sub>
        * comment lines = L<sub>non_blank</sub> - L<sub>code</sub>
        * code lines = L<sub>code</sub>
