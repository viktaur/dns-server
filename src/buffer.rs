use anyhow::{Result, anyhow};

// const SIZE: usize = 1024;

pub struct ByteDecoder {
    data: Vec<u8>,
    pos: usize
}

impl ByteDecoder {
    pub fn new(slice: &[u8]) -> Self {
        // let data = slice.try_into()
        //     .expect(&format!("Slice should be smaller than buffer size {SIZE}!"));
        let data = Vec::from(slice);

        ByteDecoder {
            data,
            pos: 0,
        }
    }

    /// Returns the number of remaining bytes from the current position until the end of
    /// the buffer.
    pub fn remaining_bytes(&self) -> usize {
        self.data.len() - self.pos
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn pos(&self) -> usize {
        self.pos
    }

    /// Returns a slice withg the next `n` bytes from the current position, updating the
    /// current poisition.
    pub fn read_n_bytes(&mut self, n: usize) -> Result<&[u8]> {
        if self.pos + n > self.data.len() {
            return Err(anyhow!("End of buffer"));
        }

        let slice = &self.data[self.pos..self.pos+n];
        self.pos += n;
        Ok(slice)
    }

    /// Moves the pointer to a specific position in the buffer.
    pub fn jump_to(&mut self, i: usize) -> Result<()> {
        if i >= self.data.len() {
            return Err(anyhow!("End of buffer"));
        }
        self.pos = i;

        // Ok(self.data[self.pos])
        Ok(())
    }

    /// Moves the pointer by one position and returns the new current element.
    pub fn next(&mut self) -> Result<u8> {
        if self.pos + 1 >= self.data.len() {
            return Err(anyhow!("End of buffer"));
        }
        self.pos += 1;

        Ok(self.data[self.pos])
    }

    /// Returns the remaining bytes from the current position until the end of the buffer.
    pub fn slice(&self) -> &[u8] {
        &self.data[self.pos..]
    }

    /// Moves the position pointer by `n` bytes.
    pub fn step(&mut self, n: usize) -> Result<()> {
        if self.pos + n >= self.data.len() {
            return Err(anyhow!("End of buffer"));
        }
        self.pos += n;

        Ok(())
    }
}
