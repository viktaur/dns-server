use anyhow::{Error, Result, anyhow};

const SIZE: usize = 1024;

pub struct ByteBuffer {
    pub(crate) buf: [u8; SIZE],
    pub(crate) pos: usize
}

impl ByteBuffer {
    pub fn new() -> Self {
        ByteBuffer {
            buf: [0; SIZE],
            pos: 0,
        }
    }

    pub fn next(&mut self) -> Result<u8> {
        if self.pos + 1 >= SIZE {
            return Err(anyhow!("End of buffer"));
        }
        self.pos += 1;

        Ok(self.buf[self.pos])
    }

    pub fn step(&mut self, n: usize) -> Result<u8> {
        if self.pos + n >= SIZE {
            return Err(anyhow!("End of buffer"));
        }
        self.pos += n;

        Ok(self.buf[self.pos])
    }
}
