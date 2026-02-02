use macroquad::{miniquad::window, prelude::*};

type Complex = num_complex::Complex<f64>;

fn hsv_to_rgb(h: f64, s: f64, v: f64) -> [u8; 3] {
    let c = v * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;

    let (r, g, b) = match h {
        h if h < 60.0 => (c, x, 0.0),
        h if h < 120.0 => (x, c, 0.0),
        h if h < 180.0 => (0.0, c, x),
        h if h < 240.0 => (0.0, x, c),
        h if h < 300.0 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };

    [
        ((r + m) * 255.0) as u8,
        ((g + m) * 255.0) as u8,
        ((b + m) * 255.0) as u8,
    ]
}

fn mandelbrot_color(c: &Complex, upper: u32) -> f64 {
    let mut z = Complex::ZERO;

    for i in 0..upper {
        z = z * z + c;

        let norm_sqr = z.norm_sqr();
        if norm_sqr > 4.0 {
            let log_zn = norm_sqr.ln() / 2.0;
            let nu = (log_zn / std::f64::consts::LN_2).ln() / std::f64::consts::LN_2;
            return i as f64 + 1.0 - nu;
        }
    }

    upper as f64
}

fn mandelbrot(data: &mut [[u8; 4]], from: &Complex, to: &Complex, width: u32, height: u32) {
    let max_iter = 255;
    for y in 0..height {
        for x in 0..width {
            let c = Complex::new(
                from.re + (to.re - from.re) * x as f64 / width as f64,
                from.im + (to.im - from.im) * y as f64 / height as f64,
            );

            let mu = mandelbrot_color(&c, max_iter);
            let index = (y * width + x) as usize;

            if mu >= max_iter as f64 {
                data[index] = [0, 0, 0, 255];
            } else {
                let t = (mu / max_iter as f64).powf(0.3);
                let hue = 360.0 * t;

                let rgb = hsv_to_rgb(hue, 1.0, 1.0);
                data[index] = [rgb[0], rgb[1], rgb[2], 255];
            }
        }
    }
}

#[macroquad::main("Mandelbrot-rust")]
async fn main() {
    let (mut width, mut height) = window::screen_size();

    let mut from = Complex::new(-1.7, -1.3);
    let mut to = Complex::new(1.0, 1.3);

    let mut image = Image::gen_image_color(width as u16, height as u16, BLACK);
    let mut bmp = Texture2D::from_image(&image);

    let mut selecting = false;
    let mut tl = Vec2::ZERO;

    mandelbrot(
        image.get_image_data_mut(),
        &from,
        &to,
        width as u32,
        height as u32,
    );
    bmp.update(&image);
    loop {
        let mut recalc = false;
        if is_mouse_button_pressed(MouseButton::Right) {
            from = Complex::new(-1.7, -1.3);
            to = Complex::new(1.0, 1.3);
            recalc = true;
        }

        if !selecting && is_mouse_button_down(MouseButton::Left) {
            selecting = true;
            tl = Vec2::from(mouse_position());
        } else if selecting && is_mouse_button_released(MouseButton::Left) {
            selecting = false;
            let br = Vec2::from(mouse_position());
            let size = to - from;
            from = Complex::new(
                from.re + tl.x as f64 * size.re / width as f64,
                from.im + tl.y as f64 * size.im / height as f64,
            );
            to = Complex::new(
                from.re + size.im * (br.x - tl.x) as f64 / width as f64,
                from.im + size.im * (br.y - tl.y) as f64 / height as f64,
            );
            recalc = true;
        }
        if recalc {
            if image.width != width as u16 || image.height != height as u16 {
                image = Image::gen_image_color(width as u16, height as u16, BLACK);
                bmp = Texture2D::from_image(&image);
            }
            mandelbrot(
                image.get_image_data_mut(),
                &from,
                &to,
                width as u32,
                height as u32,
            );
            bmp.update(&image);
        }
        let params = DrawTextureParams {
            dest_size: Some(Vec2::new(width, height)),
            ..Default::default()
        };
        draw_texture_ex(&bmp, 0.0, 0.0, WHITE, params);
        if selecting {
            let br = Vec2::from(mouse_position());
            draw_rectangle_lines(tl.x, tl.y, br.x - tl.x, br.y - tl.y, 2.0, RED);
        }
        next_frame().await
    }
}
