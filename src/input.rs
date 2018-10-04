// Handles keyboard and mouse input.
use std::collections::HashMap;
use std::f32::consts::PI;

use scenes;
use transforms;
use types::{Camera, CameraType, Scene, Shape, Vec4};

const τ: f32 = 2. * PI;


// todo could move this to a util file.
fn add_arr(arr1: [f32; 3], arr2: [f32; 3]) -> [f32; 3] {
    [arr1[0] + arr2[0], arr1[1] + arr2[1], arr1[2] + arr2[2]]
}

fn add_val(arr1: [f32; 3], val: f32) -> [f32; 3] {
    [arr1[0] + val, arr1[1] + val, arr1[2] + val]
}

#[derive(Copy, Clone, Debug)]
pub enum MoveDirection{
    Forward,
    Back,
    Left,
    Right,
    Up,
    Down,
}

pub fn move_camera(direction: MoveDirection, θ: &[f32; 3], amount: f32) -> [f32; 3] {
    // Move the camera to a new position, based on where it's pointing.
    let unit_vec = match direction {
        MoveDirection::Forward => [0., 0., 1.],
        MoveDirection::Back => [0., 0., -1.],
        MoveDirection::Left => [-1., 0., 0.],
        MoveDirection::Right => [1., 0., 0.],
        MoveDirection::Up => [0., 1., 0.],
        MoveDirection::Down => [0., -1., 0.],
    };

    let v = transforms::dot_mv4(transforms::rotate(&Vec4::from_array(θ)),
                                Vec4::from_array(&unit_vec));
    
    // De-homogenize
    [
        v.x * amount, v.y * amount, v.z * amount
    ]

}

pub fn handle_pressed<'a>(pressed: &[u32], delta_time: f32,
                      scene: &'a mut Scene) -> () {
    // shape is only used when displaying single shapes.
    // delta_time is in seconds.
    let move_amount = scene.sensitivities.0 * delta_time;
    let rotate_amount = scene.sensitivities.1 * delta_time;
    let zoom_amount = scene.sensitivities.2 * delta_time;

    // Code shorteners
    let shape = scene.shapes.get_mut(&0).unwrap();

    for code in pressed {
        match *code {
            17 => {  // W
                match scene.cam_type {
                    CameraType::Single => (),
                    _ => scene.cam.position = add_arr(scene.cam.position, move_camera(MoveDirection::Forward, &scene.cam.θ, move_amount))
                }
            },
            31 => {  // S
                match scene.cam_type {
                    CameraType::Single => (),
                    _ => scene.cam.position = add_arr(scene.cam.position, move_camera(MoveDirection::Back, &scene.cam.θ, move_amount))
                }
            },
            30 => {  // A
                match scene.cam_type {
                    CameraType::Single => (),
                    _ => scene.cam.position = add_arr(scene.cam.position, move_camera(MoveDirection::Left, &scene.cam.θ, move_amount))
                }
            },
            32 => {  // D
                match scene.cam_type {
                    CameraType::Single => (),
                    _ => scene.cam.position = add_arr(scene.cam.position, move_camera(MoveDirection::Right, &scene.cam.θ, move_amount))
                }
            },
            46 => {  // C
                match scene.cam_type {
                    CameraType::Single => (),
                    CameraType::FPS => (),
                    _ => scene.cam.position = add_arr(scene.cam.position, move_camera(MoveDirection::Down, &scene.cam.θ, move_amount))
                }
            },
            29 => {  // Lctrl
                match scene.cam_type {
                    CameraType::Single => (),
                    CameraType::FPS => (),
                    _ => scene.cam.position = add_arr(scene.cam.position, move_camera(MoveDirection::Down, &scene.cam.θ, move_amount))
                }
            },
            57 => {  // Space
                match scene.cam_type {
                    CameraType::Single => (),
                    CameraType::FPS => (),
                    _ => scene.cam.position = add_arr(scene.cam.position, move_camera(MoveDirection::Up, &scene.cam.θ, move_amount))
                }
            },
 
            // Rotations around Y and Z range from 0 to τ. (clockwise rotation).
            // X rotations range from -τ/4 to τ/4 (Looking straight down to up)
            75 => {  // Left
                match scene.cam_type {
                    CameraType::Single => shape.orientation[2] -= rotate_amount,
                    _ => scene.cam.θ[2] -= rotate_amount
                }
            },
            77 => {  // Right
                match scene.cam_type {
                    CameraType::Single => shape.orientation[2] += rotate_amount,
                    _ => scene.cam.θ[2]   += rotate_amount
                }
            },
            // Don't allow us to look greater than τ/4 up or down.
            80 => {  // Down
                match scene.cam_type {
                    CameraType::Single => shape.orientation[0] -= rotate_amount,
                    _ => scene.cam.θ[0] -= rotate_amount
                }
            },
            72 => {  // Up
                match scene.cam_type {
                    CameraType::Single => shape.orientation[0] += rotate_amount,
                    _ => scene.cam.θ[0] += rotate_amount
                }
            },
            16 => {  // Q
                match scene.cam_type {
                    CameraType::Single => shape.orientation[1] -= rotate_amount,
                    _ => scene.cam.θ[1] -= rotate_amount
                }
            },
            18 => {  // E
                match scene.cam_type {
                    CameraType::Single => shape.orientation[1] += rotate_amount,
                    _ => scene.cam.θ[1] += rotate_amount
                }
            },

            // Zoom
            13 => {  // +
                scene.cam.fov -= zoom_amount; // todo specify const
                if scene.cam.fov < 0. { scene.cam.fov = 0. }
            },
            12 => {  // -
                scene.cam.fov += zoom_amount;
                if scene.cam.fov > τ { scene.cam.fov = τ }

            }
//            2 =>{ scene = &mut scene_lib[&1].clone()},  // 1
//            2 =>{ scene = &mut scenes::cube_scene(4./3.)},  // 1
//            3 => scene = &mut scene_lib[&2].clone(),  // 2

            // todo reimplement some of these
//            Keycode::V => cam.near -= 1. * ZOOM_SENSITIVITY,
//            Keycode::B => cam.near += 1. * ZOOM_SENSITIVITY,
//
//            Keycode::N => cam.far -= 1. * ZOOM_SENSITIVITY,
//            Keycode::M => cam.far += 1. * ZOOM_SENSITIVITY,
//
//            Keycode::Minus => cam.fov += 1. * ZOOM_SENSITIVITY,
//            Keycode::Equals => cam.fov -= 1. * ZOOM_SENSITIVITY,
//
//            // reset
//            Keycode::Backspace => cam = DEFAULT_CAMERA(),
            _ => (),
        }
    }

}