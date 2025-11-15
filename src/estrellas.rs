// Módulo para generación y animación de estrellas
use crate::{Vec2, WIDTH, HEIGHT, hash, rgb_to_u32};

pub struct Estrella {
    pub pos: Vec2,
    pub seed: f32,
}

pub fn generar_estrellas(num: usize) -> Vec<Estrella> {
    let mut estrellas = Vec::with_capacity(num);
    for i in 0..num {
        let fx = hash(i as f32 * 12.345) * (WIDTH as f32 - 1.0);
        let fy = hash(i as f32 * 98.765) * (HEIGHT as f32 - 1.0);
        estrellas.push(Estrella {
            pos: Vec2::new(fx, fy),
            seed: i as f32,
        });
    }
    estrellas
}

pub fn animar_estrellas(estrellas: &Vec<Estrella>, tiempo: f32, buffer: &mut [u32]) {
    for (i, estrella) in estrellas.iter().enumerate() {
        let mov_x = (hash(estrella.seed * 1.23) * 2.0 - 1.0) * (5.0 * (tiempo * 0.2 + estrella.seed * 0.01).sin());
        let mov_y = (hash(estrella.seed * 9.87) * 2.0 - 1.0) * (5.0 * (tiempo * 0.2 + estrella.seed * 0.01).cos());
        let x = estrella.pos.x + mov_x;
        let y = estrella.pos.y + mov_y;
        let brillo = 0.7 + 0.3 * (hash(estrella.seed + tiempo * 2.0) * 2.0 - 1.0).abs();
        let color = rgb_to_u32(brillo, brillo, brillo);
        if x >= 0.0 && x < WIDTH as f32 && y >= 0.0 && y < HEIGHT as f32 {
            buffer[y as usize * WIDTH + x as usize] = color;
        }
    }
}
