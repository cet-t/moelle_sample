mod fisher_yates;

#[cfg(test)]
mod test;

pub use fisher_yates::FisherYates;

#[derive(Debug)]
pub enum ShuffleError {
    EmptyArray,
    TryFromError,
}

impl core::fmt::Display for ShuffleError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(match self {
            Self::EmptyArray => "Array is empty",
            Self::TryFromError => "TryFrom failed",
        })
    }
}

pub type ShuffleResult<T> = core::result::Result<T, ShuffleError>;

pub trait Shuffle {
    fn shuffle<T>(&mut self, items: &mut [T]) -> crate::shuffle::ShuffleResult<()>;
}
