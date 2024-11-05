use macroquad::prelude::*;
use rodio::{Decoder, OutputStream, Sink};
use std::io::Cursor;

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
            draw_circle(self.position.x, self.position.y, 2.0, self.color);
        }
    }
}

fn any_key_pressed() -> Option<char> {
    if is_key_pressed(KeyCode::A) {
        return Some('A');
    }
    if is_key_pressed(KeyCode::B) {
        return Some('B');
    }
    if is_key_pressed(KeyCode::C) {
        return Some('C');
    }
    if is_key_pressed(KeyCode::D) {
        return Some('D');
    }
    if is_key_pressed(KeyCode::E) {
        return Some('E');
    }
    if is_key_pressed(KeyCode::F) {
        return Some('F');
    }
    if is_key_pressed(KeyCode::G) {
        return Some('G');
    }
    if is_key_pressed(KeyCode::H) {
        return Some('H');
    }
    if is_key_pressed(KeyCode::I) {
        return Some('I');
    }
    if is_key_pressed(KeyCode::J) {
        return Some('J');
    }
    if is_key_pressed(KeyCode::K) {
        return Some('K');
    }
    if is_key_pressed(KeyCode::L) {
        return Some('L');
    }
    if is_key_pressed(KeyCode::M) {
        return Some('M');
    }
    if is_key_pressed(KeyCode::N) {
        return Some('N');
    }
    if is_key_pressed(KeyCode::O) {
        return Some('O');
    }
    if is_key_pressed(KeyCode::P) {
        return Some('P');
    }
    if is_key_pressed(KeyCode::Q) {
        return Some('Q');
    }
    if is_key_pressed(KeyCode::R) {
        return Some('R');
    }
    if is_key_pressed(KeyCode::S) {
        return Some('S');
    }
    if is_key_pressed(KeyCode::T) {
        return Some('T');
    }
    if is_key_pressed(KeyCode::U) {
        return Some('U');
    }
    if is_key_pressed(KeyCode::V) {
        return Some('V');
    }
    if is_key_pressed(KeyCode::W) {
        return Some('W');
    }
    if is_key_pressed(KeyCode::X) {
        return Some('X');
    }
    if is_key_pressed(KeyCode::Y) {
        return Some('Y');
    }
    if is_key_pressed(KeyCode::Z) {
        return Some('Z');
    }
    if is_key_pressed(KeyCode::Space) {
        return Some(' ');
    }

    None
}

#[macroquad::main("Cursor Explosion Editor")]
async fn main() {
    let particles_per_keypress = 5;
    let mut particles: Vec<Particle> = Vec::new();
    let mut text_buffer = String::new(); // Buffer to store typed characters

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let explosion_sound = include_bytes!("explosion_sound.mp3");
    let delete_sound = include_bytes!("delete_sound.mp3");
    let explosion_cursor = Cursor::new(explosion_sound.to_vec());
    let delete_cursor = Cursor::new(delete_sound.to_vec());

    let font_size = 30.0;
    let text_start_x = 40.0;
    let text_start_y = 50.0;

    loop {
        if is_key_pressed(KeyCode::Backspace) && !text_buffer.is_empty() {
            let sound_cursor = delete_cursor.clone();
            let sink = Sink::try_new(&stream_handle).unwrap();
            sink.append(Decoder::new(sound_cursor).unwrap());
            sink.detach();
            text_buffer.pop();
        }
        // Check for key press and add character to text buffer
        if let Some(character) = any_key_pressed() {
            text_buffer.push(character);

            // Calculate the explosion position based on the latest character position
            let text_width = measure_text(&text_buffer, None, font_size as u16, 1.0).width;
            let explosion_position = vec2(text_start_x + text_width, text_start_y);

            for _ in 0..particles_per_keypress {
                let angle = rand::gen_range(0.0, 2.0 * std::f32::consts::PI);
                let speed = rand::gen_range(50.0, 150.0);
                particles.push(Particle {
                    position: explosion_position,
                    velocity: vec2(angle.cos() * speed, angle.sin() * speed),
                    color: Color::new(
                        rand::gen_range(0.7, 1.0),
                        rand::gen_range(0.3, 0.9),
                        rand::gen_range(0.3, 0.9),
                        1.0,
                    ),
                    lifetime: rand::gen_range(0.5, 1.5),
                });
            }

            // Play explosion sound
            let sound_cursor = explosion_cursor.clone();
            let sink = Sink::try_new(&stream_handle).unwrap();
            sink.append(Decoder::new(sound_cursor).unwrap());
            sink.detach();
        }

        // Update and draw particles
        clear_background(BLACK);
        let delta = get_frame_time();
        for particle in &mut particles {
            particle.update(delta);
            particle.draw();
        }
        particles.retain(|p| p.lifetime > 0.0);

        // Draw the text buffer
        draw_text(&text_buffer, text_start_x, text_start_y, font_size, WHITE);

        next_frame().await;
    }
}
