mod ball;

use crate::ball::Ball;
use ggez::conf::WindowMode;
use ggez::event::EventHandler;
use ggez::glam::{vec2, Vec2};
use ggez::graphics::{Color, DrawMode, DrawParam, Rect, Text};
use ggez::input::keyboard::KeyInput;
use ggez::winit::event::VirtualKeyCode;
use ggez::{event, graphics, Context, ContextBuilder, GameError, GameResult};

fn main() {
    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("verlet integration", "Tobinio")
        .window_mode(WindowMode::default().resizable(true).dimensions(600., 600.))
        .build()
        .expect("aieee, could not create ggez context!");

    let simulation = Simulation::new(&mut ctx);

    event::run(ctx, event_loop, simulation);
}

pub const PHYSIC_STEP_COUNT: i32 = 400;
pub const PHYSIC_STEP_LENGTH: f32 = 1.0 / PHYSIC_STEP_COUNT as f32;

pub const BORDER_RADIUS: f32 = 400.0;

struct Simulation {
    balls: Vec<Ball>,
    time_left: f32,
}

impl Simulation {
    pub fn new(ctx: &mut Context) -> Simulation {
        // ctx.gfx.add_font(
        //     "font",
        //     graphics::FontData::from_path(ctx, "/resources/Tangerine_Regular.ttf").unwrap(),
        // );

        let balls = vec![];

        Simulation {
            balls,
            time_left: 0.0,
        }
    }

    pub fn spawn_ball(&mut self) {
        self.balls.push(Ball::new())
    }

    pub fn split_balls(&mut self) {
        for ball_1 in 0..self.balls.len() {
            for ball_2 in (ball_1 + 1)..self.balls.len() {
                let ball_1_vec = self.balls.get(ball_1).unwrap().current_location;
                let ball_2_vec = self.balls.get(ball_2).unwrap().current_location;

                let dif = ball_1_vec - ball_2_vec;
                let length = dif.length();

                let range =
                    self.balls.get(ball_1).unwrap().size + self.balls.get(ball_2).unwrap().size;

                if length > range {
                    continue;
                }

                let to_offset = (range - length) * 0.2;

                self.balls.get_mut(ball_1).unwrap().current_location +=
                    dif.normalize_or_zero() * to_offset;
                self.balls.get_mut(ball_2).unwrap().current_location -=
                    dif.normalize_or_zero() * to_offset;
            }
        }
    }

    fn physics_tick(&mut self, dt: f32) {
        let gravity = Vec2::new(0., -1000.);

        for ball in &mut self.balls {
            ball.add_acceleration(gravity);
        }

        self.split_balls();

        for ball in &mut self.balls {
            ball.keep_in_circle()
        }

        for ball in &mut self.balls {
            ball.update(dt)
        }
    }
}

impl EventHandler for Simulation {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let dt = ctx.time.delta();

        self.time_left += dt.as_secs_f32();

        while self.time_left > PHYSIC_STEP_LENGTH {
            self.physics_tick(PHYSIC_STEP_LENGTH);

            self.time_left -= PHYSIC_STEP_LENGTH
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);

        canvas.set_screen_coordinates(Rect::new(-500.0, 500.0, 1000.0, -1000.0));

        let border_mesh = graphics::Mesh::new_circle(
            ctx,
            DrawMode::fill(),
            Vec2::ZERO,
            BORDER_RADIUS,
            0.1,
            Color::BLACK,
        )?;

        canvas.draw(&border_mesh, DrawParam::from(Vec2::ZERO));

        let ball_mesh =
            graphics::Mesh::new_circle(ctx, DrawMode::fill(), Vec2::ZERO, 1.0, 0.01, Color::RED)?;

        for ball in &self.balls {
            canvas.draw(
                &ball_mesh,
                DrawParam::from(ball.current_location).scale(vec2(ball.size, ball.size)),
            );
        }

        let fps = ctx.time.fps();
        let fps_display = Text::new(format!("FPS: {fps:.2}"));
        // When drawing through these calls, `DrawParam` will work as they are documented.
        canvas.draw(
            &fps_display,
            graphics::DrawParam::from([-500.0, 500.0])
                .scale(vec2(3.0, -3.0))
                .color(Color::BLACK),
        );

        //todo
        let fps = self.balls.len();
        let fps_display = Text::new(format!("Balls: {fps}"));
        // When drawing through these calls, `DrawParam` will work as they are documented.
        canvas.draw(
            &fps_display,
            graphics::DrawParam::from([-500.0, 470.0])
                .scale(vec2(3.0, -3.0))
                .color(Color::BLACK),
        );

        canvas.finish(ctx)
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        input: KeyInput,
        _repeated: bool,
    ) -> Result<(), GameError> {
        if let Some(code) = input.keycode {
            match code {
                VirtualKeyCode::Space => self.spawn_ball(),
                _ => {}
            }
        }

        Ok(())
    }
}
