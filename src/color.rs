// Represents different color formats: Hex, RGB, and HSL.
pub enum Color {
    Hex(String),
    RGB(u8, u8, u8),
    HSL(u16, f32, f32),
}

impl Color {
    /// Color::combine
    ///
    /// Combines two colors based on a percentage and color space.
    ///
    /// * `other` - The other color to combine with.
    /// * `percentage` - The percentage of the first color to keep.
    /// * `color_space` - The color space to combine the colors in.
    pub fn combine(&self, other: &Color, percentage: f32, color_space: &str) -> Color {
        match color_space {
            "RGB" => self.combine_rgb(other, percentage),
            "HSL" => self.combine_hsl(other, percentage),
            _ => panic!("Invalid color space"),
        }
    }

    /// Color::combine_rgb
    ///
    /// Combines two colors in RGB color space.
    ///
    /// * `other` - The other color to combine with.
    /// * `percentage` - The percentage of the first color to keep.
    pub fn combine_rgb(&self, other: &Color, percentage: f32) -> Color {
        let (r1, g1, b1) = self.split();
        let (r2, g2, b2) = other.split();

        let r = ((r1 as f32 * (1.0 - percentage) + r2 as f32 * percentage) as u32).min(255) as u8;
        let g = ((g1 as f32 * (1.0 - percentage) + g2 as f32 * percentage) as u32).min(255) as u8;
        let b = ((b1 as f32 * (1.0 - percentage) + b2 as f32 * percentage) as u32).min(255) as u8;

        Color::RGB(r, g, b)
    }

    /// Color::combine_hsl
    ///
    /// Combines two colors in HSL color space.
    ///
    /// * `other` - The other color to combine with.
    /// * `percentage` - The percentage of the first color to keep.
    pub fn combine_hsl(&self, other: &Color, percentage: f32) -> Color {
        match (self.to_hsl(), other.to_hsl()) {
            (Color::HSL(h1, s1, l1), Color::HSL(h2, s2, l2)) => {
                let h = ((h1 as f32 * (1.0 - percentage) + h2 as f32 * percentage) % 360.0) as u16;
                let s = s1 * (1.0 - percentage) + s2 * percentage;
                let l = l1 * (1.0 - percentage) + l2 * percentage;

                Color::HSL(h, s, l)
            }
            _ => unreachable!(),
        }
    }

    /// Color::split
    ///
    /// Splits the color into its RGB components.
    pub fn split(&self) -> (u8, u8, u8) {
        match self {
            Color::RGB(r, g, b) => (*r, *g, *b),
            Color::Hex(_) | Color::HSL(_, _, _) => self.to_rgb().split(),
        }
    }

    /// Color::to_rgb
    ///
    /// Converts the color to RGB format.
    pub fn to_rgb(&self) -> Color {
        match self {
            // Convert Hex to RGB
            Color::Hex(hex) => {
                let temp_color: u32 = u32::from_str_radix(hex, 16).expect("Failed to convert");
                let r = ((temp_color >> 16) & 0xFF) as u8;
                let g = ((temp_color >> 8) & 0xFF) as u8;
                let b = (temp_color & 0xFF) as u8;

                Color::RGB(r, g, b)
            }
            // Return RGB as is
            Color::RGB(r, g, b) => Color::RGB(r.clone(), g.clone(), b.clone()),
            // Convert HSL to RGB
            Color::HSL(h, s, l) => {
                let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
                let x = c * (1.0 - ((h.clone() as f32 / 60.0) % 2.0 - 1.0).abs());
                let m = l - c / 2.0;

                let (r, g, b) = if h < &60 {
                    (c, x, 0.0)
                } else if h < &120 {
                    (x, c, 0.0)
                } else if h < &180 {
                    (0.0, c, x)
                } else if h < &240 {
                    (0.0, x, c)
                } else if h < &300 {
                    (x, 0.0, c)
                } else {
                    (c, 0.0, x)
                };

                let r = ((r + m) * 255.0).round() as u8;
                let g = ((g + m) * 255.0).round() as u8;
                let b = ((b + m) * 255.0).round() as u8;

                Color::RGB(r, g, b)
            }
        }
    }

    /// Color::to_hex
    ///
    /// Converts the color to Hex format.
    pub fn to_hex(&self) -> Color {
        match self.split() {
            (r, g, b) => Color::Hex(format!("{:02x}{:02x}{:02x}", r, g, b)),
        }
    }

    /// Color::to_hsl
    ///
    /// Converts the color to HSL format.
    pub fn to_hsl(&self) -> Color {
        let (r, g, b) = self.split();

        let r = r as f64 / 255.0;
        let g = g as f64 / 255.0;
        let b = b as f64 / 255.0;

        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let delta = max - min;

        // Compute Lightness
        let l = (max + min) / 2.0;

        // Compute Saturation
        let s = if delta == 0.0 {
            0.0
        } else {
            delta / (1.0 - (2.0 * l - 1.0).abs())
        };

        // Compute Hue
        let h = if delta == 0.0 {
            0.0
        } else if max == r {
            60.0 * ((g - b) / delta % 6.0)
        } else if max == g {
            60.0 * ((b - r) / delta + 2.0)
        } else {
            60.0 * ((r - g) / delta + 4.0)
        };

        let h = if h < 0.0 { h + 360.0 } else { h };

        Color::HSL(h as u16, s as f32, l as f32)
    }

    /// Color::print
    ///
    /// Prints the color in the console.
    pub fn print(&self) {
        match self {
            Color::Hex(hex) => {
                println!("Hex - #{}", hex);
            }
            Color::RGB(r, g, b) => {
                println!("RGB - R: {}, G: {}, B: {}", r, g, b);
            }
            Color::HSL(h, s, l) => {
                println!("HSL - H: {}, S: {}, L: {}", h, s, l);
            }
        }
    }

    /// Darkens the color by a percentage.
    pub fn darken(&self, percentage: f32) -> Color {
        self.combine(&Color::RGB(0, 0, 0), percentage, "RGB")
    }

    /// Lightens the color by a percentage.
    pub fn lighten(&self, percentage: f32) -> Color {
        self.combine(&Color::RGB(255, 255, 255), percentage, "RGB")
    }

    /// Saturates the color by a percentage.
    pub fn saturate(&self, percentage: f32) -> Color {
        self.combine(&Color::HSL(0, 1.0, 0.5), percentage, "RGB")
    }

    /// Desaturates the color by a percentage.
    pub fn desaturate(&self, percentage: f32) -> Color {
        self.combine(&Color::HSL(0, 0.0, 0.5), percentage, "RGB")
    }

    /// Converts the color to grayscale.
    pub fn grayscale(&self) -> Color {
        self.desaturate(1.0)
    }

    /// Rotates the hue of the color by an angle.
    pub fn hue_rotate(&self, angle: u16) -> Color {
        match self.to_hsl() {
            Color::HSL(h, s, l) => Color::HSL((h + angle) % 360, s, l),
            _ => unreachable!(),
        }
    }

    /// Inverts the color.
    pub fn invert(&self) -> Color {
        self.hue_rotate(180)
    }
}