extern crate cryptopals;

use cryptopals::bit::XorIter;
use cryptopals::conversion::char::CharWrite;
use cryptopals::conversion::hex::HexRead;

use std::iter;

fn main() {
    let input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";

    for i in 0u8..=255 {
        let res = input
            .hex()
            .xor(iter::repeat(i))
            .to_char()
            .collect::<String>();

        println!("{}: {}", i, res);
    }
}
