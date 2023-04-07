/// A trait for types that act like RGB values.
pub trait RgbLike {
    /// The red channel.
    fn red(&self) -> u8;

    /// The green channel.
    fn green(&self) -> u8;

    /// The blue channel.
    fn blue(&self) -> u8;

    /// All three channels combined into an array.
    ///
    /// The first element must always be red, the next must be green, and the final must be blue.
    fn to_array(&self) -> [u8; 3] {
        [self.red(), self.green(), self.blue()]
    }

    /// Linearly interpolates from `a` to `b` by `interp`.
    fn lerp(interp: f32, a: impl RgbLike, b: impl RgbLike) -> Self
    where
        Self: Sized,
    {
        let interp = interp.clamp(0.0, 1.0);
        let [ar, ag, ab] = a.to_array();
        let [br, bg, bb] = b.to_array();

        Self::new_f32(
            ar as f32 + interp * (br as f32 - ar as f32),
            ag as f32 + interp * (bg as f32 - ab as f32),
            ab as f32 + interp * (bb as f32 - ab as f32),
        )
    }

    fn new(red: u8, green: u8, blue: u8) -> Self
    where
        Self: Sized;

    fn new_rgb(rgb: u32) -> Self
    where
        Self: Sized,
    {
        let red = ((rgb >> 16) & 0xFF) as u8;
        let green = ((rgb >> 8) & 0xFF) as u8;
        let blue = (rgb & 0xFF) as u8;
        Self::new(red, green, blue)
    }

    fn new_f32(red: f32, green: f32, blue: f32) -> Self
    where
        Self: Sized,
    {
        debug_assert!(!red.is_nan(), "red is NaN");
        debug_assert!(!green.is_nan(), "green is NaN");
        debug_assert!(!blue.is_nan(), "blue is NaN");

        Self::new(
            (red * 255.0) as u8,
            (green * 255.0) as u8,
            (blue * 255.0) as u8,
        )
    }

    fn from_hsv(hsv: impl HsvLike) -> Self
    where
        Self: Sized,
    {
        let s = hsv.s();
        let v = hsv.v();

        if s == 0.0 {
            return Self::new_f32(v, v, v);
        }

        let h = hsv.h() * 6.0;
        let i = h.floor();
        let f = h - i;
        let p = v * (1.0 - s);
        let q = v * (1.0 - s * f);
        let t = v * (1.0 - s * (1.0 - f));

        match i as i32 {
            0 => Self::new_f32(v, t, p),
            1 => Self::new_f32(q, v, p),
            2 => Self::new_f32(p, v, t),
            3 => Self::new_f32(p, q, v),
            4 => Self::new_f32(t, p, v),
            _ => Self::new_f32(v, p, q),
        }
    }

    fn into_hsv<Hsv: HsvLike>(self) -> Hsv
    where
        Self: Sized,
    {
        Hsv::from_rgb(self)
    }
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Default)]
pub struct Rgb {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl RgbLike for Rgb {
    fn red(&self) -> u8 {
        self.red
    }

    fn green(&self) -> u8 {
        self.green
    }

    fn blue(&self) -> u8 {
        self.blue
    }

    fn new(red: u8, green: u8, blue: u8) -> Self
    where
        Self: Sized,
    {
        Self { red, green, blue }
    }
}

/// A trait for types that act like HSV values.
pub trait HsvLike {
    /// The hue channel.
    fn h(&self) -> f32;

    /// The saturation channel.
    fn s(&self) -> f32;

    /// The value channel.
    fn v(&self) -> f32;

    fn new(hue: f32, saturation: f32, value: f32) -> Self
    where
        Self: Sized;

    fn from_rgb(rgb: impl RgbLike) -> Self
    where
        Self: Sized,
    {
        let r = rgb.red() as f32 / 255.0;
        let g = rgb.green() as f32 / 255.0;
        let b = rgb.blue() as f32 / 255.0;

        let min = r.min(g).min(b);
        let max = r.max(g).max(b);
        let delta = max - min;

        let s = if max != 0.0 { delta / max } else { 0.0 };

        if s == 0.0 {
            return Self::new(0.0, s, max);
        }

        let mut h = if r == max {
            (g - b) / delta
        } else if g == max {
            2.0 + (b - r) / delta
        } else {
            4.0 + (r - g) / delta
        };

        h *= 60.0;
        if h < 0.0 {
            h += 360.0;
        }

        Self::new(h / 360.0, s, max)
    }

    fn into_rgb<Rgb: RgbLike>(self) -> Rgb
    where
        Self: Sized,
    {
        Rgb::from_hsv(self)
    }
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct Hsv {
    pub hue: f32,
    pub saturation: f32,
    pub value: f32,
}

impl HsvLike for Hsv {
    fn h(&self) -> f32 {
        self.hue
    }

    fn s(&self) -> f32 {
        self.saturation
    }

    fn v(&self) -> f32 {
        self.value
    }

    fn new(hue: f32, saturation: f32, value: f32) -> Self
    where
        Self: Sized,
    {
        debug_assert!(!hue.is_nan(), "hue is NaN");
        debug_assert!(!saturation.is_nan(), "saturation is NaN");
        debug_assert!(!value.is_nan(), "value is NaN");
        Self {
            hue,
            saturation,
            value,
        }
    }
}
