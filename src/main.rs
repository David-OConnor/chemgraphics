// We use mathematical conventions that may direct upper-case var names,
// or lower-case constants.
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![feature(non_ascii_idents)]
#![feature(vec_remove_item)]
#![feature(const_vec_new)]

#[macro_use]
extern crate ndarray;

// Vulkano
#[macro_use]
extern crate vulkano;
#[macro_use]
extern crate vulkano_shader_derive;
extern crate vulkano_win;
extern crate winit;

mod input;
mod scenes;
mod shape_maker;
mod types;
mod transforms;
mod render;

use std::collections::HashMap;

fn main() {
    println!("{}", "HELLO Chemgraphics!");
     render::render();
    
}