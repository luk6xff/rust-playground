pub use std::io::Read;

pub struct RotDecoder<R: Read> {
    pub input: R,
    pub rot: u8,
}

// Implement the `Read` trait for `RotDecoder`.
impl<R: Read> Read for RotDecoder<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let n = self.input.read(buf)?;
        for i in 0..n {
            if buf[i].is_ascii_alphabetic() {
                let base = if buf[i].is_ascii_lowercase() { b'a' } else { b'A' };
                buf[i] = (base + (buf[i] - base + self.rot) % 26) as u8;
            }
        }
        Ok(n)
    }
}

pub fn joke() {
    let mut rot =
        RotDecoder { input: "Gb trg gb gur bgure fvqr!".as_bytes(), rot: 13 };
    let mut result = String::new();
    rot.read_to_string(&mut result).unwrap();
    assert_eq!(&result, "To get to the other side!");
}

pub fn binary() {
    let input: Vec<u8> = (0..=255u8).collect();
    let mut rot = RotDecoder::<&[u8]> { input: input.as_ref(), rot: 13 };
    let mut buf = [0u8; 256];
    assert_eq!(rot.read(&mut buf).unwrap(), 256);
    for i in 0..=255 {
        if input[i] != buf[i] {
            assert!(input[i].is_ascii_alphabetic());
            assert!(buf[i].is_ascii_alphabetic());
        }
    }
}