use std::fmt;

pub struct StringTable {
    data: Vec<u8>,
}

impl StringTable {
    pub fn new(data: Vec<u8>) -> Self {
        StringTable { data }
    }

    pub fn get(&self, idx: usize) -> &str {
        let bytes = self.data[idx..].split(|n| *n == 0).next().unwrap();
        std::str::from_utf8(bytes).unwrap()
    }
}

impl fmt::Debug for StringTable {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let mut debug = fmt.debug_list();

        for n in self.data.iter() {
            match n {
                0 => {
                    debug.entry(&"");
                }
                _ => {
                    debug.entry(&char::from(*n));
                }
            }
        }

        debug.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_string() {
        let data = vec![0x00, 0x5f, 0x6d, 0x61, 0x69, 0x6e, 0x00];
        let stable = StringTable::new(data);

        assert_eq!(stable.get(1), "_main");
        assert_eq!(stable.get(2), "main");
    }
}
