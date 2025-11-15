 
pub fn gradiente_radial(x: f32, y: f32, cx: f32, cy: f32, radio: f32, color1: [f32;3], color2: [f32;3]) -> [f32;3] {
    let dx = x - cx;
    let dy = y - cy;
    let dist = ((dx*dx + dy*dy).sqrt() / radio).min(1.0);
    [
        color1[0] * (1.0-dist) + color2[0] * dist,
        color1[1] * (1.0-dist) + color2[1] * dist,
        color1[2] * (1.0-dist) + color2[2] * dist,
    ]
}

 
pub fn bandas_planetarias(y: f32, cy: f32, radio: f32, colores: &[[f32;3]]) -> [f32;3] {
    let rel = ((y-cy)/radio + 1.0) * 0.5; // Normalizado 0..1
    let num_bandas = colores.len();
    let banda = ((rel * num_bandas as f32) as usize).min(num_bandas-1);
    colores[banda]
}

 
pub fn manchas(x: f32, y: f32, seed: f32, color: [f32;3], fondo: [f32;3]) -> [f32;3] {
    let v = ((x*0.03 + y*0.03 + seed).sin() * 0.5 + 0.5) * ((x*0.07 - y*0.05 + seed*2.0).cos() * 0.5 + 0.5);
    if v > 0.7 {
        color
    } else {
        fondo
    }
}

 
pub fn gradiente_vertical(y: f32, cy: f32, radio: f32, color1: [f32;3], color2: [f32;3]) -> [f32;3] {
    let rel = ((y-cy)/radio + 1.0) * 0.5;
    [
        color1[0] * (1.0-rel) + color2[0] * rel,
        color1[1] * (1.0-rel) + color2[1] * rel,
        color1[2] * (1.0-rel) + color2[2] * rel,
    ]
}

 
pub fn mezcla(a: [f32;3], b: [f32;3], t: f32) -> [f32;3] {
    [
        a[0]*(1.0-t) + b[0]*t,
        a[1]*(1.0-t) + b[1]*t,
        a[2]*(1.0-t) + b[2]*t,
    ]
}

 
pub fn textura_planeta(x: f32, y: f32, cx: f32, cy: f32, radio: f32, seed: f32) -> [f32;3] {
    // Degradado radial base
    let base = gradiente_radial(x, y, cx, cy, radio, [0.9,0.8,0.5], [0.2,0.2,0.5]);
    // Bandas
    let bandas = bandas_planetarias(y, cy, radio, &[ [0.9,0.8,0.5], [0.7,0.6,0.3], [0.5,0.4,0.2], [0.8,0.7,0.4] ]);
    // Manchas
    let detalle = manchas(x, y, seed, [1.0,0.3,0.2], base);
    // Mezcla todo
    mezcla(
        mezcla(base, bandas, 0.5),
        detalle,
        0.3
    )
}
// textura.rs
use crate::{Vec2, fbm, clamp, mix, smoothstep};

