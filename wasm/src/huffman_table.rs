use crate::DataReader;

#[derive(Debug, Default)]
pub struct HuffmanTable {
    codes: [Vec<u16>; 16],
    symbols: [Vec<u8>; 16],
}
impl HuffmanTable {
    pub fn new(code_data: [Vec<u16>; 16], symbol_data: [Vec<u8>; 16]) -> Self {
        HuffmanTable {
            codes: code_data,
            symbols: symbol_data,
        }
    }
    pub fn get_symbol(&mut self, reader: &mut DataReader) -> i8 {
        let mut bits = 0x0000;
        for bit_length in 0..17 {
            bits = (bits << 1) + reader.get_bit() as u16;
            for (index, code) in self.codes[bit_length as usize].iter().enumerate() {
                if bits == *code {
                    return *self.symbols[bit_length as usize].get(index).unwrap() as i8;
                }
            }
        }
        return -1 as i8;
    }
}
