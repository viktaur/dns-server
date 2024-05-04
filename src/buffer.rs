use anyhow::{Error, Result, anyhow};

const SIZE: usize = 1024;

pub struct ByteBuffer {
    data: [u8; SIZE],
    pos: usize
}

impl ByteBuffer {
    pub fn new() -> Self {
        ByteBuffer {
            data: [0; SIZE],
            pos: 0,
        }
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn pos(&self) -> usize {
        self.pos
    }

    pub fn jump_to(&mut self, i: usize) -> Result<()> {
        if i >= SIZE {
            return Err(anyhow!("End of buffer"));
        }
        self.pos = i;

        // Ok(self.data[self.pos])
        Ok(())
    }

    pub fn next(&mut self) -> Result<u8> {
        if self.pos + 1 >= SIZE {
            return Err(anyhow!("End of buffer"));
        }
        self.pos += 1;

        Ok(self.data[self.pos])
    }

    pub fn step(&mut self, n: usize) -> Result<u8> {
        if self.pos + n >= SIZE {
            return Err(anyhow!("End of buffer"));
        }
        self.pos += n;

        Ok(self.data[self.pos])
    }
}
