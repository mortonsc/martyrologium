use crate::lunar::*;
use std::io::Write;

fn write_lunar_table(month: u32, day: u32, mut w: impl Write) {
    let lnums = lunar_numbers(month, day);
    write!(w, "hi!");
}
