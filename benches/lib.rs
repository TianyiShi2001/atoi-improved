#![feature(test)]
extern crate test;
extern crate atoi;
use atoi::{FromRadix10, FromRadix10Checked};
use std::str;

#[bench]
fn i32_four_digit_number(b: &mut test::Bencher) {
    b.iter(|| i32::from_radix_10(test::black_box(b"1996")))
}

#[bench]
fn i32_four_digit_number_checked(b: &mut test::Bencher) {
    b.iter(|| i32::from_radix_10_checked(test::black_box(b"1996")))
}

#[bench]
fn u32_four_digit_number(b: &mut test::Bencher) {
    b.iter(|| u32::from_radix_10(test::black_box(b"1996")))
}

#[bench]
fn u32_four_digit_number_checked(b: &mut test::Bencher) {
    b.iter(|| u32::from_radix_10_checked(test::black_box(b"1996")))
}

#[bench]
fn i32_four_hex_digit_number(b: &mut test::Bencher) {
    b.iter(|| i32::from_radix_10(test::black_box(b"1996")))
}

#[bench]
fn i32_four_hex_digit_number_checked(b: &mut test::Bencher) {
    b.iter(|| i32::from_radix_10_checked(test::black_box(b"1996")))
}

#[bench]
fn u32_four_hex_digit_number(b: &mut test::Bencher) {
    b.iter(|| u32::from_radix_10(test::black_box(b"1996")))
}

#[bench]
fn u32_four_hex_digit_number_checked(b: &mut test::Bencher) {
    b.iter(|| u32::from_radix_10_checked(test::black_box(b"1996")))
}


#[bench]
fn through_utf8(b: &mut test::Bencher) {
    b.iter(|| {
        let s = str::from_utf8(test::black_box(b"1996")).unwrap();
        s.parse::<u32>().unwrap();
    })
}