use macroquad::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct Camera2DWorld {
    pos: Vec2,   // centre de la caméra dans le monde
    zoom: f32,   // pixels par unité monde
}

impl Camera2DWorld {
    pub fn new() -> Self {
        Self {
            pos: vec2(0.0, 0.0),
            zoom: 1.0,
        }
    }

    fn screen_center(&self) -> Vec2 {
        vec2(screen_width() * 0.5, screen_height() * 0.5)
    }

    pub fn world_to_screen(&self, world: Vec2) -> Vec2 {
        let center = self.screen_center();
        (world - self.pos) * self.zoom + center
    }

    fn screen_to_world(&self, screen: Vec2) -> Vec2 {
        let center = self.screen_center();
        (screen - center) / self.zoom + self.pos
    }

    pub fn pan_pixels(&mut self, delta_pixels: Vec2) {
        self.pos -= delta_pixels / self.zoom;
    }

    pub fn zoom_at(&mut self, mouse_screen: Vec2, zoom_factor: f32) {
        let before = self.screen_to_world(mouse_screen);

        self.zoom *= zoom_factor;
        self.zoom = self.zoom.clamp(1.0, 500.0);

        let after = self.screen_to_world(mouse_screen);

        self.pos += before - after;
    }
}

fn compute_grid_step(cam: &Camera2DWorld) -> f32 {
    let target_pixels = 80.0;
    let raw_step = target_pixels / cam.zoom;

    let power = raw_step.log10().floor();
    let base = 10.0_f32.powf(power);

    let candidates = [1.0, 2.0, 5.0, 10.0];

    for c in candidates {
        let step = base * c;
        if step >= raw_step {
            return step;
        }
    }

    base * 10.0
}

fn draw_grid(cam: &Camera2DWorld) {
    let top_left = cam.screen_to_world(vec2(0.0, 0.0));
    let bottom_right = cam.screen_to_world(vec2(screen_width(), screen_height()));

    let min_x = top_left.x.min(bottom_right.x);
    let max_x = top_left.x.max(bottom_right.x);
    let min_y = top_left.y.min(bottom_right.y);
    let max_y = top_left.y.max(bottom_right.y);

    let step = compute_grid_step(cam);

    let start_x = (min_x / step).floor() * step;
    let start_y = (min_y / step).floor() * step;

    let mut x = start_x;
    while x <= max_x {
        let a = cam.world_to_screen(vec2(x, min_y));
        let b = cam.world_to_screen(vec2(x, max_y));

        let is_main = x.abs() < step * 0.5;
        let thickness = if is_main { 3.0 } else { 1.0 };
        let color = if is_main { DARKGRAY } else { LIGHTGRAY };

        draw_line(a.x, a.y, b.x, b.y, thickness, color);
        x += step;
    }

    let mut y = start_y;
    while y <= max_y {
        let a = cam.world_to_screen(vec2(min_x, y));
        let b = cam.world_to_screen(vec2(max_x, y));

        let is_main = y.abs() < step * 0.5;
        let thickness = if is_main { 3.0 } else { 1.0 };
        let color = if is_main { DARKGRAY } else { LIGHTGRAY };

        draw_line(a.x, a.y, b.x, b.y, thickness, color);
        y += step;
    }
}

fn draw_axes(cam: &Camera2DWorld) {
    let top_left = cam.screen_to_world(vec2(0.0, 0.0));
    let bottom_right = cam.screen_to_world(vec2(screen_width(), screen_height()));

    let min_x = top_left.x.min(bottom_right.x);
    let max_x = top_left.x.max(bottom_right.x);
    let min_y = top_left.y.min(bottom_right.y);
    let max_y = top_left.y.max(bottom_right.y);

    let x_axis_a = cam.world_to_screen(vec2(min_x, 0.0));
    let x_axis_b = cam.world_to_screen(vec2(max_x, 0.0));
    draw_line(x_axis_a.x, x_axis_a.y, x_axis_b.x, x_axis_b.y, 2.0, RED);

    let y_axis_a = cam.world_to_screen(vec2(0.0, min_y));
    let y_axis_b = cam.world_to_screen(vec2(0.0, max_y));
    draw_line(y_axis_a.x, y_axis_a.y, y_axis_b.x, y_axis_b.y, 2.0, BLUE);
}

fn draw_world_point(cam: &Camera2DWorld, p: Vec2, radius_world: f32, color: Color) {
    let screen_pos = cam.world_to_screen(p);
    let radius_screen = radius_world * cam.zoom;
    draw_circle(screen_pos.x, screen_pos.y, radius_screen.max(2.0), color);
}

