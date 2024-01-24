use crate::log;
#[derive(Clone)]
pub struct Component {
    x_sample: u8,
    y_sample: u8,
    dc_coeff: i64,
    component_num: u8,
    dc_huffman_id: Option<u8>,
    ac_huffman_id: Option<u8>,
    quantize_id: Option<u8>,
}

impl Component {
    pub fn new(component: u8) -> Self {
        Component {
            dc_coeff: 0,
            x_sample: 0,
            y_sample: 0,
            component_num: component,
            dc_huffman_id: None,
            ac_huffman_id: None,
            quantize_id: None,
        }
    }

    pub fn add_dc_coeff(&mut self, new_coeff: i64) -> i64 {
        self.dc_coeff += new_coeff;
        self.dc_coeff
    }

    pub fn set_dc_id(&mut self, id: u8) -> &mut Self {
        self.dc_huffman_id = Option::Some(id);
        self
    }

    pub fn set_ac_id(&mut self, id: u8) -> &mut Self {
        self.ac_huffman_id = Option::Some(id);
        self
    }

    pub fn set_quantize_id(&mut self, id: u8) -> &mut Self {
        self.quantize_id = Option::Some(id);
        self
    }

    pub fn set_x_sample(&mut self, id: u8) -> &mut Self {
        self.ac_huffman_id = Option::Some(id);
        self
    }

    pub fn set_y_sample(&mut self, id: u8) -> &mut Self {
        self.quantize_id = Option::Some(id);
        self
    }

    pub fn get_dc_id(&self) -> i8 {
        match self.dc_huffman_id {
            None => {
                log(&format!("{} invalid dc id", self.component_num));
                -1
            }
            Some(id) => id as i8,
        }
    }
    pub fn get_ac_id(&self) -> i8 {
        match self.ac_huffman_id {
            None => {
                log(&format!("{} invalid ac id", self.component_num));
                -1
            }
            Some(id) => id as i8,
        }
    }
    pub fn get_quantize_id(&self) -> i8 {
        match self.quantize_id {
            None => {
                log(&format!("{} invalid quantize id", self.component_num));
                -1
            }
            Some(id) => id as i8,
        }
    }
}
