use std::f32::consts::PI;

use crate::DataReader;

pub fn get_coeff(reader: &mut DataReader, len: i8) -> i64 {
    if len == 0 {
        return 0;
    }

    let mut coeff: i64 = 0;
    for _ in 0..len {
        coeff = (coeff << 1) + reader.get_bit() as i64;
    }

    if coeff < (1 << (len - 1)) {
        return coeff - ((1 << len) - 1);
    }

    coeff
}

fn clamp(num: i64) -> u8 {
    if num > 255 {
        return 255;
    }

    if num < 0 {
        return 0;
    }

    num as u8
}
pub fn convert_color(y: i64, cb: i64, cr: i64) -> [u8; 3] {
    let r = (y as f64 + cr as f64 * 1.402) as i64;
    let g = (y as f64 - 0.344 * cb as f64 - 0.714 * cr as f64) as i64;
    let b = (y as f64 + 1.772 * cb as f64) as i64;

    [clamp(r + 128), clamp(g + 128), clamp(b + 128)]
}

pub fn create_u16(num1: u8, num2: u8) -> u16 {
    let mut new_u16_int: u16 = 0x0000;

    new_u16_int = new_u16_int + num1 as u16;
    new_u16_int = new_u16_int << 8;
    new_u16_int = new_u16_int + num2 as u16;

    new_u16_int
}

fn fn_of_a(input: usize) -> f32 {
    if input == 0 {
        return 1.0 / (2.0 as f32).sqrt();
    }

    1.0
}

pub const ZIG_ZAG: [u8; 64] = [
    0, 1, 8, 16, 9, 2, 3, 10, 17, 24, 32, 25, 18, 11, 4, 5, 12, 19, 26, 33, 40, 48, 41, 34, 27, 20,
    13, 6, 7, 14, 21, 28, 35, 42, 49, 56, 57, 50, 43, 36, 29, 22, 15, 23, 30, 37, 44, 51, 58, 59,
    52, 45, 38, 31, 39, 46, 53, 60, 61, 54, 47, 55, 62, 63,
];

fn inverse_dct(arg1: usize, arg2: usize) -> f32 {
    (((2.0 * (arg1 as f32) + 1.0) * (arg2 as f32) * PI) / 16.0).cos()
}

pub fn create_idct_table() -> [f32; 64] {
    let mut table: [f32; 64] = [0.0; 64];

    for i in 0..8 {
        for j in 0..8 {
            table[i * 8 + j] = fn_of_a(i) * inverse_dct(j, i);
        }
    }

    table
}
