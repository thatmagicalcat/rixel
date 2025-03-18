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

const IMAGE_LEN: u32 = 80;
const IMAGE_VIEWPORT_LEN: f64 = WINDOW_HEIGHT as f64 - 20.0;
const SCALE: f64 = IMAGE_VIEWPORT_LEN / IMAGE_LEN as f64;

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
    buttons: Vec<ui::UIButton>,
}

impl<'a> App<'a> {
    fn new(opengl: OpenGL, mut glyph_cache: GlyphCache<'a>) -> Self {
        let mut image_buffer = RgbaImage::new(IMAGE_LEN, IMAGE_LEN);
        image_buffer
            .get_mut(..)
            .unwrap()
            .iter_mut()
            .for_each(|i| *i = 127);

        let texture = Texture::from_image(
            &image_buffer,
            &TextureSettings::new().filter(opengl_graphics::Filter::Nearest),
        );

        Self {
            buttons: vec![ui::UIButton::new(
                "button with automatic dimension calculation",
                color::WHITE,
                color::RED,
                17,
                [910.0, 10.0],
                [10.0, 10.0],
                &mut glyph_cache,
            )],
            gl: GlGraphics::new(opengl),

            glyph_cache,
            image_buffer,
            texture,
        }
    }

    fn render(&mut self, args: &RenderArgs) {
        self.gl.draw(args.viewport(), |c, gl| {
            graphics::image(
                &self.texture,
                c.transform.trans(10.0, 10.0).scale(SCALE, SCALE),
                gl,
            );

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

            for btn in &self.buttons {
                btn.draw(&c, gl, &mut self.glyph_cache);
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs) {}

    fn mouse_pos(&mut self, [x, y]: [f64; 2]) {
        for b in &mut self.buttons {
            b.color = if b.is_over(x, y) {
                color::GRAY
            } else {
                color::WHITE
            };
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

        e.mouse_cursor(|pos| app.mouse_pos(pos));
    }
}
