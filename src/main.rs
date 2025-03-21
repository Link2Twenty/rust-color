mod color;
use color::Color;

fn main() {
    let hex_tomato = Color::Hex(String::from("ff6347")).to_rgb();
    let color_test = Color::HSL(243, 1.0, 0.38).to_rgb();
    let color_mix = hex_tomato.combine(&color_test, 0.5, "RGB").to_rgb();
    let hsl_color_mix = hex_tomato.combine(&color_test, 0.5, "HSL").to_rgb();
    let darken_test = hex_tomato.darken(0.2).to_rgb();
    let lighten_test = hex_tomato.lighten(0.2).to_rgb();
    let saturate_test = hex_tomato.saturate(0.2).to_rgb();
    let desaturate_test = hex_tomato.desaturate(0.2).to_rgb();
    let grayscale_test = hex_tomato.grayscale().to_rgb();
    let rotate_test = hex_tomato.hue_rotate(90).to_rgb();
    let invert_test = hex_tomato.invert().to_rgb();

    // Print the colors
    hex_tomato.print();
    color_test.print();
    color_mix.print();
    hsl_color_mix.print();
    darken_test.print();
    lighten_test.print();
    saturate_test.print();
    desaturate_test.print();
    grayscale_test.print();
    rotate_test.print();
    invert_test.print();

    hex_tomato.to_hex().print();
    color_test.to_hex().print();
    color_mix.to_hex().print();
    hsl_color_mix.to_hex().print();
    darken_test.to_hex().print();
    lighten_test.to_hex().print();
    saturate_test.to_hex().print();
    desaturate_test.to_hex().print();
    grayscale_test.to_hex().print();
    rotate_test.to_hex().print();
    invert_test.to_hex().print();

    hex_tomato.to_hsl().print();
    color_test.to_hsl().print();
    color_mix.to_hsl().print();
    hsl_color_mix.to_hsl().print();
    darken_test.to_hsl().print();
    lighten_test.to_hsl().print();
    saturate_test.to_hsl().print();
    desaturate_test.to_hsl().print();
    grayscale_test.to_hsl().print();
    rotate_test.to_hsl().print();
    invert_test.to_hsl().print();
}
