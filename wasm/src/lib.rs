mod data_reader;

use data_reader::data_reader::DataReader;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

fn create_u16(num1: u8, num2: u8) -> u16 {
    let mut new_u16_int: u16 = 0x0000;

    new_u16_int = new_u16_int + num1 as u16;
    new_u16_int = new_u16_int << 8;
    new_u16_int = new_u16_int + num2 as u16;

    new_u16_int
}

#[wasm_bindgen]
pub fn decode(data: Vec<u8>) {
    let mut data_clone = data.clone();
    log("This is the size");
    log(&data.len().to_string());

    if data.len() < 2 {
        log("Where are my bytes bro!?");
        return;
    }

    let first_byte = data_clone.remove(0);

    let second_byte = data_clone.remove(0);

    if create_u16(first_byte, second_byte) != 0xFFD8 {
        log("Bad header homie!");
        log(&format!("{:02X} {:02X}", first_byte, second_byte));
        return;
    }

    let mut raw_data = Vec::<u8>::new();

    while data_clone.len() > 0 {
        let first_byte = data_clone.remove(0);

        if first_byte == 0xFF {
            let second_byte = data_clone.remove(0);
            match second_byte {
                // Application Header
                0xE0 | 0xE1 | 0xE2 | 0xE3 | 0xE4 | 0xE5 | 0xE6 | 0xE7 | 0xE8 | 0xE9 | 0xEA
                | 0xEB | 0xEC | 0xED | 0xEE | 0xEF => {
                    log("Application Header");
                    let b1 = data_clone.remove(0);
                    let b2 = data_clone.remove(0);
                    let segment_length = create_u16(b1, b2);
                    log(&format!("Length {}", segment_length));

                    log(&format!(
                        "Identifier {:02X}{:02X}{:02X}{:02X}\\{:02X}",
                        data_clone[0], data_clone[1], data_clone[2], data_clone[3], data_clone[4]
                    ));
                    log(&format!("Version {}.{}", data_clone[5], data_clone[6]));

                    log(&format!("Units {}", data_clone[7]));

                    log(&format!(
                        "X Units {}",
                        create_u16(data_clone[8], data_clone[9])
                    ));

                    log(&format!(
                        "Y Units {}",
                        create_u16(data_clone[10], data_clone[11])
                    ));

                    log(&format!("X ThumbNail {}", data_clone[12]));

                    log(&format!("Y ThumbNail {}", data_clone[13]));

                    for _i in 0..segment_length - 2 {
                        data_clone.remove(0);
                    }

                    log("***********");
                }
                // Quantization Table
                0xDB => {
                    log("QT");
                    let b1 = data_clone.remove(0);
                    let b2 = data_clone.remove(0);
                    let segment_length = create_u16(b1, b2);
                    log(&format!("Length {}", segment_length));
                    log(&format!("Luminance/Chrominance {}", data_clone.remove(0)));

                    let mut q_table: [[u8; 8]; 8] = [[0; 8]; 8];
                    for i in 0..8 {
                        for j in 0..8 {
                            q_table[i][j] = data_clone.remove(0);
                        }
                    }
                    log("***********");
                }
                // Start of Frame
                0xC0 => {
                    log("SOF");
                    let b1 = data_clone.remove(0);
                    let b2 = data_clone.remove(0);
                    let segment_length = create_u16(b1, b2);
                    log(&format!("Length {}", segment_length));

                    log(&format!("Precision {}", data_clone.remove(0)));
                    let height = create_u16(data_clone.remove(0), data_clone.remove(0)) as u64;
                    log(&format!("Line No {}", height));

                    let width = create_u16(data_clone.remove(0), data_clone.remove(0)) as u64;
                    log(&format!("Samples Per Line {}", width));

                    let number_of_components = data_clone.remove(0) as usize;

                    for _i in 0..number_of_components {
                        let b1 = data_clone.remove(0);
                        let b2 = data_clone.remove(0);
                        let b3 = data_clone.remove(0);

                        log(&format!(
                            "Component {} {}x{} {}",
                            b1,
                            (b2 & 0xF0) >> 4,
                            b2 & 0x0F,
                            b3
                        ));
                    }

                    // numberOfComponents := int(segmentDataBuffer[5])
                    log("***********");
                }
                // Start of Scan
                0xDA => {
                    log("SOC");
                    let segment_length = create_u16(data_clone.remove(0), data_clone.remove(0));
                    log(&format!("Length {}", segment_length));

                    let number_of_components = data_clone.remove(0);

                    for _i in 0..number_of_components {
                        let b1 = data_clone.remove(0);
                        let b2 = data_clone.remove(0);
                        log(&format!(
                            "Component {} DC {} AC {}",
                            b1,
                            (b2 & 0xF0) >> 4,
                            b2 & 0x0F
                        ));
                    }
                    log("***********");
                }
                // Huffman Table
                0xC4 => {
                    log("HUF");
                    let segment_length = create_u16(data_clone.remove(0), data_clone.remove(0));
                    log(&format!("Length {}", segment_length));
                    let b1 = data_clone.remove(0);

                    let class = (b1 & 0xF0) >> 4;

                    if class == 0 {
                        log("Class DC");
                    } else {
                        log("Class AC");
                    }

                    let table_id = b1 & 0x0F;
                    log(&format!("Table ID {}", table_id));

                    log(&format!(
                        "Number of Symbols {}",
                        segment_length - 1 - 2 - 16
                    ));

                    let mut frequency_counts = [0; 16];

                    let mut symbols = [
                        Vec::<u8>::new(),
                        Vec::new(),
                        Vec::new(),
                        Vec::new(),
                        Vec::new(),
                        Vec::new(),
                        Vec::new(),
                        Vec::new(),
                        Vec::new(),
                        Vec::new(),
                        Vec::new(),
                        Vec::new(),
                        Vec::new(),
                        Vec::new(),
                        Vec::new(),
                        Vec::new(),
                    ];

                    let mut codes = [
                        Vec::<u8>::new(),
                        Vec::new(),
                        Vec::new(),
                        Vec::new(),
                        Vec::new(),
                        Vec::new(),
                        Vec::new(),
                        Vec::new(),
                        Vec::new(),
                        Vec::new(),
                        Vec::new(),
                        Vec::new(),
                        Vec::new(),
                        Vec::new(),
                        Vec::new(),
                        Vec::new(),
                    ];
                    for i in 0..16 {
                        frequency_counts[i] = data_clone.remove(0);
                    }

                    let mut code = 0x0000;

                    // Generate Codes
                    for i in 0..16 {
                        for _j in 0..frequency_counts[i] {
                            symbols[i].push(data_clone.remove(0));
                            codes[i].push(code);
                            code += 1;
                        }
                        code = code << 1
                    }

                    log("***********");
                }
                // End of Image
                0xD9 => {
                    log("EOI");
                    log("***********");
                }
                0x00 => raw_data.push(first_byte),
                _ => raw_data.push(first_byte),
            }
        } else {
            raw_data.push(first_byte);
        }
    }
    log(&format!("Amount of Raw Data {}", raw_data.len()));
    // let reader = DataReader::new(raw_data);

    // for y in 0..50 {
    //     for x in 0..50 {
    //         for component in 0..3 {
    //             for b in 0..17 {}
    //             let mut index = 1;

    //             while index < 64 {
    //                 index += 1;
    //             }
    //         }
    //     }
    // }
}

#[wasm_bindgen]
pub fn hello() {
    log("Hello world");
}
