# gsize_rs
Quick haploid genome size estimation from sequencing reads using k-mer frequency histograms.

## Requirements
- Linux OS (Ubuntu 24.04.2)
- Rust >= 1.94.0
- x86-64 CPU with AVX2, or ARM CPU with NEON (required by `simd-minimizers`)

## Installation

### Building from Source
Clone the repository or download the source code, enter the `gsize_rs` directory, and run:<br>
`cargo build --release`

The generated binary is available in `target/release/gsize_rs`.

## Usage
Run with:<br>
`gsize_rs --fastq <reads.fastq>`

Optional arguments:
<pre>
<b>--kmer-size</b> [15] - K-mer size for hashing.
<b>--window-size</b> [1] - Minimizer window size. Larger values = fewer hashes => faster but less sensitive.
<b>--output-histogram</b> [hist.tsv] - Path to output k-mer frequency histogram TSV file.
<b>--output-json</b> [stats.json] - Path to output stats JSON file.
</pre>

## Output

### Histogram (`hist.tsv`)
A two-column TSV file with k-mer multiplicity and the number of distinct k-mers observed at that frequency.

### Stats (`stats.json`)
A JSON file containing genome size estimation results:

| Field | Description |
|---|---|
| `est_genome_size` | Estimated haploid genome size in base pairs |
| `peak_kmer_mult` | K-mer multiplicity at the main coverage peak |
| `kmer_size` | K-mer size used |
| `window_size` | Minimizer window size used |

## Method
`gsize_rs` counts minimizer k-mers from a FASTQ file using SIMD intrinsics, followed by building a k-mer frequency histogram. The histogram is analysed to identify the error peak and the main coverage peak. Genome size is estimated as the total k-mer count (excluding errors and trailing zero count kmers) divided by the peak multiplicity.
