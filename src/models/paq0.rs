use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};

const CONTEXT_LENGTH: usize = 4;
const ORDER: usize = 0x100;

struct PpmModel {
    counts: HashMap<Vec<u8>, [u32; ORDER]>,
    escapes: [u32; ORDER],
}

impl PpmModel {
    fn new() -> PpmModel {
        PpmModel {
            counts: HashMap::new(),
            escapes: [1; ORDER],
        }
    }

    fn update(&mut self, context: &[u8], symbol: u8) {
        let count = self.counts.entry(context.to_vec()).or_insert([1; ORDER]);
        count[symbol as usize] += 1;
    }

    fn encode(&mut self, context: &[u8], symbol: u8) -> Vec<u8> {
        let count = self.counts.get(context).unwrap_or(&self.escapes);
        let total = count.iter().sum::<u32>() + ORDER as u32;
        let mut low = 0;
        let mut high = 0x10000;

        for (i, &c) in count.iter().enumerate() {
            if i == symbol as usize {
                high = low + ((c + 1) * (0x10000 - low) / total);
            } else {
                low += c * (0x10000 - low) / total;
                high = low + (c * (0x10000 - low) / total);
            }
        }

        self.update(context, symbol);

        let mut compressed = vec![];
        while high < 0x8000 {
            compressed.push((low >> 8) as u8);
            low = (low << 8) & 0xffff;
            high = (high << 8) & 0xffff;
        }
        compressed.push((low >> 8) as u8);
        compressed
    }
}

fn compress_file(input_file: &str, output_file: &str) -> Result<(), std::io::Error> {
    let mut input = File::open(input_file)?;
    let mut output = File::create(output_file)?;

    let mut ppm_model = PpmModel::new();
    let mut context = [0; CONTEXT_LENGTH];
    let mut buf = [0; 1];

    while input.read_exact(&mut buf).is_ok() {
        let compressed = ppm_model.encode(&context, buf[0]);
        output.write_all(&compressed)?;
        context.rotate_left(1);
        context[0] = buf[0];
    }

    Ok(())
}

fn main() {
    let input_file = "input.txt";
    let output_file = "output.paq0";

    if let Err(e) = compress_file(input_file, output_file) {
        eprintln!("Error: {}", e);
    } else {
        println!("Compression successful!");
    }
}
