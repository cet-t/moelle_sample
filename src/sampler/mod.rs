mod alias;
mod bst;

pub use alias::Alias;
pub use bst::Bst;

pub type SamplerResult<T> = Result<T, alloc::boxed::Box<dyn core::error::Error>>;

pub trait Sampler {
    fn build(&mut self) -> SamplerResult<()>;
    fn sample(&mut self) -> Option<usize>;
}
