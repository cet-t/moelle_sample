use fil::{Rng, Word};

#[cfg(test)]
mod test;

pub trait Choice: fil::Rng {
    fn choice<'a, T>(&mut self, items: &'a [T]) -> Option<&'a T>;
    fn choice_mut<'a, T>(&mut self, items: &'a mut [T]) -> Option<&'a mut T>;
}

impl<R> Choice for R
where
    R: Rng,
    <R::Word as Word>::Int: From<u8> + TryFrom<usize> + TryInto<usize>,
{
    fn choice<'a, T>(&mut self, items: &'a [T]) -> Option<&'a T> {
        if items.is_empty() {
            None
        } else {
            let max = (items.len() - 1).try_into().ok()?;
            let i: usize = self.randi(0u8.into(), max).try_into().ok()?;
            Some(&items[i])
        }
    }

    fn choice_mut<'a, T>(&mut self, items: &'a mut [T]) -> Option<&'a mut T> {
        if items.is_empty() {
            None
        } else {
            let max = (items.len() - 1).try_into().ok()?;
            let i: usize = self.randi(0u8.into(), max).try_into().ok()?;
            Some(&mut items[i])
        }
    }
}
