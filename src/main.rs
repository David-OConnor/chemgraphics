// We use mathematical conventions that may direct upper-case var names,
// or lower-case constants.
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![feature(non_ascii_idents)]
#![feature(vec_remove_item)]
#![feature(const_vec_new)]

#[macro_use]
extern crate ndarray;

extern crate num;

// Vulkano
#[macro_use]
extern crate vulkano;
#[macro_use]
extern crate vulkano_shader_derive;

extern crate winit;
// The `vulkano_win` crate is the link between `vulkano` and `winit`. Vulkano doesn't know about
// winit, and winit doesn't know about vulkano, so import a crate that will provide a link between
// the two.
extern crate vulkano_win;

mod input;
mod ops;
mod scenes;
mod shape_maker;
mod types;
mod transforms;
mod render;

fn main() {
    render::render();
    
}