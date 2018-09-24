pub trait CharWrite {
    fn to_char(self) -> CharWriter<Self>
    where
        Self: Sized;
}

impl<I> CharWrite for I
where
    I: Iterator<Item = u8>,
{
    fn to_char(self) -> CharWriter<Self> {
        CharWriter { iter: self }
    }
}

#[derive(Clone, Debug)]
pub struct CharWriter<I> {
    iter: I,
}

impl<I> Iterator for CharWriter<I>
where
    I: Iterator<Item = u8>,
{
    type Item = char;

    fn next(&mut self) -> Option<char> {
        self.iter.next().map(|c| c as char)
    }
}
