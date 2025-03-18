use crate::*;

use graphics::CharacterCache;

pub struct Button {
    pub text: String,
    pub color: Color,
    pub text_color: Color,
    pub size: u32,
    pub pos: [f64; 2],
    pub width: f64,
    pub height: f64,
    pub padding: [f64; 2],
}

impl Button {
    pub fn new(
        text: &str,
        color: Color,
        text_color: Color,
        size: u32,
        pos: [f64; 2],
        padding: [f64; 2],
        glyphs: &mut GlyphCache,
    ) -> Self {
        let (width, height) = measure_text_dimensions(text, size, glyphs);

        Button {
            text: text.to_string(),
            color,
            text_color,
            size,
            pos,
            width,
            height,
            padding,
        }
    }

    pub fn draw(
        &self,
        ctx: &graphics::Context,
        graphics: &mut GlGraphics,
        glyphs: &mut GlyphCache,
    ) {
        // draw button also according to padding
        graphics::rectangle(
            self.color,
            [
                self.pos[0],
                self.pos[1],
                self.width + self.padding[0] * 2.0,
                self.height + self.padding[1] * 2.0,
            ],
            ctx.transform,
            graphics,
        );

        // graphics::rectangle(
        //     self.color,
        //     [self.pos[0], self.pos[1], self.width, self.height],
        //     ctx.transform,
        //     graphics,
        // );

        draw_text(
            ctx,
            graphics,
            glyphs,
            self.text_color,
            [
                self.pos[0] + self.padding[0],
                self.pos[1] + self.height - 1.0 + self.padding[1],
            ],
            &self.text,
            self.size,
        );
    }

    pub fn point_inside(&self, x: f64, y: f64) -> bool {
        x > self.pos[0]
            && y > self.pos[1]
            && x < self.pos[0] + self.width + self.padding[0] * 2.0
            && y < self.pos[1] + self.height + self.padding[1] * 2.0
    }
}
pub fn draw_text(
    ctx: &graphics::Context,
    graphics: &mut GlGraphics,
    glyphs: &mut GlyphCache,
    color: [f32; 4],
    pos: [f64; 2],
    text: &str,
    font_size: u32,
) {
    graphics::Text::new_color(color, font_size)
        .draw(
            text,
            glyphs,
            &ctx.draw_state,
            ctx.transform.trans(pos[0], pos[1]),
            graphics,
        )
        .unwrap();
}

pub fn measure_text_dimensions(text: &str, font_size: u32, glyphs: &mut GlyphCache) -> (f64, f64) {
    let mut w: f64 = 0.0;
    let mut h: f64 = 0.0;

    for ch in text.chars() {
        let character = glyphs.character(font_size, ch).unwrap();
        let top = character.top();
        w += character.advance_width();
        h = h.max(top);
    }

    (w, h)
}
