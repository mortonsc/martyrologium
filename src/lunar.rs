use chrono::{Datelike, NaiveDate};

pub const LUNAR_LETTER_COUNT: usize = 31;
pub const LUNAR_LETTERS: [&'static str; LUNAR_LETTER_COUNT] = [
    "a", "b", "c", "d", "e", "f", "g", "h", "i", "k", "l", "m", "n", "p", "q", "r", "s", "t", "u",
    "A", "B", "C", "D", "E", "F1", "F2", "G", "H", "M", "N", "P",
];

pub const A_INDEX: usize = 19;
pub const F2_INDEX: usize = 25;

const CYCLE_START: u32 = 6;
const CYCLE_LENGTH: u32 = 59;

fn lunar_numbers_1(month: u32, day: u32) -> (u32, u32, Option<u32>) {
    let ord = NaiveDate::from_ymd_opt(2001, month, day).unwrap().ordinal();
    if ord < CYCLE_START {
        return (ord + 25, 30, None);
    }
    let position = (ord - CYCLE_START) % CYCLE_LENGTH;
    match position {
        0..29 => (position + 1, 30, None),
        29 => (30, 30, Some(1)),
        30..58 => (position - 29, 29, None),
        58 => (29, 29, Some(30)),
        _ => panic!(),
    }
}

fn lunar_numbers_2(F2_value: u32, modulus: u32, F1_value: Option<u32>) -> Vec<u32> {
    let mut result = vec![0; LUNAR_LETTER_COUNT];
    for i in 0..LUNAR_LETTER_COUNT {
        let index = (F2_INDEX + i) % LUNAR_LETTER_COUNT;
        let value = (((F2_value - 1) + (i as u32)) % modulus) + 1;
        result[index] = value;
    }
    if let Some(val) = F1_value {
        result[F2_INDEX - 1] = val;
    }
    result
}

pub fn lunar_numbers(month: u32, day: u32) -> Vec<u32> {
    let (F2_value, modulus, F1_value) = lunar_numbers_1(month, day);
    lunar_numbers_2(F2_value, modulus, F1_value)
}

#[cfg(test)]
mod test {
    use super::*;

    fn assert_eq_lunar_numbers(month: u32, day: u32, correct: [u32; LUNAR_LETTER_COUNT]) {
        let output = lunar_numbers(month, day);
        assert_eq!(output.len(), LUNAR_LETTER_COUNT);
        for i in 0..LUNAR_LETTER_COUNT {
            assert_eq!(output[i], correct[i]);
        }
    }

    #[test]
    fn test_lunar_numbers() {
        assert_eq_lunar_numbers(
            1,
            2,
            [
                3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
                25, 26, 27, 27, 28, 29, 30, 1, 2,
            ],
        );
        assert_eq_lunar_numbers(
            2,
            4,
            [
                6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27,
                28, 29, 1, 30, 1, 2, 3, 4, 5,
            ],
        );
        assert_eq_lunar_numbers(
            3,
            5,
            [
                6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27,
                28, 29, 30, 29, 1, 2, 3, 4, 5,
            ],
        );
        assert_eq_lunar_numbers(
            10,
            20,
            [
                28, 29, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                22, 23, 22, 23, 24, 25, 26, 27,
            ],
        );
        assert_eq_lunar_numbers(
            11,
            1,
            [
                11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 1,
                2, 3, 4, 5, 5, 6, 7, 8, 9, 10,
            ],
        );
    }
}
