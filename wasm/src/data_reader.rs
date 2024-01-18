pub mod data_reader {
    pub struct DataReader {
        raw_data: Vec<u8>,
        bit_pos: u8,
    }

    impl DataReader {
        // Constructor method to create instances
        pub fn new(data: Vec<u8>) -> Self {
            DataReader {
                raw_data: data,
                bit_pos: 0,
            }
        }
        pub fn getBit(&mut self) -> u8 {
            let bit = self.raw_data.get(0).unwrap() << self.bit_pos;

            if self.bit_pos == 7 {
                self.raw_data.remove(0);
            }
            self.bit_pos = (self.bit_pos + 1) % 8;

            bit
        }
    }
}
