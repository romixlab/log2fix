#![feature(unchecked_math)]

/// Inverse log base 2 of e
pub const INV_LOG2_E_Q1DOT31: u64 = 0x58b90bfc;
/// Inverse log base 2 of 10
pub const INV_LOG2_10_Q1DOT31: u64 = 0x268826a1;

/// This implementation is based on Clay. S. Turner's fast binary logarithm
/// algorithm.
/// C. S. Turner,  "A Fast Binary Logarithm Algorithm", IEEE Signal
/// Processing Mag., pp. 124,140, Sep. 2010.
pub fn log2fix(x: u32, precision: usize) -> i32 {
    let mut x = x;
    let mut b: i32 = 1 << (precision - 1);
    let mut y: i32 = 0;

    if precision < 1 || precision > 31 {
        return i32::MAX;
    }

    if x == 0 {
        return i32::MIN;
    }

    while x < (1 << precision) {
        x = x << 1;
        y -= 1 << precision;
    }

    while x >= (2 << precision) {
        x = x >> 1;
        y += 1 << precision;
    }

    let mut z = x as u64;
    for _ in 0..precision {
        z = (z * z) >> precision;
        if z >= (2 << precision) {
            z = z >> 1;
            y += b;
        }
        b = b >> 1;
    }
    y
}

pub fn logefix(x: u32, precision: usize) -> i32 {
    let t = unsafe { (log2fix(x, precision) as u64).unchecked_mul(INV_LOG2_E_Q1DOT31) };
    (t >> 31) as i32
}

pub fn log10fix(x: u32, precision: usize) -> i32 {
    let t = unsafe { (log2fix(x, precision) as u64).unchecked_mul(INV_LOG2_10_Q1DOT31) };
    (t >> 31) as i32
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
