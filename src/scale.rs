#[derive(Copy, Clone)]
#[repr(u64)]
pub enum Scale {
    B = 1,
    K = (Self::B as u64) * 1000,
    M = (Self::K as u64) * 1000,
    G = (Self::M as u64) * 1000,
    T = (Self::G as u64) * 1000,
    P = (Self::T as u64) * 1000,
}

#[derive(Copy, Clone)]
#[repr(u64)]
pub enum BiScale {
    B = 1,
    K = (Self::B as u64) << 10,
    M = (Self::K as u64) << 10,
    G = (Self::M as u64) << 10,
    T = (Self::G as u64) << 10,
    P = (Self::T as u64) << 10,
}