pub fn planet_texture(pos: Vec2, center: Vec2, radius: f32, planet_index: usize, is_gas: bool) -> (f32, f32, f32, f32) {
    let p = pos.sub(center);
    let dist = p.len();
    let t = dist / radius;
    if t > 1.0 { return (0.0, 0.0, 0.0, 0.0); }

    // Paletas mejoradas
    let (base_r, base_g, base_b) = match (is_gas, planet_index % 4) {
        (false, 0) => (0.55, 0.48, 0.38), // Tierra
        (false, 1) => (0.38, 0.42, 0.60), // Azul
        (false, 2) => (0.65, 0.62, 0.5),  // Gris
        (false, 3) => (0.7, 0.5, 0.3),    // Marrón
        (true, 0) => (0.95, 0.7, 0.25),   // Amarillo
        (true, 1) => (0.45, 0.7, 0.95),   // Azul claro
        (true, 2) => (0.7, 0.9, 0.6),     // Verde
        _ => (0.6, 0.6, 0.6),
    };

    let base_light = 1.0 - 0.5 * t;
    let angle = (p.y / radius) * std::f32::consts::PI;
    let band_coord = angle;
    let bands = band_coord.sin() * 0.5 + 0.5;
    let nx = p.x / radius * 2.0;
    let ny = p.y / radius * 2.0;
    let storm = fbm(nx * 3.0, ny * 3.0);
    let rim = smoothstep(0.85, 1.0, t);

    let (mut r, mut g, mut b) = (base_r, base_g, base_b);
    if is_gas {
        // --- CAPA 1: bandas principales ---
        let band_t = mix(0.5, 1.0, bands * 0.8 + 0.2 * storm);
        r *= band_t * base_light;
        g *= mix(band_t, 1.0, 0.7) * base_light;
        b *= mix(1.0, band_t, 0.7) * base_light;

        // --- CAPA 2: bandas secundarias y degradado extra ---
        let bands2 = ((angle * 2.7).sin() * 0.5 + 0.5) * (0.7 + 0.3 * fbm(nx * 1.2, ny * 1.2));
        let color2 = (
            mix(r, 0.9, bands2 * 0.4),
            mix(g, 0.85, bands2 * 0.4),
            mix(b, 1.0, bands2 * 0.4)
        );
        r = mix(r, color2.0, 0.5);
        g = mix(g, color2.1, 0.5);
        b = mix(b, color2.2, 0.5);

        // --- CAPA 3: swirl y detalles suaves ---
        let swirl = fbm(nx * 1.5 + storm * 2.0, ny * 1.5 - storm * 2.0);
        r = mix(r, r * 1.2 + 0.10, swirl * 0.3);
        g = mix(g, g * 1.1 + 0.08, swirl * 0.2);
        b = mix(b, b * 1.05 + 0.05, swirl * 0.15);

        // --- CAPA 4: degradado polar ---
        let polar = (p.y / radius).abs().powf(1.5);
        r = mix(r, 1.0, polar * 0.08);
        g = mix(g, 1.0, polar * 0.06);
        b = mix(b, 1.0, polar * 0.04);

        // --- CAPA 5: bandas diagonales ---
        let diag = ((nx + ny) * 2.5).sin() * 0.5 + 0.5;
        r = mix(r, 0.7, diag * 0.28);
        g = mix(g, 0.85, diag * 0.22);
        b = mix(b, 1.0, diag * 0.18);

        // --- CAPA 6: manchas suaves ---
        let spots = fbm(nx * 3.5 - 2.0, ny * 3.5 + 2.0);
        let spot_mask = smoothstep(0.65, 0.8, spots);
        r = mix(r, r * 0.5 + 0.35, spot_mask * 0.7);
        g = mix(g, g * 0.5 + 0.35, spot_mask * 0.7);
        b = mix(b, b * 0.5 + 0.35, spot_mask * 0.7);

        // --- CAPA 7: gradiente vertical extra ---
        let gradv = (p.y / radius).abs().powf(2.0);
        r = mix(r, 0.85, gradv * 0.22);
        g = mix(g, 0.82, gradv * 0.18);
        b = mix(b, 1.0, gradv * 0.14);

        // --- CAPA 8: "viento" horizontal ---
        let wind = (nx * 4.0 + fbm(ny * 2.0, nx * 2.0)).sin() * 0.5 + 0.5;
        r = mix(r, 1.0, wind * 0.15);
        g = mix(g, 1.0, wind * 0.12);
        b = mix(b, 1.0, wind * 0.10);
    } else {
        // --- CAPA 1: estratos principales ---
        let strata = smoothstep(-0.7, 0.7, (p.x / radius) * 3.2 + fbm(nx * 2.2, ny * 2.2));
        let strata_t = mix(0.85, 1.25, strata);
        r *= strata_t * base_light;
        g *= mix(0.92, 1.15, strata) * base_light;
        b *= mix(0.82, 1.08, strata * 0.8) * base_light;

        // --- CAPA 2: bandas secundarias y color extra ---
        let bands2 = ((angle * 2.0).sin() * 0.5 + 0.5) * (0.7 + 0.3 * fbm(nx * 1.5, ny * 1.5));
        let color2 = (
            mix(r, 0.7, bands2 * 0.5),
            mix(g, 0.6, bands2 * 0.5),
            mix(b, 0.8, bands2 * 0.5)
        );
        r = mix(r, color2.0, 0.4);
        g = mix(g, color2.1, 0.4);
        b = mix(b, color2.2, 0.4);

        // --- CAPA 3: cráteres y detalles ---
        let crater = fbm(nx * 6.5, ny * 6.5);
        let crater_mask = smoothstep(0.58, 0.64, crater);
        r = mix(r, r * 0.45, crater_mask * 0.95);
        g = mix(g, g * 0.45, crater_mask * 0.95);
        b = mix(b, b * 0.45, crater_mask * 0.95);

        // --- CAPA 4: degradado radial extra ---
        let grad = t.powf(1.7);
        r = mix(r, 0.95, grad * 0.12);
        g = mix(g, 0.92, grad * 0.10);
        b = mix(b, 1.0, grad * 0.08);

        // --- CAPA 5: bandas diagonales ---
        let diag = ((nx - ny) * 2.2).sin() * 0.5 + 0.5;
        r = mix(r, 0.6, diag * 0.32);
        g = mix(g, 0.5, diag * 0.22);
        b = mix(b, 0.8, diag * 0.18);

        // --- CAPA 6: manchas suaves ---
        let spots = fbm(nx * 4.0 + 1.0, ny * 4.0 - 1.0);
        let spot_mask = smoothstep(0.62, 0.8, spots);
        r = mix(r, r * 0.4 + 0.45, spot_mask * 0.8);
        g = mix(g, g * 0.4 + 0.45, spot_mask * 0.8);
        b = mix(b, b * 0.4 + 0.45, spot_mask * 0.8);

        // --- CAPA 7: gradiente vertical extra ---
        let gradv = (p.y / radius).abs().powf(2.0);
        r = mix(r, 0.75, gradv * 0.22);
        g = mix(g, 0.72, gradv * 0.18);
        b = mix(b, 0.95, gradv * 0.14);

        // --- CAPA 8: "viento" horizontal ---
        let wind = (ny * 4.0 + fbm(nx * 2.0, ny * 2.0)).sin() * 0.5 + 0.5;
        r = mix(r, 1.0, wind * 0.13);
        g = mix(g, 1.0, wind * 0.10);
        b = mix(b, 1.0, wind * 0.08);
    }

    // Borde más suave y con halo
    let rim_strength = rim * 0.8;
    r = mix(r, 1.0, rim_strength * 0.38);
    g = mix(g, 1.0, rim_strength * 0.28);
    b = mix(b, 1.0, rim_strength * 0.18);
    let edge_dark = mix(1.0, 0.72, t * 0.85);
    r *= edge_dark;
    g *= edge_dark;
    b *= edge_dark;

    // Contraste y saturación extra
    let sat = 1.08;
    r = clamp(r * sat, 0.0, 1.0);
    g = clamp(g * sat, 0.0, 1.0);
    b = clamp(b * sat, 0.0, 1.0);

    (r, g, b, 1.0)
}
