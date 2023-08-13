use sdl2::pixels::Color;

pub fn rgb_to_hsv(c: Color) -> (f64, f64, f64) {
    let (r_u8, g_u8, b_u8) = c.rgb();
    let r = r_u8 as f64 / 255.0;
    let g = g_u8 as f64 / 255.0;
    let b = b_u8 as f64 / 255.0;

    let max = r.max(g.max(b));
    let min = r.min(g.min(b));
    let delta = max - min;

    let hue = if max == min {
        0.0
    } else if max == r {
        60.0 * ((g - b) / delta) % 360.0
    } else if max == g {
        (60.0 * ((b - r) / delta) + 120.0) % 360.0
    } else {
        (60.0 * ((r - g) / delta) + 240.0) % 360.0
    };
    let sat = if max == 0.0 {
        0.0
    } else {
        (delta / max) * 100.0
    };
    let val = max;

    (hue, sat, val)
}

pub fn hsv_to_rgb(hue: f64, sat: f64, val: f64) -> Color {
    let boosted_value = val * 255.0;
    let complement = boosted_value * (1.0 - sat / 100.0);
    let offset = (boosted_value - complement) * (1.0 - ((hue / 60.0) % 2.0 - 1.0).abs());

    if hue < 60.0 {
        Color::RGB(
            boosted_value as u8,
            (complement + offset) as u8,
            complement as u8,
        )
    } else if hue < 120.0 {
        Color::RGB(
            (complement + offset) as u8,
            boosted_value as u8,
            complement as u8,
        )
    } else if hue < 180.0 {
        Color::RGB(
            complement as u8,
            boosted_value as u8,
            (complement + offset) as u8,
        )
    } else if hue < 240.0 {
        Color::RGB(
            complement as u8,
            (complement + offset) as u8,
            boosted_value as u8,
        )
    } else if hue < 300.0 {
        Color::RGB(
            (complement + offset) as u8,
            complement as u8,
            boosted_value as u8,
        )
    } else {
        Color::RGB(
            boosted_value as u8,
            complement as u8,
            (complement + offset) as u8,
        )
    }
}

pub fn lerp_color(a: Color, b: Color, t: f64) -> Color {
    let (hue_a, sat_a, val_a) = rgb_to_hsv(a);
    let (hue_b, sat_b, val_b) = rgb_to_hsv(b);

    let hue_t = hue_b * t + hue_a * (1.0 - t);
    let sat_t = sat_b * t + sat_a * (1.0 - t);
    let val_t = val_b * t + val_a * (1.0 - t);

    hsv_to_rgb(hue_t, sat_t, val_t)
}