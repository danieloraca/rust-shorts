use macroquad::prelude::*;
use rodio::{Decoder, OutputStream, Sink};
use std::io::Cursor;

// Structure to represent a particle
struct Particle {
    position: Vec2,
    velocity: Vec2,
    color: Color,
    lifetime: f32,
}

impl Particle {
    fn update(&mut self, delta: f32) {
        self.position += self.velocity * delta;
        self.lifetime -= delta;
    }

    fn draw(&self) {
        if self.lifetime > 0.0 {
            draw_circle(self.position.x, self.position.y, 5.0, self.color);
        }
    }
}

#[macroquad::main("Explosion Editor")]
async fn main() {
    let particles_per_keypress = 20;
    let mut particles: Vec<Particle> = Vec::new();

    // Set up audio
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let explosion_sound = include_bytes!("explosion_sound.mp3"); // Your explosion sound file
    let explosion_cursor = Cursor::new(explosion_sound.to_vec());

    loop {
        // Check for keypress
        if is_key_pressed(KeyCode::Space) {
            // Trigger explosion effect with particles
            for _ in 0..particles_per_keypress {
                let angle = rand::gen_range(0.0, 2.0 * std::f32::consts::PI);
                let speed = rand::gen_range(50.0, 150.0);
                particles.push(Particle {
                    position: vec2(screen_width() / 2.0, screen_height() / 2.0),
                    velocity: vec2(angle.cos() * speed, angle.sin() * speed),
                    color: Color::new(
                        rand::gen_range(0.5, 1.0),
                        rand::gen_range(0.5, 1.0),
                        rand::gen_range(0.5, 1.0),
                        1.0,
                    ),
                    lifetime: rand::gen_range(0.5, 1.5),
                });
            }

            // Play explosion sound
            let sound_cursor = explosion_cursor.clone();
            let sink = Sink::try_new(&stream_handle).unwrap();
            sink.append(Decoder::new(sound_cursor).unwrap());
            sink.detach(); // Detach so it plays independently
        }

        // Update and draw particles
        clear_background(BLACK);
        let delta = get_frame_time();
        for particle in &mut particles {
            particle.update(delta);
            particle.draw();
        }
        particles.retain(|p| p.lifetime > 0.0);

        next_frame().await;
    }
}
