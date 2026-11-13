use crate::{Int, ShuffleError, ShuffleResult};

pub struct FisherYates<'a, R>
where
    R: moelle::Rng,
{
    pub(crate) rng: &'a mut R,
}

impl<'a, R> FisherYates<'a, R>
where
    R: moelle::Rng,
{
    pub const fn new(rng: &'a mut R) -> Self {
        Self { rng }
    }
}

impl<'a, R> crate::shuffle::Shuffle for FisherYates<'a, R>
where
    R: moelle::Rng,
    crate::Int<R>: TryFrom<usize>,
    usize: TryFrom<crate::Int<R>>,
{
    fn shuffle<T>(&mut self, items: &mut [T]) -> crate::ShuffleResult<()> {
        if items.is_empty() {
            return Err(super::ShuffleError::EmptyArray);
        }

        (1..items.len()).rev().try_for_each(|i| {
            let upper = Int::<R>::try_from(i)
                .ok()
                .ok_or(ShuffleError::TryFromError)?;

            let lower = Int::<R>::try_from(0usize)
                .ok()
                .ok_or(ShuffleError::TryFromError)?;

            let j = usize::try_from(self.rng.randi(lower, upper))
                .ok()
                .ok_or(ShuffleError::TryFromError)?;

            items.swap(i, j);

            ShuffleResult::Ok(())
        })
    }
}
