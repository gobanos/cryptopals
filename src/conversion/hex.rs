use std::str;

pub trait HexRead<'a> {
    fn hex(&'a self) -> HexReader<'a>;
}

impl<'a> HexRead<'a> for str {
    fn hex(&'a self) -> HexReader<'a> {
        HexReader {
            chars: self.chars(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct HexReader<'a> {
    chars: str::Chars<'a>,
}

impl<'a> Iterator for HexReader<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        let a = self.chars.next()?.to_digit(16)? as u8;
        let b = self.chars.next()?.to_digit(16)? as u8;

        Some((a << 4) + b)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (min, max) = self.chars.size_hint();

        (min / 2, max.map(|m| m / 2))
    }

    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.chars.count() / 2
    }
}

pub trait HexWrite {
    fn to_hex(self) -> HexWriter<Self>
    where
        Self: Sized;
}

impl<I> HexWrite for I
where
    I: Iterator<Item = u8>,
{
    fn to_hex(self) -> HexWriter<Self> {
        HexWriter {
            iter: self,
            buffer: None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct HexWriter<I> {
    iter: I,
    buffer: Option<char>,
}

impl<I> Iterator for HexWriter<I>
where
    I: Iterator<Item = u8>,
{
    type Item = char;

    fn next(&mut self) -> Option<char> {
        if self.buffer.is_some() {
            self.buffer.take()
        } else if let Some(c) = self.iter.next() {
            self.buffer = Some(to_char(c & 0b1111));
            Some(to_char(c >> 4))
        } else {
            None
        }
    }
}

fn to_char(x: u8) -> char {
    debug_assert_eq!(x & 0b1111_0000, 0);

    (match x {
        0..=9 => b'0' + x,
        10..=16 => b'a' + x - 10,
        _ => unreachable!(),
    }) as char
}

#[cfg(test)]
mod tests {
    use super::HexRead;

    #[test]
    fn full_range() {
        for i in 0u8..=255 {
            let s = format!("{:02x}", i);
            assert_eq!(s.hex().next(), Some(i));
        }
    }
}
