use std::fs::File;
use std::io::{BufRead, BufReader, Write};

const COMP_INTRO_END_LINE: usize = 29;
const COMP_MAIN_END_LINE: usize = 13146932;
const COMP_CODA_END_LINE: usize = 13147025;

const DECOMP_MAIN_END_LINE: usize = 13146905;
const DECOMP_INTRO_END_LINE: usize = 13146934;
const DECOMP_CODA_END_LINE: usize = 13147027;

pub fn split4_comp(enwik9_filename: &str) {
    let ifile = BufReader::new(File::open(enwik9_filename).unwrap());
    let mut ofile1 = File::create(".intro").unwrap();
    let mut ofile2 = File::create(".main").unwrap();
    let mut ofile3 = File::create(".coda").unwrap();

    let mut line_count = 0;

    for line in ifile.lines() {
        let s = line.unwrap();
        if line_count < COMP_INTRO_END_LINE {
            writeln!(ofile1, "{}", s).unwrap();
        } else if line_count < COMP_MAIN_END_LINE {
            writeln!(ofile2, "{}", s).unwrap();
        } else if line_count < COMP_CODA_END_LINE {
            writeln!(ofile3, "{}", s).unwrap();
        } else {
            write!(ofile3, "{}", s).unwrap();
        }
        line_count += 1;
    }

    ofile1.flush().unwrap();
    ofile2.flush().unwrap();
    ofile3.flush().unwrap();
}

pub fn split4_decomp(inpnam: &str) {
    let ifile = BufReader::new(File::open(inpnam).unwrap());
    let mut ofile1 = File::create(".intro_decomp").unwrap();
    let mut ofile2 = File::create(".main_decomp").unwrap();
    let mut ofile3 = File::create(".coda_decomp").unwrap();

    let mut line_count = 0;

    for line in ifile.lines() {
        let s = line.unwrap();
        if line_count < DECOMP_MAIN_END_LINE {
            writeln!(ofile2, "{}", s).unwrap();
        } else if line_count < DECOMP_INTRO_END_LINE {
            writeln!(ofile1, "{}", s).unwrap();
        } else if line_count < DECOMP_CODA_END_LINE {
            writeln!(ofile3, "{}", s).unwrap();
        } else {
            write!(ofile3, "{}", s).unwrap();
        }
        line_count += 1;
    }

    ofile1.flush().unwrap();
    ofile2.flush().unwrap();
    ofile3.flush().unwrap();
}
