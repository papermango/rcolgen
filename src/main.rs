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
    assert_eq!(rcolgen::color::hex_conv(12), String::from("0C")); // successful
    // edge analysis
    assert_eq!(rcolgen::color::hex_conv(176), String::from("B0")); // successful
    assert_eq!(rcolgen::color::hex_conv(178), String::from("B2")); // successful
    assert_eq!(rcolgen::color::hex_conv(175), String::from("AF")); // successful
  }

  #[test]
  fn hextest_edges() {
    assert_eq!(rcolgen::color::hex_conv(0), String::from("00")); // successful
    assert_eq!(rcolgen::color::hex_conv(255), String::from("FF")); // successful
  }
}
