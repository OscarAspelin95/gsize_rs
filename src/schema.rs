use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AppResult {
    pub est_genome_size: usize,
    pub peak_kmer_mult: usize,
    pub kmer_size: usize,
    pub window_size: usize,
}
