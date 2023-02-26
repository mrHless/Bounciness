use std::{thread, time};
use rand::{thread_rng, Rng};
use crossterm::{cursor, terminal, ClearType, AlternateScreen, Result};
use std::io::{stdout, Write};

struct Ball {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
}

impl Ball {
    fn new(x: f32, y: f32, vx: f32, vy: f32) -> Self {
        Self { x, y, vx, vy }
    }

    fn move_ball(&mut self) {
        self.x += self.vx;
        self.y += self.vy;
    }

    fn bounce(&mut self, width: f32, height: f32) {
        if self.x < 0.0 {
            self.vx = -self.vx;
            self.x = 0.0;
        } else if self.x > width {
            self.vx = -self.vx;
            self.x = width;
        }

        if self.y < 0.0 {
            self.vy = -self.vy;
            self.y = 0.0;
        } else if self.y > height {
            self.vy = -self.vy;
            self.y = height;
        }
    }
}

fn main() -> Result<()> {
    let mut ball = Ball::new(10.0, 5.0, 0.5, 0.2);
    let mut rng = thread_rng();
    let mut stdout = stdout();
    let mut i = 0;

    // Set terminal to alternate screen mode and hide the cursor
    terminal::enable_raw_mode()?;
    write!(stdout, "{}", AlternateScreen::to_string())?;
    write!(stdout, "{}", cursor::Hide)?;

    // Main loop
    loop {
        ball.move_ball();
        ball.bounce(80.0, 20.0);

        // Clear screen and redraw ball
        write!(stdout, "{}", ClearType::All)?;
        write!(stdout, "{}", cursor::MoveTo(ball.x as u16, ball.y as u16))?;
        write!(stdout, "o")?;

        // Sleep for a short time to slow down the animation
        thread::sleep(time::Duration::from_millis(50));

        // Randomly change the velocity every 10th iteration
        i += 1;
        if i % 10 == 0 {
            ball.vx = rng.gen_range(-1.0..1.0);
            ball.vy = rng.gen_range(-1.0..1.0);
        }
    }
}
