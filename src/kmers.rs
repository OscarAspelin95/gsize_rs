use bio_utils_rs::io::bio_fastq_reader;
use dashmap::DashMap;

use packed_seq::{PackedSeqVec, SeqVec};
use rayon::prelude::*;
use rustc_hash::FxBuildHasher;
use simd_minimizers::canonical_minimizers;
use std::path::PathBuf;

use crate::errors::AppError;

pub fn get_kmer_counts(
    fastq: PathBuf,
    kmer_size: usize,
    window_size: usize,
) -> Result<DashMap<u64, usize, FxBuildHasher>, AppError> {
    let reader = bio_fastq_reader(Some(fastq))?;

    let counts: DashMap<u64, usize, FxBuildHasher> = DashMap::with_hasher(FxBuildHasher);

    reader.records().par_bridge().for_each(|record| {
        let record = match record {
            Ok(record) => record,
            Err(_) => return,
        };

        let packed_seq = PackedSeqVec::from_ascii(record.seq());

        let mut _pos: Vec<u32> = vec![];

        for minimizer in canonical_minimizers(kmer_size, window_size)
            .run(packed_seq.as_slice(), &mut _pos)
            .values_u64()
        {
            counts.entry(minimizer).and_modify(|c| *c += 1).or_insert(1);
        }
    });

    Ok(counts)
}
