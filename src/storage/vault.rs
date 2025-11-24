use rand::random;

#[derive(Debug)]
pub struct RootValut {
    pub magic: [u8; 4],
    pub version: u16,
    pub salt: [u8; 16],
    pub n_regs: u16,
}

impl RootValut {
    pub fn new() -> Self {
        let f = Self {
            magic: [0x50, 0x57, 0x4D, 0x4E],
            version: 1,
            salt: rand::random(),
            n_regs: 0,
        };
        f
    }
}
