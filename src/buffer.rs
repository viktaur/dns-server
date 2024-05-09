use anyhow::{Error, Result, anyhow};

// const SIZE: usize = 1024;

pub struct ByteReader {
    data: Vec<u8>,
    pos: usize
}

impl ByteReader {
    pub fn new(slice: &[u8]) -> Self {
        // let data = slice.try_into()
        //     .expect(&format!("Slice should be smaller than buffer size {SIZE}!"));
        let data = Vec::from(slice);

        ByteReader {
            data,
            pos: 0,
        }
    }

    pub fn remaining_bytes(&self) -> usize {
        self.data.len() - self.pos
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn pos(&self) -> usize {
        self.pos
    }

    pub fn read_n_bytes(&mut self, n: usize) -> Result<&[u8]> {
        if self.pos + n > self.data.len() {
            return Err(anyhow!("End of buffer"));
        }

        let slice = &self.data[self.pos..self.pos+n];
        self.pos = self.pos + n;
        Ok(slice)
    }

    pub fn jump_to(&mut self, i: usize) -> Result<()> {
        if i >= self.data.len() {
            return Err(anyhow!("End of buffer"));
        }
        self.pos = i;

        // Ok(self.data[self.pos])
        Ok(())
    }

    pub fn next(&mut self) -> Result<u8> {
        if self.pos + 1 >= self.data.len() {
            return Err(anyhow!("End of buffer"));
        }
        self.pos += 1;

        Ok(self.data[self.pos])
    }

    pub fn slice(&self) -> &[u8] {
        &self.data[self.pos..]
    }

    pub fn step(&mut self, n: usize) -> Result<()> {
        if self.pos + n >= self.data.len() {
            return Err(anyhow!("End of buffer"));
        }
        self.pos += n;

        Ok(())
    }
}
