# Color encoding spec

HSV and RGB are encoded as tuples within the program. HSV is a tuple made of
three 32-bit floats, while RGB is encoded by three unsigned 8-bit integers. This
means there will be losses in precision when converting from HSV to RGB, but the
losses shouldn't break the program.

## Publicly available methods for HSV and RGB

pub fn convert() -> RGB
pub fn convert() -> HSV
Converts from HSV to RGB or vice versa.

pub fn print()
Prints out the color.

# Available methods of generation

Each method generates a 5-color palette. Generating palettes of arbitrary amount
of colors may be implemented in the future.

## Analogous spread

Parameters:
+ *offset* - the amount to differ the shades by hue
+ *shake* - maximum degree of randomization of saturation [TODO]
+ *color* - starting color

Offsets each generated color from *color* so each generated shade is at most
*offset* from *color.* The distribution of the offset will be even. *shake* is
intended to be the maximum amount to randomly alter the saturation of each
generated shade.
