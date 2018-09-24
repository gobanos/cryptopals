use std::iter;

pub trait XorIter {
    fn xor<J>(self, other: J) -> Xor<Self, <J as iter::IntoIterator>::IntoIter>
    where
        J: IntoIterator<Item = u8>,
        Self: Sized;
}

impl<I> XorIter for I
where
    I: Iterator<Item = u8>,
{
    fn xor<J>(self, other: J) -> Xor<Self, <J as iter::IntoIterator>::IntoIter>
    where
        J: IntoIterator<Item = u8>,
        Self: Sized,
    {
        Xor {
            iter: self.zip(other),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Xor<I, J> {
    iter: iter::Zip<I, J>,
}

impl<I, J> Iterator for Xor<I, J>
where
    I: Iterator<Item = u8>,
    J: Iterator<Item = u8>,
{
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        self.iter.next().map(|(a, b)| a ^ b)
    }
}
