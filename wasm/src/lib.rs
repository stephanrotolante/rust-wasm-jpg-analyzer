mod component;
mod console_log;
mod data_reader;
mod huffman_table;
mod jpeg;
mod utils;

use component::Component;
use console_log::*;
use data_reader::DataReader;
use huffman_table::HuffmanTable;
use jpeg::Jpeg;
use utils::get_coeff;
use utils::{convert_color, create_idct_table, create_u16, ZIG_ZAG};
use wasm_bindgen::prelude::*;
use web_sys::Element;

#[wasm_bindgen]
pub fn decode(data: Vec<u8>) {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let element: Element;

    if let Some(_element) = document.get_element_by_id("myCanvas") {
        element = _element;
    } else {
        log(&format!("Can't find the canvas"));
        return;
    }

    let canvas: web_sys::HtmlCanvasElement = element
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let mut data_clone = data.clone();
    log("This is the size");
    log(&data.len().to_string());

    if data.len() < 2 {
        log("Where are my bytes bro!?");
        return;
    }

    let mut jpeg_data = Jpeg::new();

    let first_byte = data_clone.remove(0);

    let second_byte = data_clone.remove(0);

    if create_u16(first_byte, second_byte) != 0xFFD8 {
        log("Bad header homie!");
        log(&format!("{:02X} {:02X}", first_byte, second_byte));
        return;
    }

    let idct_table = create_idct_table();

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
                    let segment_length = create_u16(data_clone.remove(0), data_clone.remove(0));
                    log(&format!("Length {}", segment_length));

                    let sliced_data = &mut data_clone[0..(segment_length - 2) as usize].to_vec();
                    data_clone.drain(..(segment_length - 2) as usize);

                    log(&format!(
                        "Identifier {:02X}{:02X}{:02X}{:02X}\\{:02X}",
                        sliced_data[0],
                        sliced_data[1],
                        sliced_data[2],
                        sliced_data[3],
                        sliced_data[4]
                    ));
                    log(&format!("Version {}.{}", sliced_data[5], sliced_data[6]));

                    log(&format!("Units {}", sliced_data[7]));

                    log(&format!(
                        "X Units {}",
                        create_u16(sliced_data[8], sliced_data[9])
                    ));

                    log(&format!(
                        "Y Units {}",
                        create_u16(sliced_data[10], sliced_data[11])
                    ));

                    log(&format!("X ThumbNail {}", sliced_data[12]));

                    log(&format!("Y ThumbNail {}", sliced_data[13]));

                    log("***********");
                }
                // Quantization Table
                0xDB => {
                    log("QT");
                    let segment_length = create_u16(data_clone.remove(0), data_clone.remove(0));
                    log(&format!("Length {}", segment_length));
                    let sliced_data = &mut data_clone[0..(segment_length - 2) as usize].to_vec();
                    data_clone.drain(..(segment_length - 2) as usize);

                    let table_id = sliced_data.remove(0);

                    log(&format!("Luminance/Chrominance {}", table_id));

                    let mut new_q_table: [u8; 64] = [0; 64];
                    for i in 0..8 {
                        for j in 0..8 {
                            new_q_table[i * 8 + j] = sliced_data.remove(0);
                        }
                    }

                    jpeg_data.quantize_tables[table_id as usize] = new_q_table;
                    log("***********");
                }
                // Start of Frame
                0xC0 => {
                    log("SOF");
                    let segment_length = create_u16(data_clone.remove(0), data_clone.remove(0));
                    log(&format!("Length {}", segment_length));
                    let sliced_data = &mut data_clone[0..(segment_length - 2) as usize].to_vec();
                    data_clone.drain(..(segment_length - 2) as usize);

                    log(&format!("Precision {}", sliced_data.remove(0)));
                    jpeg_data.height =
                        create_u16(sliced_data.remove(0), sliced_data.remove(0)) as i64;
                    log(&format!("Line No {}", jpeg_data.height));

                    jpeg_data.width =
                        create_u16(sliced_data.remove(0), sliced_data.remove(0)) as i64;
                    log(&format!("Samples Per Line {}", jpeg_data.width));

                    let number_of_components = sliced_data.remove(0) as usize;

                    for _i in 0..number_of_components {
                        let component_num = sliced_data.remove(0);

                        let samples = sliced_data.remove(0);
                        let y_sample = samples & 0x0F;
                        let x_sample = (samples & 0xF0) >> 4;

                        let quantize_id = sliced_data.remove(0);

                        jpeg_data.components.push(
                            Component::new(component_num)
                                .set_quantize_id(quantize_id)
                                .set_x_sample(x_sample)
                                .set_y_sample(y_sample)
                                .to_owned(),
                        );

                        log(&format!(
                            "Component {} {}x{} {}",
                            component_num, x_sample, y_sample, quantize_id
                        ));
                    }

                    log("***********");
                }
                // Start of Scan
                0xDA => {
                    log("SOC");
                    let segment_length = create_u16(data_clone.remove(0), data_clone.remove(0));
                    log(&format!("Length {}", segment_length));

                    let sliced_data = &mut data_clone[0..(segment_length - 2) as usize].to_vec();
                    data_clone.drain(..(segment_length - 2) as usize);

                    let number_of_components = sliced_data.remove(0);

                    for component_index in 0..number_of_components {
                        let component_num = sliced_data.remove(0);

                        let tables = sliced_data.remove(0);
                        let dc_table_id = (tables & 0xF0) >> 4;
                        let ac_table_id = tables & 0x0F;

                        match jpeg_data.components.get_mut(component_index as usize) {
                            Some(component) => {
                                component.set_ac_id(ac_table_id).set_dc_id(dc_table_id);
                            }
                            None => {
                                log(&format!("Cannot find index in vec {}", component_index));
                            }
                        }

                        log(&format!(
                            "Component {} DC {} AC {}",
                            component_num, dc_table_id, ac_table_id
                        ));
                    }
                    log("***********");
                }
                // Huffman Table
                0xC4 => {
                    log("HUF");
                    let segment_length = create_u16(data_clone.remove(0), data_clone.remove(0));
                    log(&format!("Length {}", segment_length));

                    let sliced_data = &mut data_clone[0..(segment_length - 2) as usize].to_vec();
                    data_clone.drain(..(segment_length - 2) as usize);

                    let b1: u8 = sliced_data.remove(0);

                    let class = (b1 & 0xF0) >> 4;
                    let table_id = b1 & 0x0F;

                    if class == 0 {
                        log("Class DC");
                    } else {
                        log("Class AC");
                    }

                    log(&format!("Table ID {}", table_id));

                    log(&format!(
                        "Number of Symbols {}",
                        segment_length - 1 - 2 - 16
                    ));

                    let mut frequency_counts = [0; 16];

                    let mut symbols: [Vec<u8>; 16] = Default::default();
                    let mut codes: [Vec<u16>; 16] = Default::default();

                    for i in 0..16 {
                        frequency_counts[i] = sliced_data.remove(0);
                    }

                    let mut code = 0x0000;

                    // Generate Codes
                    for i in 0..16 {
                        for _j in 0..frequency_counts[i] {
                            symbols[i].push(sliced_data.remove(0));
                            codes[i].push(code);
                            code += 1;
                        }
                        code = code << 1
                    }

                    if class == 0 {
                        jpeg_data.dc_huffman_tables[table_id as usize] =
                            HuffmanTable::new(codes, symbols);
                    } else {
                        jpeg_data.ac_huffman_tables[table_id as usize] =
                            HuffmanTable::new(codes, symbols);
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

    let mut reader: DataReader = DataReader::new(raw_data);

    for _y in 0..jpeg_data.height / 8 {
        for _x in 0..jpeg_data.width / 8 {
            for component_index in 0..3 {
                let mut mcu: [i64; 64] = [0; 64];
                if let Some(component) = jpeg_data.components.get_mut(component_index as usize) {
                    let dc_id = component.get_dc_id();
                    if dc_id == -1 {
                        log(&format!("Invalid DC id table"));
                        return;
                    }
                    let coeff_length =
                        jpeg_data.dc_huffman_tables[dc_id as usize].get_symbol(&mut reader) & 0x0F;

                    let coeff = get_coeff(&mut reader, coeff_length);

                    let q_value = jpeg_data
                        .quantize_tables
                        .get(component.get_quantize_id() as usize)
                        .unwrap()[0];

                    mcu[0] = component.add_dc_coeff(coeff) * q_value as i64;

                    let mut index = 1;
                    while index < 64 {
                        let ac_id = component.get_ac_id();
                        if ac_id == -1 {
                            log(&format!("Invalid AC id table"));
                            return;
                        }
                        let symbol =
                            jpeg_data.ac_huffman_tables[ac_id as usize].get_symbol(&mut reader);

                        let coeff_length = symbol & 0x0F;
                        let number_of_zeros = (symbol >> 4) & 0x0F;

                        if symbol == 0x00 || index + number_of_zeros > 64 {
                            break;
                        }

                        index += number_of_zeros;

                        let coeff = get_coeff(&mut reader, coeff_length);

                        let q_value: u8 = jpeg_data
                            .quantize_tables
                            .get(component.get_quantize_id() as usize)
                            .unwrap()[index as usize];

                        mcu[ZIG_ZAG[index as usize] as usize] = coeff * q_value as i64;
                        index += 1;
                    }
                } else {
                    log(&format!("Invalid Component"));
                    return;
                }

                jpeg_data.mcus.push(mcu);
            }
        }
    }

    let mut y = 0;
    let mut x = 0;

    for mcu_index in (0..jpeg_data.mcus.len()).step_by(jpeg_data.components.len()) {
        y = (mcu_index / 3) as i64 / (jpeg_data.width / 8);
        x = (mcu_index / 3) as i64 % (jpeg_data.width / 8);

        for i in 0..jpeg_data.components.len() {
            let temp_data = jpeg_data.mcus.get(mcu_index + i).unwrap().clone();

            for y in 0..8 {
                for x in 0..8 {
                    let mut sum: f64 = 0.0;
                    for n in 0..8 {
                        for m in 0..8 {
                            sum += (temp_data[n * 8 + m] as f64)
                                * (idct_table[m * 8 + x] as f64)
                                * (idct_table[n * 8 + y] as f64);
                        }
                    }

                    jpeg_data.mcus.get_mut(mcu_index + i).unwrap()[y * 8 + x] = (sum / 4.0) as i64;
                }
            }
        }

        for yy in 0..8 {
            for xx in 0..8 {
                let colors = convert_color(
                    jpeg_data.mcus.get(mcu_index).unwrap()[8 * yy + xx],
                    jpeg_data.mcus.get(mcu_index + 1).unwrap()[8 * yy + xx],
                    jpeg_data.mcus.get(mcu_index + 2).unwrap()[8 * yy + xx],
                );
                let y_pos: i64 = y * 8 + yy as i64;
                let x_pos = x * 8 + xx as i64;
                context.set_fill_style(&JsValue::from_str(&format!(
                    "rgb({}, {}, {})",
                    colors[0], colors[1], colors[2]
                )));
                context.fill_rect(x_pos as f64, y_pos as f64, 1.0, 1.0)
            }
        }
    }
}
