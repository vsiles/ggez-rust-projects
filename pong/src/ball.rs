use ggez::graphics;
use ggez::graphics::DrawParam;
use ggez::Context;
use ggez::GameResult;

use cgmath::{Point2, Vector2};

use super::paddle::Paddle;

pub struct Ball {
    pub xy: Point2<f32>,
    size: Point2<f32>,
    pub delta: Vector2<f32>,
}

impl Ball {
    pub fn collides(&self, paddle: &Paddle) -> bool {
        if self.xy.x > paddle.xy.x + paddle.size.x || paddle.xy.x > self.xy.x + self.size.x {
            return false;
        }

        if self.xy.y > paddle.xy.y + paddle.size.y || paddle.xy.y > self.xy.y + self.size.y {
            return false;
        }

        return true;
    }

    pub fn new() -> Ball {
        let xy = Point2::new(super::WIDTH / 2.0 - 6.0, super::HEIGHT / 2.0 - 6.0);

        let size = Point2::new(12.0, 12.0);
        let delta = Vector2::new(0.0, 0.0);
        Ball { xy, size, delta }
    }

    pub fn reset(&mut self) {
        self.xy.x = super::WIDTH / 2.0 - 6.0;
        self.xy.y = super::HEIGHT / 2.0 - 6.0;
        self.size.x = 12.0;
        self.size.y = 12.0;
        self.delta.x = 0.0;
        self.delta.y = 0.0;
    }

    pub fn update(&mut self, dt: f32) {
        self.xy = self.xy + dt * self.delta
    }

    pub fn render(&self, ctx: &mut Context) -> GameResult<()> {
        let rect = graphics::Rect::new(self.xy.x, self.xy.y, self.size.x, self.size.y);
        let mesh =
            graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, graphics::Color::WHITE)?;
        graphics::draw(ctx, &mesh, DrawParam::default())
    }
}
