// Set up different combinations of shapes, camera, adn other variables.
use std::collections::HashMap;
use std::f32::consts::PI;

use shape_maker;
use types::{Camera, Lighting, Scene, Shape, CameraType};

const τ: f32 = 2. * PI;

const base_lighting: Lighting = Lighting {
        ambient_intensity: 0.8,
        diffuse_intensity: 0.6,
        ambient_color: [1.0, 1.0, 1.0, 0.6],
        diffuse_color: [1., 1., 1., 1.0],
        diffuse_direction: [0., 0., -1.],
        sources: Vec::new(),
};

const base_camera: Camera = Camera {
    position: [0., 1., 2.],
    θ: [0., 0., 0.],
    fov: τ / 5.,
    aspect: 1.,
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
        sensitivities: (2.5, 0.5, 0.2),
    }
}

pub fn scene_1(aspect: f32) -> Scene {
    make_scene(aspect, vec![
        Shape::new(shape_maker::cube(0.5), [0., 0., 0.], [0., 0., 0.]),
        Shape::new(shape_maker::cube(0.5), [0.4, 0., 0.], [0., 0., 0.]),
    ])
}

