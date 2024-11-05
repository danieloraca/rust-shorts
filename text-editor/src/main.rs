use macroquad::prelude::*;
use rodio::{Decoder, OutputStream, Sink};
use std::io::Cursor;

const CURSOR_BLINK_INTERVAL: f32 = 0.5; // Blink interval in seconds
const SCROLL_SPEED: f32 = 20.0; // Scroll speed per scroll input
const MAX_VISIBLE_LINES: usize = 10; // Number of lines that can be visible without scrolling

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
    if is_key_pressed(KeyCode::Key1) {
        return Some('1');
    }
    if is_key_pressed(KeyCode::Key2) {
        return Some('2');
    }
    if is_key_pressed(KeyCode::Key3) {
        return Some('3');
    }
    if is_key_pressed(KeyCode::Key4) {
        return Some('4');
    }
    if is_key_pressed(KeyCode::Key5) {
        return Some('5');
    }
    if is_key_pressed(KeyCode::Key6) {
        return Some('6');
    }
    if is_key_pressed(KeyCode::Key7) {
        return Some('7');
    }
    if is_key_pressed(KeyCode::Key8) {
        return Some('8');
    }
    if is_key_pressed(KeyCode::Key9) {
        return Some('9');
    }
    if is_key_pressed(KeyCode::Key0) {
        return Some('0');
    }
    if is_key_pressed(KeyCode::Minus) {
        return Some('-');
    }
    if is_key_pressed(KeyCode::Backslash) {
        return Some('\\');
    }
    if is_key_pressed(KeyCode::LeftBracket) {
        return Some('[');
    }
    if is_key_pressed(KeyCode::RightBracket) {
        return Some(']');
    }
    if is_key_pressed(KeyCode::Semicolon) {
        return Some(';');
    }
    if is_key_pressed(KeyCode::Comma) {
        return Some(',');
    }
    if is_key_pressed(KeyCode::Period) {
        return Some('.');
    }
    if is_key_pressed(KeyCode::Slash) {
        return Some('/');
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

    let mut cursor_visible = true;
    let mut cursor_timer = 0.0;

    let mut scroll_offset = 0.0; // Vertical scroll position
    let mut line_height = font_size * 1.0; // Height of each line

    loop {
        cursor_timer += get_frame_time();
        if cursor_timer >= CURSOR_BLINK_INTERVAL {
            cursor_visible = !cursor_visible;
            cursor_timer = 0.0;
        }

        // Handle scrolling up and down
        if is_key_pressed(KeyCode::Up) {
            scroll_offset += SCROLL_SPEED;
            if scroll_offset > 0.0 {
                scroll_offset = 0.0; // Prevent scrolling past the top
            }
        } else if is_key_pressed(KeyCode::Down) {
            scroll_offset -= SCROLL_SPEED;
        }

        if is_key_pressed(KeyCode::Enter) {
            text_buffer.push('\n');
        }

        if is_key_pressed(KeyCode::Backspace) && !text_buffer.is_empty() {
            let sound_cursor = delete_cursor.clone();
            let sink = Sink::try_new(&stream_handle).unwrap();
            sink.append(Decoder::new(sound_cursor).unwrap());
            sink.detach();

            // Calculate the explosion position for backspace
            let text_width = measure_text(&text_buffer, None, font_size as u16, 1.0).width;
            let explosion_position = vec2(text_start_x + text_width, text_start_y);

            // Create backspace explosion particles with different characteristics
            for _ in 0..particles_per_keypress {
                let angle = rand::gen_range(0.0, 2.0 * std::f32::consts::PI);
                let speed = rand::gen_range(70.0, 120.0); // Different speed range for backspace
                particles.push(Particle {
                    position: explosion_position,
                    velocity: vec2(angle.cos() * speed, angle.sin() * speed),
                    color: Color::new(
                        rand::gen_range(0.9, 1.0), // Brighter color for backspace
                        rand::gen_range(0.0, 0.2), // Lower green component for backspace
                        rand::gen_range(0.0, 0.2), // Lower blue component for backspace
                        1.0,
                    ),
                    lifetime: rand::gen_range(0.3, 0.7), // Shorter lifetime for backspace particles
                });
            }
            text_buffer.pop();
        }
        // Check for key press and add character to text buffer
        if let Some(character) = any_key_pressed() {
            text_buffer.push(character);

            // Split lines and calculate the explosion position based on the last line
            let lines: Vec<&str> = text_buffer.split_inclusive('\n').collect();
            let last_line = lines.last().unwrap_or(&"");
            let text_width = measure_text(last_line.trim_end(), None, font_size as u16, 1.0).width;

            let explosion_position = vec2(
                text_start_x + text_width,
                text_start_y + (lines.len() - 1) as f32 * line_height + scroll_offset,
            );

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
        // draw_text(&text_buffer, text_start_x, text_start_y, font_size, WHITE);

        let lines: Vec<&str> = text_buffer.split_inclusive('\n').collect();
        for (i, line) in lines.iter().enumerate() {
            let y_position = text_start_y + i as f32 * line_height + scroll_offset;
            draw_text(line.trim_end(), text_start_x, y_position, font_size, WHITE);
            // trim_end to avoid extra space
        }

        if cursor_visible {
            let last_line = lines.last().unwrap_or(&"");
            let text_width = measure_text(last_line.trim_end(), None, font_size as u16, 1.0).width;
            let cursor_y = text_start_y + (lines.len() - 1) as f32 * line_height + scroll_offset;
            draw_line(
                text_start_x + text_width,
                cursor_y - font_size * 0.5,
                text_start_x + text_width,
                cursor_y,
                2.0,
                YELLOW,
            );
        }

        next_frame().await;
    }
}
