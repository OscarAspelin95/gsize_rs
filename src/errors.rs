use bio_utils_rs::errors::BioError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error(transparent)]
    BioError(#[from] BioError),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error("No kmers found")]
    NoKmersFoundError(String),

    #[error("Could not find error peak")]
    ErrorPeakNotFound(String),

    #[error("Count not find trailing zero counts")]
    TrailingZeroError(String),

    #[error("Could not find kmer peak.")]
    KmerPeakError(String),
}
