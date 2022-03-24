#[derive(Default)]
pub struct ColorComponents {
    pub ambiant : RGBColor,
    pub diffuse : RGBColor,
    pub specular : RGBColor
}

/// A basic RGB color with unsigned 8 bits integers (0..255)
#[derive(Debug, Default, Copy, Clone)]
pub struct RGBColor {
    pub r : u8,
    pub g : u8,
    pub b : u8
}