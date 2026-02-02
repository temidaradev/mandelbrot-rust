use macroquad::{miniquad::window, prelude::*};

type Complex = num_complex::Complex<f64>;

fn mandelbrot_color(c: &Complex, upper: u32) -> u32 {
    let mut z = Complex::ZERO;

    for i in 0..upper {
        z = z * z + c;
        if z.norm_sqr() > 4.0 {
            return i;
        }
    }

    upper
}

fn mandelbrot(data: &mut [[u8; 4]], from: &Complex, to: &Complex, width: u32, height: u32) {
    for y in 0..height {
        for x in 0..width {
            let c = Complex::new(
                from.re + (to.re - from.re) * x as f64 / width as f64,
                from.im + (to.im - from.im) * y as f64 / height as f64,
            );

            let iter = mandelbrot_color(&c, 255);
            let index = (y * width + x) as usize;

            data[index] = [255 - iter as u8, 255 - iter as u8, 255 - iter as u8, 255];
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
