use macroquad::color::Color;

pub struct Theme {
    pub rosewater: Color,
    pub flamingo: Color,
    pub pink: Color,
    pub mauve: Color,
    pub red: Color,
    pub maroon: Color,
    pub peach: Color,
    pub yellow: Color,
    pub green: Color,
    pub teal: Color,
    pub sky: Color,
    pub sapphire: Color,
    pub blue: Color,
    pub lavender: Color,
    pub text: Color,
    pub subtext1: Color,
    pub subtext0: Color,
    pub overlay2: Color,
    pub overlay1: Color,
    pub overlay0: Color,
    pub surface2: Color,
    pub surface1: Color,
    pub surface0: Color,
    pub base: Color,
    pub mantle: Color,
    pub crust: Color,
}

impl Theme {
    pub fn mocha() -> Self {
        Self {
            rosewater: Color::from_hex(0xf5e0dc),
            flamingo: Color::from_hex(0xf2cdcd),
            pink: Color::from_hex(0xf5c2e7),
            mauve: Color::from_hex(0xcba6f7),
            red: Color::from_hex(0xf38ba8),
            maroon: Color::from_hex(0xeba0ac),
            peach: Color::from_hex(0xfab387),
            yellow: Color::from_hex(0xf9e2af),
            green: Color::from_hex(0xa6e3a1),
            teal: Color::from_hex(0x94e2d5),
            sky: Color::from_hex(0x89dceb),
            sapphire: Color::from_hex(0x74c7ec),
            blue: Color::from_hex(0x89b4fa),
            lavender: Color::from_hex(0xb4befe),
            text: Color::from_hex(0xcdd6f4),
            subtext1: Color::from_hex(0xbac2de),
            subtext0: Color::from_hex(0xa6adc8),
            overlay2: Color::from_hex(0x9399b2),
            overlay1: Color::from_hex(0x7f849c),
            overlay0: Color::from_hex(0x6c7086),
            surface2: Color::from_hex(0x585b70),
            surface1: Color::from_hex(0x45475a),
            surface0: Color::from_hex(0x313244),
            base: Color::from_hex(0x1e1e2e),
            mantle: Color::from_hex(0x181825),
            crust: Color::from_hex(0x11111b),
        }
    }

}