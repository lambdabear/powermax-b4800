pub const BATTERY_INIT_ADDR: u8 = 0x16;
pub const MAX_BATTERY_PACK_NUMBER: usize = 7;

#[derive(Debug, Clone)]
pub enum Error {
    DataLengthErr,
    EncodeErr,
    SumCheckErr,
}

#[derive(Debug, Clone)]
pub struct PowermaxB4800 {}

impl PowermaxB4800 {
    pub fn voltages_1_12_req(addr: u8) -> [u8; 9] {
        let sum: u16 = [addr as u16, 0x24, 0x01, 0x00].iter().sum();
        let sum = sum.to_le_bytes();
        [0x3A, addr, 0x24, 0x01, 0x00, sum[0], sum[1], 0x0D, 0x0A]
    }

    pub fn voltages_13_24_req(addr: u8) -> [u8; 9] {
        let sum: u16 = [addr as u16, 0x25, 0x01, 0x00].iter().sum();
        let sum = sum.to_le_bytes();
        [0x3A, addr, 0x25, 0x01, 0x00, sum[0], sum[1], 0x0D, 0x0A]
    }

    pub fn status_req(addr: u8) -> [u8; 9] {
        let sum: u16 = [addr as u16, 0x2A, 0x01, 0x00].iter().sum();
        let sum = sum.to_le_bytes();
        [0x3A, addr, 0x2A, 0x01, 0x00, sum[0], sum[1], 0x0D, 0x0A]
    }

    pub fn battery_balance_req(addr: u8) -> [u8; 9] {
        let sum: u16 = [addr as u16, 0xFE, 0x01, 0x00].iter().sum();
        let sum = sum.to_le_bytes();
        [0x3A, addr, 0xFE, 0x01, 0x00, sum[0], sum[1], 0x0D, 0x0A]
    }

    pub fn product_info_req(addr: u8) -> [u8; 9] {
        let sum: u16 = [addr as u16, 0x2B, 0x01, 0x00].iter().sum();
        let sum = sum.to_le_bytes();
        [0x3A, addr, 0x2B, 0x01, 0x00, sum[0], sum[1], 0x0D, 0x0A]
    }

    /// Get data frame from response bytes
    pub fn parse_response(bytes: &[u8]) -> Result<&[u8], Error> {
        let len = bytes.len();

        if len <= 8 {
            return Err(Error::DataLengthErr);
        }

        if bytes[0] != 0x3A || bytes[len - 2] != 0x0D || bytes[len - 1] != 0x0A {
            return Err(Error::EncodeErr);
        }

        let sum: u16 = bytes[1..len - 4].iter().map(|byte| *byte as u16).sum();
        let sum_check = u16::from_le_bytes([bytes[len - 4], bytes[len - 3]]);

        if sum != sum_check {
            return Err(Error::SumCheckErr);
        }

        let data_len = bytes[3] as usize;

        if len != (data_len + 8) {
            return Err(Error::DataLengthErr);
        }

        Ok(&bytes[4..data_len + 4])
    }
}

#[derive(Debug, Clone)]
pub struct PowermaxB4800Status {
    pub voltage: i32,
    pub current: i32,
    pub temperature1: u8,
    pub temperature2: u8,
    pub temperature3: u8,
    pub temperature4: u8,
    pub remaining_energy: i32,
    pub charging_status_code: u8,
    pub discharging_status_code: u8,
    pub charging_warning_code: u8,
    pub discharging_warning_code: u8,
    pub remaining_capacity: u8,
}

impl PowermaxB4800Status {
    pub fn from(bytes: &[u8]) -> Result<Self, Error> {
        if bytes.len() != 24 {
            return Err(Error::DataLengthErr);
        }

        let current = i32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]); // unit: mA
        let voltage = i32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]); // unit: mV
        let temperature1 = bytes[8];
        let temperature2 = bytes[9];
        let temperature3 = bytes[10];
        let temperature4 = bytes[11];
        let remaining_energy = i32::from_le_bytes([bytes[12], bytes[13], bytes[14], bytes[15]]); // unit: mAh
        let charging_status_code = bytes[16];
        let discharging_status_code = bytes[17];
        let charging_warning_code = bytes[18];
        let discharging_warning_code = bytes[19];
        let remaining_capacity = bytes[20];

        Ok(PowermaxB4800Status {
            current,
            voltage,
            temperature1,
            temperature2,
            temperature3,
            temperature4,
            remaining_energy,
            charging_status_code,
            discharging_status_code,
            charging_warning_code,
            discharging_warning_code,
            remaining_capacity,
        })
    }
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
