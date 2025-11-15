use crate::{Vec2, rgb_to_u32, clamp, fbm, smoothstep};
use crate::textura::planet_texture;
pub use crate::{WIDTH, HEIGHT};

pub struct Framebuffer {
    pub buffer: Vec<u32>,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize, color: u32) -> Self {
        Self { buffer: vec![color; width * height] }
    }

    pub fn draw_body(&mut self, center: Vec2, radius: f32, planet_index: usize, is_gas: bool, has_rings: bool) {
        let minx = clamp(center.x - radius - 2.0,0.0,(WIDTH-1) as f32) as usize;
        let maxx = clamp(center.x + radius + 2.0,0.0,(WIDTH-1) as f32) as usize;
        let miny = clamp(center.y - radius - 2.0,0.0,(HEIGHT-1) as f32) as usize;
        let maxy = clamp(center.y + radius + 2.0,0.0,(HEIGHT-1) as f32) as usize;

        for y in miny..=maxy {
            for x in minx..=maxx {
                let idx = y*WIDTH + x;
                let p = Vec2::new(x as f32 + 0.5, y as f32 + 0.5);
                let (r,g,b,a) = planet_texture(p, center, radius, planet_index, is_gas);
                if a <= 0.0 { continue; }
                let dst = self.buffer[idx];
                let dst_r = ((dst>>16)&0xFF) as f32 /255.0;
                let dst_g = ((dst>>8)&0xFF) as f32 /255.0;
                let dst_b = (dst&0xFF) as f32 /255.0;
                let out_r = r*a + dst_r*(1.0-a); let out_g = g*a + dst_g*(1.0-a); let out_b = b*a + dst_b*(1.0-a);
                self.buffer[idx] = rgb_to_u32(out_r, out_g, out_b);
            }
        }

        if has_rings {
            let ring_outer = radius*2.0; let ring_inner = radius*1.15;
            for y in (center.y-ring_outer).max(0.0) as usize..=(center.y+ring_outer).min(HEIGHT as f32-1.0) as usize {
                for x in (center.x-ring_outer).max(0.0) as usize..=(center.x+ring_outer).min(WIDTH as f32-1.0) as usize {
                    let dx = x as f32 - center.x; let dy = (y as f32 - center.y)*0.6;
                    let rdist = (dx*dx + dy*dy).sqrt();
                    if rdist < ring_inner || rdist > ring_outer { continue; }
                    let band_count = 12.0; let band_pos = (rdist - ring_inner)/(ring_outer-ring_inner)*band_count; 
                    let band_mod = band_pos.fract(); let band_mask = smoothstep(0.02,0.0,(band_mod-0.5).abs());
                    let dust = fbm(dx*0.2, dy*0.2); let intensity = 0.3*band_mask + 0.1*dust;
                    let dst = self.buffer[y*WIDTH+x]; let dst_r = ((dst>>16)&0xFF) as f32/255.0;
                    let dst_g = ((dst>>8)&0xFF) as f32/255.0; let dst_b = (dst&0xFF) as f32/255.0;
                    let out_r = clamp(dst_r + 0.8*intensity,0.0,1.0);
                    let out_g = clamp(dst_g + 0.75*intensity,0.0,1.0);
                    let out_b = clamp(dst_b + 0.6*intensity,0.0,1.0);
                    self.buffer[y*WIDTH+x] = rgb_to_u32(out_r,out_g,out_b);
                }
            }
        }
    }

    pub fn draw_orbit(&mut self, center: Vec2, orbit_radius: f32) {
        for angle in (0..360).step_by(1) {
            let rad = (angle as f32).to_radians();
            let x = center.x + orbit_radius * rad.cos();
            let y = center.y + orbit_radius * rad.sin()*0.4;
            if x >=0.0 && y>=0.0 && x<WIDTH as f32 && y<HEIGHT as f32 {
                self.buffer[y as usize*WIDTH + x as usize] = rgb_to_u32(0.3,0.3,0.4);
            }
        }
    }

    pub fn draw_star(&mut self, center: Vec2, radius: f32) {
        for y in (center.y-radius*2.0).max(0.0) as usize..=(center.y+radius*2.0).min(HEIGHT as f32-1.0) as usize {
            for x in (center.x-radius*2.0).max(0.0) as usize..=(center.x+radius*2.0).min(WIDTH as f32-1.0) as usize {
                let dx = x as f32 + 0.5 - center.x; let dy = y as f32 + 0.5 - center.y;
                let d = (dx*dx + dy*dy).sqrt();
                if d > radius*2.0 { continue; }
                let t = d/radius;
                let core = smoothstep(0.0,1.0,1.0-t);
                let glow = smoothstep(1.0,2.0,1.0-t);
                let r = clamp(1.0*core + 0.6*glow,0.0,1.0);
                let g = clamp(0.8*core + 0.35*glow,0.0,1.0);
                let b = clamp(0.2*core + 0.1*glow,0.0,1.0);
                let dst = self.buffer[y*WIDTH+x]; let dst_r = ((dst>>16)&0xFF) as f32/255.0;
                let dst_g = ((dst>>8)&0xFF) as f32/255.0; let dst_b = (dst&0xFF) as f32/255.0;
                self.buffer[y*WIDTH+x] = rgb_to_u32(dst_r+r, dst_g+g, dst_b+b);
            }
        }
    }

    pub fn as_slice(&self) -> &[u32] {
        &self.buffer
    }
}
