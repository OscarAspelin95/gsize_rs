use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
pub struct Args {
    #[arg(short, long, help = "Path to fastq file")]
    pub fastq: PathBuf,

    #[arg(
        long,
        default_value = "hist.tsv",
        help = "Path to output histogram TSV file."
    )]
    pub output_histogram: String,

    #[arg(
        long,
        default_value = "stats.json",
        help = "Path to output stats JSON file."
    )]
    pub output_json: String,

    #[arg(short, long, default_value_t = 15, help = "Kmer size")]
    pub kmer_size: usize,

    #[arg(short, long, default_value_t = 1, help = "Minimizer window size")]
    pub window_size: usize,
}
