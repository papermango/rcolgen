extern crate rcolgen;

fn main() {
  print!("Hello world!");
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn hextest() {
    assert_eq!(rcolgen::color::hex_conv(55), String::from("37")); // successful
    assert_eq!(color::hex_conv(176), String::from("B0")); // fails, returns AX instead of B0
    assert_eq!(rcolgen::color::hex_conv(178), String::from("B2")); // successful
  }

  #[test]
  fn hextest_edges() {
    assert_eq!(rcolgen::color::hex_conv(0), String::from("00")); // fails, error is 'attempt to
                                                                 // subtract with overflow'
    assert_eq!(rcolgen::color::hex_conv(255), String::from("FF")); // successful
  }
}
