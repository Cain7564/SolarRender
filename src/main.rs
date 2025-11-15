// Estructura Vec3 y Camera para movimiento libre
#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
    pub fn add(self, o: Vec3) -> Vec3 {
        Vec3::new(self.x + o.x, self.y + o.y, self.z + o.z)
    }
    pub fn sub(self, o: Vec3) -> Vec3 {
        Vec3::new(self.x - o.x, self.y - o.y, self.z - o.z)
    }
    pub fn scale(self, s: f32) -> Vec3 {
        Vec3::new(self.x * s, self.y * s, self.z * s)
    }
    pub fn len(self) -> f32 {
        (self.x*self.x + self.y*self.y + self.z*self.z).sqrt()
    }
    pub fn normalize(self) -> Vec3 {
        let l = self.len();
        if l > 0.0 { self.scale(1.0/l) } else { self }
    }
    pub fn cross(self, o: Vec3) -> Vec3 {
        Vec3::new(
            self.y * o.z - self.z * o.y,
            self.z * o.x - self.x * o.z,
            self.x * o.y - self.y * o.x,
        )
    }
}

pub struct Camera {
    pub position: Vec3,
    pub yaw: f32,
    pub pitch: f32,
    pub speed: f32,
}
impl Camera {
    pub fn new(position: Vec3) -> Self {
        Self { position, yaw: 0.0, pitch: 0.0, speed: 40.0 }
    }
    pub fn get_forward(&self) -> Vec3 {
        let yaw_rad = self.yaw.to_radians();
        let pitch_rad = self.pitch.to_radians();
        Vec3::new(
            yaw_rad.cos() * pitch_rad.cos(),
            pitch_rad.sin(),
            yaw_rad.sin() * pitch_rad.cos(),
        ).normalize()
    }
    pub fn get_right(&self) -> Vec3 {
        let forward = self.get_forward();
        forward.cross(Vec3::new(0.0, 1.0, 0.0)).normalize()
    }
    pub fn get_up(&self) -> Vec3 {
        self.get_right().cross(self.get_forward()).normalize()
    }
    pub fn move_forward(&mut self, dt: f32) {
        self.position = self.position.add(self.get_forward().scale(self.speed * dt));
    }
    pub fn move_backward(&mut self, dt: f32) {
        self.position = self.position.sub(self.get_forward().scale(self.speed * dt));
    }
    pub fn move_left(&mut self, dt: f32) {
        self.position = self.position.sub(self.get_right().scale(self.speed * dt));
    }
    pub fn move_right(&mut self, dt: f32) {
        self.position = self.position.add(self.get_right().scale(self.speed * dt));
    }
    pub fn rotate(&mut self, dx: f32, dy: f32) {
        self.yaw += dx * 0.2;
        self.pitch += dy * 0.2;
        self.pitch = self.pitch.clamp(-89.0, 89.0);
    }
}

// Proyección de perspectiva
fn project(camera: &Camera, world: Vec3) -> Option<Vec2> {
    let forward = camera.get_forward();
    let right = camera.get_right();
    let up = camera.get_up();
    let rel = world.sub(camera.position);
    let z = rel.x * forward.x + rel.y * forward.y + rel.z * forward.z;
    if z <= 0.1 { return None; }
    let x = rel.x * right.x + rel.y * right.y + rel.z * right.z;
    let y = rel.x * up.x + rel.y * up.y + rel.z * up.z;
    let fov = 400.0;
    let px = WIDTH as f32 * 0.5 + x * fov / z;
    let py = HEIGHT as f32 * 0.5 - y * fov / z;
    Some(Vec2::new(px, py))
}

mod framebuffer;
mod textura;
mod estrellas;
use minifb::{Key, Window, WindowOptions};
pub const WIDTH: usize = 1920;
pub const HEIGHT: usize = 1080;

#[derive(Clone, Copy)]
pub struct Vec2 { pub x: f32, pub y: f32 }
impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self { Self { x, y } }
    pub fn sub(self, o: Vec2) -> Vec2 { Vec2::new(self.x - o.x, self.y - o.y) }
    pub fn len(self) -> f32 { (self.x*self.x + self.y*self.y).sqrt() }
}

