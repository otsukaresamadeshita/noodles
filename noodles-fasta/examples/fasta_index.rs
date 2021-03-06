//! Builds a FASTA index.
//!
//! This writes the output to stdout rather than `<src>.fai`.
//!
//! The result matches the output of `samtools faidx <src>`.

use std::{env, io};

use noodles_fasta::fai;

fn main() -> io::Result<()> {
    let src = env::args().nth(1).expect("missing src");

    let index = fai::index(src)?;

    let stdout = io::stdout();
    let handle = stdout.lock();
    let mut writer = fai::Writer::new(handle);

    for record in &index {
        writer.write_record(record)?;
    }

    Ok(())
}
