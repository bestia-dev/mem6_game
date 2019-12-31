//utilsmod.rs
//! small generic helper functions

///format ordinal numbers as string 1st, 2nd,3rd,...
#[allow(clippy::indexing_slicing, clippy::integer_arithmetic)]
pub fn ordinal_numbers(number: usize) -> String {
    //these are only ascii characters, so no problem with utf_8
    let mut ord_str = format!("{}", number);
    let places = ord_str.len();

    let slice = &ord_str[places - 1..];
    if slice == "1" {
        ord_str.push_str("st");
    } else if slice == "2" {
        ord_str.push_str("nd");
    } else if slice == "3" {
        ord_str.push_str("rd");
    } else {
        ord_str.push_str("th");
    }
    //return
    ord_str
}
