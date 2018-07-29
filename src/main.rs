#![feature(use_extern_macros)]
#![feature(extern_prelude)]

extern crate gtk;
#[macro_use]
extern crate relm;
#[macro_use]
extern crate relm_derive;

extern crate treexml;

use relm::Widget;

mod database;

mod windows;
use windows::*;

fn main() {
    MainWindow::run(()).unwrap();
}
