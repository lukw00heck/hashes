#![no_std]
#![feature(test)]
#[macro_use]
extern crate digest;
extern crate streebog;

bench_digest!(streebog::Streebog512);
