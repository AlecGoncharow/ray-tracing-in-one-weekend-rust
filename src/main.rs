use std::fs::File;
use std::io::prelude::*;

mod vec;
use vec::Vec3;

struct Output {
    rows: u32,
    cols: u32,
    colors: Vec<Color>,
}

impl Output {
    fn write(&self) -> std::io::Result<()> {
        let mut file = File::create("out.ppm")?;

        let header = format!("P3\n{} {}\n255\n", self.cols, self.rows);

        file.write_all(header.as_bytes())?;

        self.colors.iter().for_each(|color| {
            let row = format!("{} {} {}\n", color.r, color.g, color.b);
            file.write_all(row.as_bytes()).expect("color machine broke");
        });

        Ok(())
    }
}

struct Color {
    r: u8,
    g: u8,
    b: u8,
}

fn main() -> std::io::Result<()> {
    let mut out = Output {
        rows: 250,
        cols: 500,
        colors: vec![],
    };

    for i in 0..out.rows {
        for j in 0..out.cols {
            let normalized_colors = Vec3::new(
                j as f32 / out.cols as f32,
                (out.rows - i) as f32 / out.rows as f32,
                0.2f32,
            );

            let color = Color {
                r: (normalized_colors.x * 255.99) as u8,
                g: (normalized_colors.y * 255.99) as u8,
                b: (normalized_colors.z * 255.99) as u8,
            };

            out.colors.push(color);
        }
    }

    out.write()
}
