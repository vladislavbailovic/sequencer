use super::{color, MAX_HEIGHT, MIN_WIDTH};
use std::fs::File;
use std::io::{BufWriter, Write};
pub struct Hits {
    beats_per_bar: usize,
    width: usize,
    height: usize,
}

impl Default for Hits {
    fn default() -> Self {
        Self {
            beats_per_bar: 4,
            width: MIN_WIDTH as usize,
            height: MAX_HEIGHT as usize,
        }
    }
}

impl Hits {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn beats(&mut self, beats: usize) -> &mut Self {
        self.beats_per_bar = beats;
        self
    }

    pub fn graph(&self, raw: &[(f64, f64)]) -> Vec<u8> {
        let max_size = 3 * self.width * self.height;
        let bheight: usize = self.height / 2;
        let bwidth: usize = 25;

        let mut buffer = vec![0; max_size as usize];
        self.draw_frame(&mut buffer);

        // Blocks
        let block_color = color(0x33AA33);
        let mut relx = 0;
        for block in raw {
            let posx = relx + (block.1 as usize * bwidth) as usize;
            let posy = (block.0 * bheight as f64) as usize;
            for y in self.height - posy..self.height {
                for x in relx..posx {
                    let offset = (y * self.width * 3) + (x * 3);
                    buffer[offset] = block_color.0;
                    buffer[offset + 1] = block_color.1;
                    buffer[offset + 2] = block_color.2;
                }
            }

            relx = posx;
        }

        buffer
    }

    #[allow(clippy::unused_io_amount)]
    pub fn draw(&self, fname: &str, raw: &[(f64, f64)]) -> std::io::Result<()> {
        let mut p = BufWriter::new(File::create(fname)?);
        p.write(format!("P6 {} {} 255\n", self.width, self.height).as_bytes())?;
        p.write(&self.graph(raw))?;

        Ok(())
    }

    fn draw_frame(&self, buffer: &mut Vec<u8>) {
        let bheight: usize = 10;
        let bwidth: usize = 25;

        // frame
        let bg_color_light = color(0x061739);
        let bg_color_dark = color(0x001133);
        let fg_color = color(0x102143);

        for y in 0..self.height {
            for x in 0..self.width {
                let offset = ((y * self.width * 3) + (x * 3)) as usize;
                if x % bwidth == 0 || y % bheight == 0 {
                    buffer[offset] = fg_color.0;
                    buffer[offset + 1] = fg_color.1;
                    buffer[offset + 2] = fg_color.2;
                } else {
                    let bar_idx = x / bwidth;
                    let bg_color = if bar_idx % 2 == 0 {
                        bg_color_light
                    } else {
                        bg_color_dark
                    };
                    buffer[offset] = bg_color.0;
                    buffer[offset + 1] = bg_color.1;
                    buffer[offset + 2] = bg_color.2;
                }
            }
        }
    }
}
