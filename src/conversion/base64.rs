pub trait Base64Write {
    fn to_base64(self) -> Base64Writer<Self>
    where
        Self: Sized;
}

impl<I> Base64Write for I
where
    I: Iterator<Item = u8>,
{
    fn to_base64(self) -> Base64Writer<Self> {
        Base64Writer {
            iter: self,
            buffer: [0; 4],
            offset: 4,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Base64Writer<I> {
    iter: I,
    buffer: [u8; 4],
    offset: usize,
}

impl<I> Base64Writer<I>
where
    I: Iterator<Item = u8>,
{
    fn load_buffer(&mut self) -> bool {
        debug_assert_eq!(self.offset, 4);

        let a = self.iter.next();
        let b = self.iter.next();
        let c = self.iter.next();

        match (a, b, c) {
            // Early return, to stop iteration
            (None, _, _) => return false,
            (Some(a), Some(b), Some(c)) => {
                self.buffer[0] = encode(a >> 2);
                self.buffer[1] = encode(((a & 0b11) << 4) + (b >> 4));
                self.buffer[2] = encode(((b & 0b1111) << 2) + (c >> 6));
                self.buffer[3] = encode(c & 0b11_1111);
            }
            (Some(a), Some(b), None) => {
                self.buffer[0] = encode(a >> 2);
                self.buffer[1] = encode(((a & 0b11) << 4) + (b >> 4));
                self.buffer[2] = encode((b & 0b1111) << 2);
                self.buffer[3] = b'=';
            }
            (Some(a), None, _) => {
                self.buffer[0] = encode(a >> 2);
                self.buffer[1] = encode((a & 0b11) << 4);
                self.buffer[2] = b'=';
                self.buffer[3] = b'=';
            }
        }

        self.offset = 0;

        true
    }
}

fn encode(x: u8) -> u8 {
    debug_assert_eq!(x & 0b1100_0000, 0);

    match x {
        0b00_0000..=0b01_1001 => b'A' + x,
        0b01_1010..=0b11_0011 => b'a' + x - 0b01_1010,
        0b11_0100..=0b11_1101 => b'0' + x - 0b11_0100,
        0b11_1110 => b'+',
        0b11_1111 => b'/',
        _ => unreachable!(),
    }
}

impl<I> Iterator for Base64Writer<I>
where
    I: Iterator<Item = u8>,
{
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        if self.offset == 4 && !self.load_buffer() {
            return None;
        }

        let c = self.buffer[self.offset];
        self.offset += 1;

        Some(c)
    }
}
