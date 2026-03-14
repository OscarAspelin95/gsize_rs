use crate::hist::Thresholds;

pub fn estimate_genome_size(hist: &[usize], thresholds: &Thresholds) -> usize {
    let prod = &hist[thresholds.error_end..thresholds.zero_start]
        .iter()
        .enumerate()
        .map(|(i, val)| (i + thresholds.error_end) * val)
        .sum::<usize>();

    prod / thresholds.mean_kmer_index
}
