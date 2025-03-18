use piston::*;

use graphics::Transformed;
use opengl_graphics::GlGraphics;
use opengl_graphics::GlyphCache;
use opengl_graphics::Texture;
use opengl_graphics::TextureSettings;

use glfw_window::GlfwWindow;
use glfw_window::OpenGL;

use image::ImageBuffer;
use image::Rgba;
use image::RgbaImage;

mod ui;

const WINDOW_WIDTH: u32 = 1400;
const WINDOW_HEIGHT: u32 = 900;

const IMAGE_LEN: u32 = 24;
const IMAGE_VIEWPORT_LEN: f64 = WINDOW_HEIGHT as f64 - 20.0;

// scale
const PIXEL_SIZE: f64 = IMAGE_VIEWPORT_LEN / IMAGE_LEN as f64;

pub type Color = [f32; 4];

macro_rules! colors {
    [ $($name:ident => [$r:expr, $g:expr, $b:expr]),* $(,)? ] => {
        #[allow(unused)]
        mod color {$(
            pub const $name: super::Color = [$r, $g, $b, 1.0];
        )*}
    };
}

colors! {
    RED => [1.0, 0.0, 0.0],
    GREEN => [0.0, 1.0, 0.0],
    BLUE => [0.0, 0.0, 1.0],
    WHITE => [1.0, 1.0, 1.0],
    GRAY => [0.5, 0.5, 0.5],
    BLACK => [0.0, 0.0, 0.0],
}

struct App<'a> {
    gl: GlGraphics,
    glyph_cache: GlyphCache<'a>,
    image_buffer: ImageBuffer<Rgba<u8>, Vec<u8>>,
    texture: Texture,
    buttons: Vec<ui::Button>,

    inside_canvas: bool,
    mouse_down: bool,
    mouse_pos: [f64; 2],
}

impl<'a> App<'a> {
    fn new(opengl: OpenGL, mut glyph_cache: GlyphCache<'a>) -> Self {
        let mut image_buffer = RgbaImage::new(IMAGE_LEN, IMAGE_LEN);
        image_buffer
            .get_mut(..)
            .unwrap()
            .chunks_mut(4)
            .for_each(|i| i.copy_from_slice(&[0, 0, 0, !0]));

        let texture = Texture::from_image(
            &image_buffer,
            &TextureSettings::new().filter(opengl_graphics::Filter::Nearest),
        );

        Self {
            buttons: vec![ui::Button::new(
                "clear",
                color::WHITE,
                color::BLACK,
                22,
                [910.0, 420.0],
                [10.0, 10.0],
                &mut glyph_cache,
            )],
            gl: GlGraphics::new(opengl),
            glyph_cache,
            image_buffer,
            texture,

            inside_canvas: false,
            mouse_down: false,
            mouse_pos: [-1.0; 2],
        }
    }

