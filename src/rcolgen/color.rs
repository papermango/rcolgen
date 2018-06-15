pub struct HSV(f32, f32, f32);
pub struct RGB(u8, u8, u8);

enum Index { // used to simplify match expressions
  Zero,
  One,
  Two,
}

impl RGB {
  pub fn convert(&self) -> HSV {
    let M = match RGB::max(&self) {
              Index::Zero => self.0 as f32,
              Index::One => self.1 as f32,
              Index::Two => self.2 as f32
            } / 255.0; // max
    let m = match RGB::min(&self) {
              Index::Zero => self.0 as f32,
              Index::One => self.1 as f32,
              Index::Two => self.2 as f32
            } / 255.0; // min
    let value = M;
    let delta = M - m;
    let saturation = delta / value;
    let hue = match delta == 0.0 {
                true => 0.0,
                false => {
                  let (rp, gp, bp) = RGB::normalize(&self);
                  match RGB::max(&self) {
                    Index::Zero => 60.0 * (((gp - bp) / delta) % 6.0), // R' is biggest
                    Index::One => 60.0 * (((bp - rp) / delta) + 2.0), // G' is biggest
                    Index::Two => 60.0 * (((rp - gp) / delta) + 4.0), // B' is biggest
                  }
                }
              };
    HSV(hue, saturation, value) // return statement
  }

  fn max(&self) -> Index { // finds maximum
    match self.0 > self.1 {
      true => match self.0 > self.2 {
                true => Index::Zero,
                false => Index::Two,
              },
      false => match self.1 > self.2 {
                true => Index::One,
                false => Index::Two,
              },
    }
  }

  fn min(&self) -> Index { // finds minimum
    match self.0 < self.1 {
      true => match self.0 < self.2 {
                true => Index::Zero,
                false => Index::Two,
              },
      false => match self.1 < self.2 {
                true => Index::One,
                false => Index::Two,
              },
    }
  }

  pub fn print(&self) -> String {
    let string = String::from("#") + &hex_conv(self.0) + &hex_conv(self.1) + &hex_conv(self.2);
    string
  }

  fn normalize(&self) -> (f32, f32, f32) {
    ((self.0 as f32) / 255.0, (self.1 as f32) / 255.0, (self.2 as f32) / 255.0)
  }
}

impl HSV {
  pub fn convert() -> RGB {
    unimplemented!();
  }

  fn max(&self) -> Index { // finds maximum, same function as above
    match self.0 > self.1 {
      true => match self.0 > self.2 {
                true => Index::Zero,
                false => Index::Two,
              },
      false => match self.1 > self.2 {
                true => Index::One,
                false => Index::Two,
              },
    }
  }

  fn min(&self) -> Index { // finds minimum, same function as above
    match self.0 < self.1 {
      true => match self.0 < self.2 {
                true => Index::Zero,
                false => Index::Two,
              },
      false => match self.1 < self.2 {
                true => Index::One,
                false => Index::Two,
              },
    }
  }

  pub fn print(&self) -> String { // returns string of the color
    let string = format!("({}, {}, {})", self.0, self.1, self.2);
    string
  }
}

pub fn hex_conv(x: u8) -> String { // converts to hex, requires testing
  let mut d1 = 0;
  while d1 < x {
    d1 += 16;
  }
  d1 -= 16;
  let d2 = x - d1; // calculates ones digit
  d1 /= 16; // calculates sixteens (second) digit

  let hexmake = |x: u8| { // converts digit to hex
    let s = match x {
      0 => String::from("0"),
      1 => String::from("1"),
      2 => String::from("2"),
      3 => String::from("3"),
      4 => String::from("4"),
      5 => String::from("5"),
      6 => String::from("6"),
      7 => String::from("7"),
      8 => String::from("8"),
      9 => String::from("9"),
      10 => String::from("A"),
      11 => String::from("B"),
      12 => String::from("C"),
      13 => String::from("D"),
      14 => String::from("E"),
      15 => String::from("F"),
      _ => String::from("X"), // error handling
    };
    return s;
  };

  let s = hexmake(d1) + &hexmake(d2);
  s
}
