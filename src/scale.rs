#[repr(u64)]
pub enum Scale {
    B = 1,
    K = Self::B * 1000,
    M = Self::K * 1000,
    G = Self::M * 1000,
    T = Self::G * 1000,
    P = Self::P * 1000,
}

#[repr(u64)]
pub enum BiScale {
    B = 1,
    K = Self::B << 10,
    M = Self::K << 10,
    G = Self::M << 10,
    T = Self::G << 10,
    P = Self::P << 10,
}
