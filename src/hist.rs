use dashmap::DashMap;
use rustc_hash::FxBuildHasher;

use crate::errors::AppError;

pub fn build_histogram(
    counts: &DashMap<u64, usize, FxBuildHasher>,
) -> Result<Vec<usize>, AppError> {
    let max_item = counts
        .iter()
        .max_by_key(|s| *s.value())
        .ok_or(AppError::NoKmersFoundError("Kmer set is empty.".into()))?;

    let max_count = max_item.value();

    let mut kmer_hist: Vec<usize> = vec![0; *max_count + 1];

    for entry in counts.iter() {
        kmer_hist[*entry.value()] += 1
    }

    Ok(kmer_hist)
}

#[derive(Debug)]
pub struct Thresholds {
    pub error_end: usize,
    pub zero_start: usize,
    pub mean_kmer_index: usize,
    pub mean_kmer_count: usize,
}

pub fn get_thresholds(hist: &[usize]) -> Result<Thresholds, AppError> {
    let error_end = *&hist[1..]
        .array_windows()
        .enumerate()
        .find_map(|(i, [a, b, c, d, e])| {
            if a > b && b > c && c < d && d < e {
                // i is the window index (starts at a).
                // our target index is  at c (which is two steps ahead of a)
                // we have also disregarded index 0 in &hist so we need an extra +1 to account for this.
                return Some(i + 2 + 1);
            }

            return None;
        })
        .ok_or(AppError::ErrorPeakNotFound("".into()))?;

    let zero_start = hist[error_end..]
        .array_windows()
        .enumerate()
        .find_map(|(i, [a, b, c])| {
            if a + b + c == 0 {
                return Some(i + error_end);
            }

            return None;
        })
        .ok_or(AppError::TrailingZeroError("".into()))?;

    let (mean_kmer_index, mean_kmer_count) = &hist[error_end..zero_start]
        .iter()
        .enumerate()
        .max_by_key(|(_, count)| *count)
        .map(|(i, count)| (i + error_end, *count))
        .ok_or(AppError::KmerPeakError("".into()))?;

    Ok(Thresholds {
        error_end,
        zero_start,
        mean_kmer_index: *mean_kmer_index,
        mean_kmer_count: *mean_kmer_count,
    })
}
