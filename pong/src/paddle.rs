use ggez::graphics;
use ggez::graphics::DrawParam;
use ggez::Context;
use ggez::GameResult;

use cgmath::Point2;

pub struct Paddle {
    pub xy: Point2<f32>,
    pub size: Point2<f32>,
    pub dy: f32,
}

impl Paddle {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Paddle {
        let xy = Point2::new(x, y);
        let size = Point2::new(width, height);
        Paddle { xy, size, dy: 0.0 }
    }

    pub fn update(&mut self, dt: f32) {
        if self.dy < 0.0 {
            let limit = 0.0 as f32;
            self.xy.y = limit.max(self.xy.y + self.dy * dt)
        } else {
            let h = super::HEIGHT - self.size.y;
            self.xy.y = h.min(self.xy.y + self.dy * dt)
        }
    }

    pub fn render(&self, ctx: &mut Context) -> GameResult<()> {
        let rect = graphics::Rect::new(self.xy.x, self.xy.y, self.size.x, self.size.y);
        let mesh =
            graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, graphics::WHITE)?;
        graphics::draw(ctx, &mesh, DrawParam::default())
    }
}
