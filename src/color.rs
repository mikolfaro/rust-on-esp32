use anyhow::Result;

#[derive(Debug)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}
impl TryFrom<&str> for Color {
    type Error = anyhow::Error;

    fn try_from(input: &str) -> Result<Color> {
        if input.len() != 6 {
            return Err(());
        }

        let red: u8 = u8::from_str_radix(&input[0..2], 16)?;
        let blue: u8 = u8::from_str_radix(&input[2..4], 16)?;
        let green: u8 = u8::from_str_radix(&input[4..6], 16)?;
        Ok(Color { r: red, g: green, b: blue })
    }
}
