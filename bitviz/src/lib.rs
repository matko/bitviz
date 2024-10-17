pub mod render;

use std::{
    collections::VecDeque,
    io::{self, ErrorKind, Read},
    marker::PhantomData,
};

use byteorder::{ByteOrder, ReadBytesExt};

pub struct WordSize(usize);

impl WordSize {
    pub const fn new(size: usize) -> Self {
        if size != 1 && size != 2 && size != 4 && size != 8 {
            panic!("invalid word size");
        }

        Self(size)
    }

    pub fn read_word<R: Read, B: ByteOrder>(&self, mut reader: R) -> Result<Vec<bool>, io::Error> {
        Ok(match self.0 {
            1 => {
                let num = reader.read_u8()? as u64;
                iter_word(num, 1)
            }
            2 => {
                let num = reader.read_u16::<B>()? as u64;
                iter_word(num, 2)
            }
            4 => {
                let num = reader.read_u32::<B>()? as u64;
                iter_word(num, 4)
            }
            8 => {
                let num = reader.read_u64::<B>()?;
                iter_word(num, 8)
            }
            _ => panic!("a"),
        }
        .collect())
    }
}

pub fn iter_word(word: u64, size: usize) -> impl Iterator<Item = bool> {
    (0..8 * size as u64)
        .rev()
        .map(move |b| (word >> b) & 1 != 0)
}

pub fn reader_to_bits<R: Read, B: ByteOrder>(reader: R, word_size: WordSize) -> BitIterator<R, B> {
    BitIterator {
        reader,
        word_size,
        current: VecDeque::new(),
        _x: PhantomData,
    }
}

pub struct BitIterator<R, B: ByteOrder> {
    reader: R,
    word_size: WordSize,
    current: VecDeque<bool>,
    _x: PhantomData<B>,
}

impl<R: Read, B: ByteOrder> Iterator for BitIterator<R, B> {
    type Item = bool;

    fn next(&mut self) -> Option<bool> {
        if self.current.is_empty() {
            let result = self.word_size.read_word::<_, B>(&mut self.reader);
            match result {
                Ok(bits) => self.current = bits.into(),
                Err(e) if e.kind() == ErrorKind::UnexpectedEof => return None,
                Err(e) => panic!("error: {e}"),
            }
        }

        Some(self.current.pop_front().unwrap())
    }
}
