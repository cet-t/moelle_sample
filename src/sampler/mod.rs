mod alias;
mod bst;

#[cfg(test)]
mod test;

pub use alias::Alias;
pub use bst::Bst;

#[derive(Debug)]
pub enum SamplerError {
    A,
}

impl core::fmt::Display for SamplerError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(match self {
            Self::A => "",
        })
    }
}

impl core::error::Error for SamplerError {}

pub type SamplerResult<T> = Result<T, crate::SamplerError>;

pub trait Sampler {
    fn build(&mut self) -> SamplerResult<()>;
    fn sample(&mut self) -> Option<usize>;
}