pub fn clamp(v: f32, a: f32, b: f32) -> f32 { v.min(b).max(a) }
pub fn rgb_to_u32(r: f32, g: f32, b: f32) -> u32 {
    let r = (clamp(r,0.0,1.0)*255.0) as u32;
    let g = (clamp(g,0.0,1.0)*255.0) as u32;
    let b = (clamp(b,0.0,1.0)*255.0) as u32;
    (r << 16) | (g << 8) | b
}
pub fn mix(a: f32, b: f32, t: f32) -> f32 { a*(1.0-t) + b*t }
pub fn smoothstep(e0: f32, e1: f32, x: f32) -> f32 {
    let t = clamp((x - e0)/(e1 - e0), 0.0, 1.0);
    t*t*(3.0 - 2.0*t)
}

pub fn hash(seed: f32) -> f32 {
    let x = (seed * 12345.6789).sin() * 43758.5453;
    x - x.floor()
}
pub fn noise2(x: f32, y: f32) -> f32 { let s = x*12.9898 + y*78.233; hash(s) }
pub fn fbm(x: f32, y: f32) -> f32 {
    let mut amp = 0.5; let mut freq = 1.0; let mut sum = 0.0;
    for _ in 0..5 { sum += amp * noise2(x*freq, y*freq); amp *= 0.5; freq *= 2.0; }
    sum
}

// planet_shader_layers ahora está en textura.rs como planet_texture

