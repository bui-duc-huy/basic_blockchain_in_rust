use std::fmt;

#[cfg_attr(test, derive(Default))]
#[derive(Debug, Eq, Clone, PartialEq)]
pub struct BlockHeader {
  pub parent_hash: &'static str,

  pub difficulty: u64,
  pub number: u64,
  pub timestamp: i64,

  pub nonce: u64,
}

impl fmt::Display for BlockHeader {
    fn fmt(&self,  f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "parent_hash: {}, difficulty: {}, number: {}, timestamp: {}, nonce: {}", self.parent_hash, self.difficulty, self.number, self.timestamp, self.nonce)
    }
}

#[cfg_attr(test, derive(Default))]
#[derive(Debug, Eq, Clone, PartialEq)]
pub struct Block {
  pub header: BlockHeader,
  pub ommers: Vec<BlockHeader>,
}
