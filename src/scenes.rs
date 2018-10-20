// Set up different combinations of shapes, camera, adn other variables.
use std::collections::HashMap;
use std::f32::consts::PI;

use shape_maker;
use types::{Camera, Lighting, Scene, Shape, CameraType};

const τ: f32 = 2. * PI;

const base_lighting: Lighting = Lighting {
        ambient_intensity: 0.8,
        diffuse_intensity: 0.6,
        ambient_color: [0., 0., 0.5, 1.0],
        diffuse_color: [0.5, 1., 0.5, 1.0],
        diffuse_direction: [-1., -1., -1.],
        sources: Vec::new(),
};

const base_camera: Camera = Camera {
    position: [0., 0., -7.],
    θ: [0., 0., 0.],
    fov: τ / 4.,
    aspect: 4./3.,
    near: 0.1,
    far: 100.,
};

fn make_scene(aspect: f32, shapes: Vec<Shape>) -> Scene {
    let shapes: HashMap<u32, Shape> = shapes.into_iter().enumerate()
        .map(|(i, shape)| (i as u32, shape)).collect();

    Scene {
        shapes,
        cam: base_camera,
        cam_type: CameraType::Free,
        lighting: base_lighting,
        sensitivities: (2., 2., 0.2),
    }
}


pub fn make_nucleus(protons: u8, neutrons: u8, position: [f32; 3]) -> Shape {
    // todo returns a cube; change to sphere
//    let size = (protons + neutrons) as f32 * 0.2
    // todo temp
    let size = 1.;
    Shape::new(shape_maker::cube(size), position, [0., 0., 0.])
}

//pub fn make_electron(protons: u8, neutrons: u8, position: [f32; 3]) -> Shape {
//    // todo returns a cube; change to sphere
//    let size = (protons + neutrons) as f32 * 0.2;
//    Shape::new(shape_maker::cube(size), position, [0., 0., 0.])
//}

pub fn scene_1(aspect: f32) -> Scene {
    make_scene(aspect, vec![
        make_nucleus(8, 8, [0., 0., 0.]),
        make_nucleus(8, 8, [1., -1., -4.]),
        make_nucleus(8, 8, [-3., 4., 0.]),
    ])
}

