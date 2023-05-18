use substring::Substring;

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
    pub fn rgba(&self) -> String {
        let fa = (self.a as f32) / 255.0;
        format!("rgba({},{},{},{:.1})", self.r, self.g, self.b, fa)
    }
    pub fn opacity(&self) -> f32 {
        let a = self.a as f32;
        a / 255.0
    }
    pub fn is_zero(&self) -> bool {
        self.r == 0 && self.g == 0 && self.b == 0 && self.a == 0
    }
    pub fn is_transparent(&self) -> bool {
        self.a == 0
    }
    pub fn is_nontransparent(&self) -> bool {
        self.a == 255
    }
    pub fn white() -> Color {
        (255, 255, 255).into()
    }
    pub fn black() -> Color {
        (0, 0, 0).into()
    }
    pub fn with_alpha(&self, a: u8) -> Color {
        let mut c = *self;
        c.a = a;
        c
    }
}

impl From<(u8, u8, u8)> for Color {
    fn from(values: (u8, u8, u8)) -> Self {
        Color {
            r: values.0,
            g: values.1,
            b: values.2,
            a: 255,
        }
    }
}
impl From<(u8, u8, u8, u8)> for Color {
    fn from(values: (u8, u8, u8, u8)) -> Self {
        Color {
            r: values.0,
            g: values.1,
            b: values.2,
            a: values.3,
        }
    }
}

fn parse_hex(hex: &str) -> u8 {
    u8::from_str_radix(hex, 16).unwrap_or_default()
}

impl From<&str> for Color {
    fn from(value: &str) -> Self {
        let mut c = Color::default();
        if !value.starts_with('#') {
            return c;
        }
        let hex = value.substring(1, value.len());
        if hex.len() == 3 {
            c.r = parse_hex(hex.substring(0, 1));
            c.g = parse_hex(hex.substring(1, 2));
            c.b = parse_hex(hex.substring(2, 3));
        } else {
            c.r = parse_hex(hex.substring(0, 2));
            c.g = parse_hex(hex.substring(2, 4));
            c.b = parse_hex(hex.substring(4, 6));
        }
        c.a = 255;
        c
    }
}

#[cfg(test)]
mod tests {
    use super::Color;
    #[test]
    fn color_hex() {
        let mut c: Color = (200, 200, 200).into();
        assert_eq!("#C8C8C8", c.hex());

        c = (51, 51, 51).into();
        assert_eq!("#333333", c.hex());
    }
    #[test]
    fn color_rgba() {
        let mut c: Color = (200, 200, 200).into();
        assert_eq!("rgba(200,200,200,1.0)", c.rgba());
        c = (51, 51, 51, 51).into();
        assert_eq!("rgba(51,51,51,0.2)", c.rgba());
    }
    #[test]
    fn color_opacity() {
        let mut c: Color = (200, 200, 200).into();
        assert_eq!(1.0, c.opacity());
        c = (51, 51, 51, 51).into();
        assert_eq!(0.2, c.opacity());
    }
    #[test]
    fn color_is_zero() {
        let mut c: Color = (200, 200, 200).into();
        assert!(!c.is_zero());
        c = (0, 0, 0, 0).into();
        assert!(c.is_zero());
    }
    #[test]
    fn color_is_transparent() {
        let mut c: Color = (200, 200, 200).into();
        assert!(!c.is_transparent());
        assert!(c.is_nontransparent());
        c = (200, 200, 200, 0).into();
        assert!(c.is_transparent());
        c = (200, 200, 200, 100).into();
        assert!(!c.is_nontransparent());
    }
    #[test]
    fn color_static() {
        assert_eq!("rgba(255,255,255,1.0)", Color::white().rgba());
        assert_eq!("rgba(0,0,0,1.0)", Color::black().rgba());
    }

    #[test]
    fn color_with_alpha() {
        let mut c = Color::white();
        assert_eq!("rgba(255,255,255,1.0)", c.rgba());
        c = c.with_alpha(51);
        assert_eq!("rgba(255,255,255,0.2)", c.rgba());
    }
}