fn main() {
    let mut window = Window::new("Sistema Solar Procedural", WIDTH, HEIGHT, WindowOptions::default()).unwrap();
    let bg_color = rgb_to_u32(0.02,0.02,0.05);
    let num_estrellas = 3000;
    let estrellas = estrellas::generar_estrellas(num_estrellas);
        let mut camera = Camera::new(Vec3::new(
            WIDTH as f32 * 0.5,       // centrado en X
            HEIGHT as f32 * 0.5 - 1000.0, // arriba del sistema
            1400.0                    // suficientemente cerca
        ));

        let mut last_mouse = window
            .get_mouse_pos(minifb::MouseMode::Pass)
            .unwrap_or((WIDTH as f32 * 0.5, HEIGHT as f32 * 0.5));

        let sun_pos = Vec3::new(WIDTH as f32 * 0.5, HEIGHT as f32 * 0.5, 0.0);
        let orbit_radii = [160.0,230.0,310.0,400.0,490.0];

        let mut tiempo = 0.0;
        let mut last_time = std::time::Instant::now();
        while window.is_open() && !window.is_key_down(Key::Escape) {
            let now = std::time::Instant::now();
            let dt = (now - last_time).as_secs_f32();
            last_time = now;
            tiempo += dt;

            let rocky_angles = [tiempo*0.5, tiempo*0.3, tiempo*0.2];
            let rocky_positions_anim: Vec<Vec3> = orbit_radii[..3]
                .iter()
                .zip(rocky_angles.iter())
                .map(|(r, ang)| Vec3::new(
                    sun_pos.x + r * ang.cos(),
                    sun_pos.y + r * ang.sin(),
                    sun_pos.z
                ))
                .collect();
            let gas_angles = [tiempo*0.15, tiempo*0.1];
            let gas_positions_anim: Vec<Vec3> = orbit_radii[3..]
                .iter()
                .zip(gas_angles.iter())
                .map(|(r, ang)| Vec3::new(
                    sun_pos.x + r * ang.cos(),
                    sun_pos.y + r * ang.sin(),
                    sun_pos.z
                ))
                .collect();

            let mut next_pos = camera.position;
            if window.is_key_down(Key::W) {
                next_pos = next_pos.add(camera.get_forward().scale(camera.speed * dt));
            }
            if window.is_key_down(Key::S) {
                next_pos = next_pos.sub(camera.get_forward().scale(camera.speed * dt));
            }
            if window.is_key_down(Key::A) {
                next_pos = next_pos.sub(camera.get_right().scale(camera.speed * dt));
            }
            if window.is_key_down(Key::D) {
                next_pos = next_pos.add(camera.get_right().scale(camera.speed * dt));
            }
            if window.is_key_down(Key::Space) {
                next_pos.y += camera.speed * dt;
            }
            if window.is_key_down(Key::LeftShift) {
                next_pos.y -= camera.speed * dt;
            }

            let planet_positions: Vec<Vec3> = rocky_positions_anim.iter().cloned()
                .chain(gas_positions_anim.iter().cloned())
                .chain(std::iter::once(sun_pos))
                .collect();
            let planet_radii: Vec<f32> = vec![20.0, 20.0, 20.0, 40.0, 55.0, 90.0];
            let mut collided = false;
            for (pos, rad) in planet_positions.iter().zip(planet_radii.iter()) {
                if next_pos.sub(*pos).len() < *rad + 10.0 {
                    collided = true;
                    break;
                }
            }
            if !collided {
                camera.position = next_pos;
            }

            if let Some((mx, my)) = window.get_mouse_pos(minifb::MouseMode::Pass) {
                let dx = mx - last_mouse.0;
                let dy = my - last_mouse.1;
                camera.rotate(dx, dy);
                last_mouse = (mx, my);
            }

            let mut framebuffer = framebuffer::Framebuffer::new(WIDTH, HEIGHT, bg_color);

            estrellas::animar_estrellas(&estrellas, tiempo, &mut framebuffer.buffer);

            for r in orbit_radii.iter() {
                let mut prev: Option<Vec2> = None;
                for a in 0..=360 {
                    let ang = (a as f32).to_radians();
                    let pos3d = Vec3::new(
                        sun_pos.x + r * ang.cos(),
                        sun_pos.y + r * ang.sin(),
                        sun_pos.z
                    );
                    if let Some(p2d) = project(&camera, pos3d) {
                        if let Some(prev2d) = prev {
                            framebuffer.draw_body(p2d, 1.0, 0, false, false);
                        }
                        prev = Some(p2d);
                    }
                }
            }

            let luna_radio = 40.0;
            let luna_ang = tiempo * 1.2;
            let luna_pos = Vec3::new(
                rocky_positions_anim[0].x + luna_radio * luna_ang.cos(),
                rocky_positions_anim[0].y + luna_radio * luna_ang.sin(),
                rocky_positions_anim[0].z
            );

            if let Some(sun_2d) = project(&camera, sun_pos) {
                let dist = sun_pos.sub(camera.position).len().max(1.0);
                let scale = 800.0 / dist;
                let radio = 90.0 * scale;
                framebuffer.draw_star(sun_2d, radio);
            }

            for (i, p) in rocky_positions_anim.iter().enumerate() {
                // Ocultar si está detrás del sol
                let to_planet = p.sub(camera.position);
                let to_sun = sun_pos.sub(camera.position);
                let dot = to_planet.x * to_sun.x + to_planet.y * to_sun.y + to_planet.z * to_sun.z;
                if dot > to_sun.len() * to_planet.len() { // Si el ángulo es menor a 90° (está detrás)
                    continue;
                }
                if let Some(pos2d) = project(&camera, *p) {
                    let dist = p.sub(camera.position).len().max(1.0);
                    let scale = 800.0 / dist;
                    let radio = 20.0 * scale;
                    framebuffer.draw_body(pos2d, radio, i, false, false);
                }
            }

            if let Some(moon_2d) = project(&camera, luna_pos) {
                let dist = luna_pos.sub(camera.position).len().max(1.0);
                let scale = 800.0 / dist;
                let radio = 8.0 * scale;
                framebuffer.draw_body(moon_2d, radio, 0, false, false);
            }

            for (i, p) in gas_positions_anim.iter().enumerate() {
                let to_planet = p.sub(camera.position);
                let to_sun = sun_pos.sub(camera.position);
                let dot = to_planet.x * to_sun.x + to_planet.y * to_sun.y + to_planet.z * to_sun.z;
                if dot > to_sun.len() * to_planet.len() {
                    continue;
                }
                if let Some(pos2d) = project(&camera, *p) {
                    let dist = p.sub(camera.position).len().max(1.0);
                    let scale = 800.0 / dist;
                    let radio = if i==0 {40.0} else {55.0} * scale;
                    framebuffer.draw_body(pos2d, radio, i, true, i==0);
                }
            }

            window.update_with_buffer(framebuffer.as_slice(), WIDTH, HEIGHT).unwrap();
            let frame_time = (std::time::Instant::now() - now).as_secs_f32();
            let min_frame = 1.0 / 60.0;
            if frame_time < min_frame {
                std::thread::sleep(std::time::Duration::from_secs_f32(min_frame - frame_time));
            }
            }
        }
