use std::f32::consts::PI;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;
use crossterm::terminal;

// Espaciado entre puntos al recorrer la superficie
// donde theta recorre el círculo de R1 y phi el círculo de r2
const THETA_SPACING: f32 = 0.07;
const PHI_SPACING: f32 = 0.02;

// Radios y distancia del objeto desde el observador
// r1 es el tubo del toroide, r2 es la distancia del eje al centro del tubo
// y k2 es la distancia del observador al toroide (profundidad)
const R1: f32 = 1.0;
const R2: f32 = 2.0;
const K2: f32 = 4.6;

// Tiempo entre cada frame en milisegundos
const SLEEP_TIME: u64 = 60;

// Renderizado de un frame
fn render_frame(a: f32, b: f32, screen_width: usize, screen_height: usize) {
    // Constante de proyección: escala para que quepa en pantalla 
    let k1: f32 = screen_width as f32 * 3.0 / (8.0 * (R1 + R2));
    let (sin_a, cos_a) = a.sin_cos();
    let (sin_b, cos_b) = b.sin_cos();

    // Buffer de salida y profundidad
    let mut output = vec![b' '; screen_width * screen_height];
    let mut z_buffer = vec![0.0; screen_width * screen_height];

    // Recorrido dla superficie del toroide con dos ángulos (theta y phi)
    let mut theta: f32 = 0.0;
    while theta < 2.0 * PI {
        let (sin_theta, cos_theta) = theta.sin_cos();
        let mut phi: f32 = 0.0;

        while phi < 2.0 * PI {
            let (sin_phi, cos_phi) = phi.sin_cos();

            // Coordenadas del tubo
            let x_circle: f32 = R2 + R1 * cos_theta;
            let y_circle: f32 = R1 * sin_theta;

            // Rotación y conversión de 3D a 2D
            let x: f32 = x_circle * ( cos_b * cos_phi + sin_a * sin_b *sin_phi) - y_circle * cos_a * sin_b;
            let y: f32 = x_circle * ( sin_b * cos_phi - sin_a * cos_b *sin_phi) + y_circle * cos_a * cos_b;
            let z: f32 = K2 + cos_a * x_circle * sin_phi + y_circle * sin_a;
            let z_inverse: f32 = 1.0 / z;

            // Coordenadas 3D a posiciones en pantalla
            let x_projection: isize = (screen_width as f32 / 2.0 + k1 * z_inverse * x) as isize;
            let y_projection: isize = (screen_height as f32 / 2.0 - k1 * z_inverse * y) as isize;

            // Cálculo cómo se reflja la luz en la superficie
            let luminance: f32 = cos_phi * cos_theta * sin_b - cos_a * cos_theta * sin_phi - sin_a * sin_theta + cos_b * (cos_a * sin_theta - cos_theta * sin_a * sin_phi);

            // Se dibuja si:
            // - El punto es visible / está dentro de la pantalla
            // - El punto está más cercano o por encima de otro
            if x_projection >= 0 && x_projection < screen_width as isize && y_projection >= 0 && y_projection < screen_height as isize {
                let index = y_projection as usize * screen_width + x_projection as usize;

                if luminance > 0.0 && z_inverse > z_buffer[index] {
                    z_buffer[index] = z_inverse;

                    // Tabla de caracteres según la luz
                    let chars = b".,-~:;=!*#$@";
                    let luminance_index = (luminance * 8.0) as usize;
                    output[index] = chars[luminance_index.min(chars.len() -1)];
                }
            }
            phi += PHI_SPACING;
        }
        theta += THETA_SPACING;
    }

    // Dibujado de pantalla
    print!("\x1b[H");
    let mut stdout = io::stdout().lock();

    for y in 0..screen_height {
        let start = y * screen_width;
        let end = start + screen_width;

        stdout.write_all(&output[start..end]).unwrap();
        stdout.write_all(b"\n").unwrap();
    }
}

fn main() {
    // Obtener dimensiones de la terminal
    let (screen_width, screen_height) = terminal::size().unwrap();
    let screen_width = screen_width as usize;
    let screen_height = screen_height as usize;

    // Ángulos inicales
    let mut a = 0.0;
    let mut b = 0.0;

    // Loop de renderizado de frames
    loop {
        render_frame(a, b, screen_width, screen_height);
        thread::sleep(Duration::from_millis(SLEEP_TIME));
        a += 0.04;
        b += 0.08;
    }
}
 