    fn render(&mut self, args: &RenderArgs) {
        let i = ((self.mouse_pos[0] - 10.0) / PIXEL_SIZE).abs().floor();
        let j = ((self.mouse_pos[1] - 10.0) / PIXEL_SIZE).abs().floor();

        let active_x = 10.0 + PIXEL_SIZE * i;
        let active_y = 10.0 + PIXEL_SIZE * j;

        self.gl.draw(args.viewport(), |c, gl| {
            graphics::clear(color::BLACK, gl);

            // canvas border
            graphics::rectangle(
                if self.inside_canvas {
                    color::GREEN
                } else {
                    color::RED
                },
                graphics::rectangle::rectangle_by_corners(9.0, 9.0, 891.0, 891.0),
                c.transform,
                gl,
            );

            // canvas
            graphics::image(
                &self.texture,
                c.transform.trans(10.0, 10.0).scale(PIXEL_SIZE, PIXEL_SIZE),
                gl,
            );

            // gridlines
            let count = (IMAGE_VIEWPORT_LEN / PIXEL_SIZE) as u32;
            for i in 0..count {
                graphics::rectangle(
                    [0.1, 0.1, 0.1, 1.0],
                    graphics::rectangle::rectangle_by_corners(
                        10.0 + i as f64 * PIXEL_SIZE,
                        10.0,
                        10.0 + i as f64 * PIXEL_SIZE + 1.0,
                        890.0,
                    ),
                    c.transform,
                    gl,
                );

                graphics::rectangle(
                    [0.1, 0.1, 0.1, 1.0],
                    graphics::rectangle::rectangle_by_corners(
                        10.0,
                        10.0 + i as f64 * PIXEL_SIZE,
                        890.0,
                        10.0 + i as f64 * PIXEL_SIZE + 1.0,
                    ),
                    c.transform,
                    gl,
                );
            }

            // active pixel
            if active_x < 890.0 && active_y < 890.0 {
                graphics::rectangle(
                    color::WHITE,
                    graphics::rectangle::rectangle_by_corners(
                        active_x + 1.0,
                        active_y + 1.0,
                        active_x + PIXEL_SIZE,
                        active_y + PIXEL_SIZE,
                    ),
                    c.transform,
                    gl,
                );
            }

            // separator
            graphics::rectangle(
                color::WHITE,
                graphics::rectangle::rectangle_by_corners(
                    WINDOW_HEIGHT as _,
                    10.0,
                    WINDOW_HEIGHT as f64 + 2.0,
                    WINDOW_HEIGHT as f64 - 10.0,
                ),
                c.transform,
                gl,
            );

            // sidebar elements
            // buttons
            for btn in &self.buttons {
                btn.draw(&c, gl, &mut self.glyph_cache);
            }

            // viewport border
            graphics::rectangle(
                color::GRAY,
                graphics::rectangle::rectangle_by_corners(909.0, 9.0, 1311., 411.0),
                c.transform,
                gl,
            );

            // viewport
            graphics::image(
                &self.texture,
                c.transform
                    .trans(910.0, 10.0)
                    .scale(400.0 / IMAGE_LEN as f64, 400.0 / IMAGE_LEN as f64),
                gl,
            );
        });
    }

    fn update(&mut self, args: &UpdateArgs) {}

    fn mouse_pos(&mut self, p @ [x, y]: [f64; 2]) {
        self.mouse_pos = p;
        for b in &mut self.buttons {
            if b.point_inside(x, y) {
                b.color = color::GRAY;
            } else {
                b.color = color::WHITE;
            }
        }

        self.inside_canvas = x > 10.0 && x < 890.0 && y > 10.0 && y < 890.0;
    }

    fn draw(&mut self) {
        let i = ((self.mouse_pos[0] - 10.0) / PIXEL_SIZE).abs().floor();
        let j = ((self.mouse_pos[1] - 10.0) / PIXEL_SIZE).abs().floor();

        if let Some(pixel) = self.image_buffer.get_pixel_mut_checked(i as _, j as _) {
            pixel.0 = [!0; 4];
        }

        self.texture.update(&self.image_buffer);
    }

    fn handle_events(&mut self, e: Event) {
        e.mouse_cursor(|pos| self.mouse_pos(pos));

        e.button(|btn| match (btn.state, btn.button) {
            (ButtonState::Press, Button::Mouse(MouseButton::Left)) => self.mouse_down = true,
            (ButtonState::Release, Button::Mouse(MouseButton::Left)) => self.mouse_down = false,
            _ => {}
        });

        if self.inside_canvas && self.mouse_down {
            self.draw();
        }

        if !self.inside_canvas && self.mouse_down {
            for b in &self.buttons {
                if matches!(b.color, color::GRAY) {
                    #[allow(clippy::single_match)]
                    match b.text.as_str() {
                        "clear" => {
                            self.image_buffer
                                .get_mut(..)
                                .unwrap()
                                .chunks_mut(4)
                                .for_each(|i| i.copy_from_slice(&[0, 0, 0, !0]));

                            self.texture.update(&self.image_buffer);
                        }

                        _ => {}
                    }
                }
            }
        }
    }
}

fn main() {
    let opengl = OpenGL::V3_3;
    let mut window: GlfwWindow = WindowSettings::new("Rix3l", [WINDOW_WIDTH, WINDOW_HEIGHT])
        .graphics_api(opengl)
        .resizable(false)
        .exit_on_esc(true)
        .build()
        .unwrap();

    gl::load_with(|s| window.window.get_proc_address(s));

    let font_bytes = include_bytes!("../JetBrainsFont.ttf");
    let mut app = App::new(
        opengl,
        GlyphCache::from_bytes(font_bytes, (), TextureSettings::new()).unwrap(),
    );

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }

        app.handle_events(e);
    }
}
