use super::*;

#[derive(PartialEq, Debug)]
pub(crate) struct DecimalSat {
  height: Height,
  offset: u64,
}

impl From<Sat> for DecimalSat {
  fn from(sat: Sat) -> Self {
    Self {
      height: sat.height(),
      offset: sat.third(),
    }
  }
}

impl Display for DecimalSat {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "{}.{}", self.height, self.offset)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn decimal() {
    assert_eq!(
      Sat(0).decimal(),
      DecimalSat {
        height: Height(0),
        offset: 0
      }
    );
    assert_eq!(
      Sat(1).decimal(),
      DecimalSat {
        height: Height(0),
        offset: 1
      }
    );
    assert_eq!(
      Sat(8399999990759999).decimal(),
      DecimalSat {
        height: Height(27719999),
        offset: 0
      }
    );
  }
}
