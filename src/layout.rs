use crate::ui;

pub trait Widget {
    fn size(&self) -> (f64, f64);
    fn draw(&mut self, ctx: &graphics::Context, gl: &mut opengl_graphics::GlGraphics);
}

impl Widget for ui::Button<'_> {
    fn size(&self) -> (f64, f64) {
        (self.width, self.height)
    }

    fn draw(&mut self, ctx: &graphics::Context, gl: &mut opengl_graphics::GlGraphics) {
        self.render(ctx, gl);
    }
}

pub struct Spacing {
    pub x: Option<f64>,
    pub y: Option<f64>,
}

impl Spacing {
    pub fn new() -> Self {
        Self { x: None, y: None }
    }

    pub fn with_x(mut self, x: f64) -> Self {
        self.x = Some(x);
        self
    }

    pub fn with_y(mut self, y: f64) -> Self {
        self.y = Some(y);
        self
    }

    pub fn with_position(mut self, x: f64, y: f64) -> Self {
        self.x = Some(x);
        self.y = Some(y);
        self
    }
}

impl Widget for Spacing {
    fn size(&self) -> (f64, f64) {
        (self.x.unwrap_or(0.0), self.y.unwrap_or(0.0))
    }

    fn draw(&mut self, _: &graphics::Context, _: &mut opengl_graphics::GlGraphics) {}
}

/// Sets the appropriate position for each widget in the slice
pub fn arrange_horizontally(widgets: &[&dyn Widget]) -> Vec<(f64, f64)> {
    let mut x = 0.0;
    let mut max_y: f64 = 0.0;
    let mut positions = Vec::new();

    for widget in widgets {
        let size = widget.size();

        x += size.0;
        max_y = max_y.max(size.1);

        positions.push((x, size.1));
    }

    positions
}

pub fn arrange_vertically(widgets: &[&dyn Widget]) -> Vec<(f64, f64)> {
    let mut y = 0.0;
    let mut max_x: f64 = 0.0;
    let mut positions = Vec::new();

    for widget in widgets {
        let size = widget.size();

        y += size.1;
        max_x = max_x.max(size.0);

        positions.push((size.0, y));
    }

    positions
}