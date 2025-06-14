#[derive(Clone)]
pub struct TTEntry {
    data: u16,
}

impl TTEntry {
    pub fn new(depth: u8, score: i8, flag: i8) -> Self {
        let mut entry = Self { data: 0u16 };
        entry.set_flag(flag);
        entry.set_depth(depth);
        entry.set_score(score);
        entry
    }

    pub fn score(&self) -> i8 {
        let raw = self.data & 0b0111_1111;
        let magnitude = (raw & 0b0011_1111) as i8;
        let sign_bit = (raw & 0b0100_0000) != 0;
        if sign_bit {
            -magnitude
        } else {
            magnitude
        }
    }

    pub fn flag(&self) -> i8 {
        match (self.data >> 14) & 0b11 {
            0b01 => -1,
            0b10 => 1,
            0b00 => 0,
            _ => 0,
        }
    }

    pub fn depth(&self) -> u8 {
        ((self.data >> 7) & 0b0111_1111) as u8
    }

    fn set_depth(&mut self, depth: u8) {
        let depth = depth & 0b0111_1111;
        self.data &= !(0b0111_1111 << 7);
        self.data |= (depth as u16) << 7;
    }

    fn set_score(&mut self, score: i8) {
        let mut s = score.abs() & 0b0011_1111;
        if score < 0 {
            s |= 0b0100_0000;
        }
        self.data &= !0b0111_1111;
        self.data |= s as u16;
    }

    fn set_flag(&mut self, flag: i8) {
        self.data &= !(0b11 << 14);
        match flag {
            -1 => self.data |= 0b01 << 14,
            1 => self.data |= 0b10 << 14,
            0 => {}
            _ => panic!("Cannot encode unknown flag."),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score_encoding() {
        for val in -63..=63 {
            let entry = TTEntry::new(0, val, 0);
            assert_eq!(
                entry.score(),
                val,
                "Score encoding/decoding failed for {}",
                val
            );
        }
    }

    #[test]
    fn test_depth_encoding() {
        for depth in 0..=127 {
            let entry = TTEntry::new(depth, 0, 0);
            assert_eq!(
                entry.depth(),
                depth,
                "Depth encoding/decoding failed for {}",
                depth
            );
        }
    }

    #[test]
    fn test_flag_encoding() {
        let entry1 = TTEntry::new(0, 0, -1);
        assert_eq!(entry1.flag(), -1);

        let entry2 = TTEntry::new(0, 0, 1);
        assert_eq!(entry2.flag(), 1);

        let entry3 = TTEntry::new(0, 0, 0);
        assert_eq!(entry3.flag(), 0);
    }

    #[test]
    #[should_panic(expected = "Cannot encode unknown flag.")]
    fn test_invalid_flag_panics() {
        TTEntry::new(0, 0, 5);
    }

    #[test]
    fn test_combined_encoding() {
        let entry = TTEntry::new(42, -17, 1);
        assert_eq!(entry.depth(), 42);
        assert_eq!(entry.score(), -17);
        assert_eq!(entry.flag(), 1);
    }
}
