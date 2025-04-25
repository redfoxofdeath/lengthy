use std::io::{Read, Write};

#[allow(dead_code)]
trait LengthPrefix: Read + Write {
    fn write_len(&mut self, len: usize) -> std::io::Result<()>;
    fn write_lp_bytes(&mut self, bytes: &[u8]) -> std::io::Result<()>;
    fn read_len(&mut self) -> std::io::Result<usize>;
    fn read_lp_bytes(&mut self) -> std::io::Result<Vec<u8>>;
}

impl<T: Read + Write> LengthPrefix for T {
    fn write_len(&mut self, mut len: usize) -> std::io::Result<()> {
        let mut lbytes = vec![];
        lbytes.push(len as u8 & 127); // first without cont' bit
        len = len >> 7;
        while len > 0 {
            lbytes.push((len as u8 & 127) + 128); // +128 adds cont' bit
            len = len >> 7;
        }
        lbytes.reverse();
        self.write_all(&mut lbytes)?;
        Ok(())
    }

    fn write_lp_bytes(&mut self, bytes: &[u8]) -> std::io::Result<()> {
        self.write_len(bytes.len())?;
        self.write_all(&bytes)?;
        Ok(())
    }

    fn read_len(&mut self) -> std::io::Result<usize> {
        let mut len = 0;
        loop {
            let mut lbyte = vec![0];
            self.read(&mut lbyte)?;
            let lbyte = lbyte[0] as usize;
            len = len << 7;
            if lbyte & 128 == 128 { // cont' bit present
                len += lbyte - 128;
            }
            else { // cont' bit not present
                len += lbyte;
                return Ok(len);
            }
        }
    }

    fn read_lp_bytes(&mut self) -> std::io::Result<Vec<u8>> {
        let len = self.read_len()?;
        let mut buf = vec![0; len];
        self.read(&mut buf)?;
        Ok(buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Cursor, Seek};

    #[test]
    fn test() {
        let mut c = Cursor::new(Vec::<u8>::new());
    
        let str_in = String::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum");
        let bytes_in = str_in.as_bytes();
        let _ = c.write_lp_bytes(&bytes_in);
    
        let _ = c.seek(std::io::SeekFrom::Start(0));
    
        let bytes_out = c.read_lp_bytes().unwrap();
        let str_out = String::from_utf8(bytes_out).unwrap();
    
        println!("{}", str_out);
        assert!(str_in == str_out);
    }
}
