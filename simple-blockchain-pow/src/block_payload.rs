use core::fmt;
use std::fmt::Formatter;

#[derive(Debug, Clone)]
pub struct BlockPayload {
   pub text: String,
}

impl fmt::Display for BlockPayload {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "block-data:{}", self.text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn block_payload_format() {
        let block_payload = BlockPayload {
            text: String::from("Hello world!"),
        };

        assert_eq!(format!("{}", block_payload), "block-data:Hello world!");
    }
}


