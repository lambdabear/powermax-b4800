#[derive(Debug, Clone)]
pub struct PowermaxB4800 {}

impl PowermaxB4800 {
    pub fn voltages_1_12_req(addr: u8) -> [u8; 9] {
        let sum: u16 = [addr as u16, 0x24, 0x01, 0x00].iter().sum();
        let sum = sum.to_le_bytes();
        [0x3A, addr, 0x24, 0x01, 0x00, sum[1], sum[0], 0x0D, 0x0A]
    }

    pub fn voltages_13_24_req(addr: u8) -> [u8; 9] {
        let sum: u16 = [addr as u16, 0x25, 0x01, 0x00].iter().sum();
        let sum = sum.to_le_bytes();
        [0x3A, addr, 0x25, 0x01, 0x00, sum[1], sum[0], 0x0D, 0x0A]
    }

    pub fn state_req(addr: u8) -> [u8; 9] {
        let sum: u16 = [addr as u16, 0x2A, 0x01, 0x00].iter().sum();
        let sum = sum.to_le_bytes();
        [0x3A, addr, 0x2A, 0x01, 0x00, sum[1], sum[0], 0x0D, 0x0A]
    }

    pub fn battery_balance_req(addr: u8) -> [u8; 9] {
        let sum: u16 = [addr as u16, 0xFE, 0x01, 0x00].iter().sum();
        let sum = sum.to_le_bytes();
        [0x3A, addr, 0xFE, 0x01, 0x00, sum[1], sum[0], 0x0D, 0x0A]
    }

    pub fn product_info_req(addr: u8) -> [u8; 9] {
        let sum: u16 = [addr as u16, 0x2B, 0x01, 0x00].iter().sum();
        let sum = sum.to_le_bytes();
        [0x3A, addr, 0x2B, 0x01, 0x00, sum[1], sum[0], 0x0D, 0x0A]
    }
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
