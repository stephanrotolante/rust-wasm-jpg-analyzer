use crate::Component;
use crate::HuffmanTable;
pub struct Jpeg {
    pub height: i64,
    pub width: i64,
    pub mcus: Vec<[i64; 64]>,
    pub components: Vec<Component>,
    pub quantize_tables: Vec<[u8; 64]>,
    pub ac_huffman_tables: [HuffmanTable; 2],
    pub dc_huffman_tables: [HuffmanTable; 2],
}

impl Jpeg {
    pub fn new() -> Self {
        Jpeg {
            ac_huffman_tables: Default::default(),
            dc_huffman_tables: Default::default(),
            components: Vec::<Component>::new(),
            quantize_tables: Vec::<[u8; 64]>::new(),
            mcus: Vec::<[i64; 64]>::new(),
            height: 0,
            width: 0,
        }
    }
}
