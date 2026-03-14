mod args;
mod errors;
mod gsize;
mod hist;
mod kmers;
mod schema;
mod writer;

use args::Args;
use bio_utils_rs::io::write_json;
use errors::AppError;
use hist::{build_histogram, get_thresholds};
use kmers::get_kmer_counts;

use clap::Parser;
use std::path::PathBuf;

use crate::schema::AppResult;

fn main() -> Result<(), AppError> {
    let args = Args::parse();

    let counts = get_kmer_counts(args.fastq, args.kmer_size, args.window_size)?;
    let hist = build_histogram(&counts)?;

    writer::write_histogram(PathBuf::from(args.output_histogram), &hist)?;

    match get_thresholds(&hist) {
        Ok(thresholds) => {
            let est_genome_size = gsize::estimate_genome_size(&hist, &thresholds);

            let result = AppResult {
                est_genome_size: est_genome_size,
                peak_kmer_mult: thresholds.mean_kmer_index,
                kmer_size: args.kmer_size,
                window_size: args.window_size,
            };

            write_json(Some(PathBuf::from(args.output_json)), result)?;
        }
        Err(e) => eprintln!("Failed to estimate genome size: `{}`", e.to_string()),
    }

    Ok(())
}
