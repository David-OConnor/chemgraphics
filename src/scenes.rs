// Set up different combinations of shapes, camera, adn other variables.
use std::collections::HashMap;
use std::f32::consts::PI;

use ndarray::prelude::*;

use shape_maker;
use types::{Camera, Lighting, Scene, Shape, CameraType};

const τ: f32 = 2. * PI;
const SHAPE_OP: f32 = 0.3;

const base_lighting: Lighting = Lighting {
        ambient_intensity: 0.8,
        diffuse_intensity: 0.6,
        ambient_color: [1.0, 1.0, 1.0, 0.6],
        diffuse_color: [1., 1., 1., 1.0],
        diffuse_direction: [0., 0., -1.],
        sources: Vec::new(),
};

fn base_camera() -> Camera {
    // function instead of a const, due to the ndarrays.
    Camera {
        position: Array::zeros(4),
        θ: Array::zeros(3),
        fov: τ / 5.,
        aspect: 1.,
        near: 0.05,
        far: 600.,
    }
}

fn make_single_scene(aspect: f32, shape: Shape) -> Scene {
    let mut shapes = HashMap::new();
    shapes.insert(0, shape);
    Scene {
        shapes,
        cam: Camera {
            position: array![0., 0., 1.],
            θ: array![0., 0., 0.],
            fov: τ / 5.5,
            aspect,
            ..base_camera()
        },
        cam_type: CameraType::Free,
        lighting: base_lighting,
        sensitivities: (0.1, 0.5, 0.2),
    }
}

pub fn cube_scene(aspect: f32) -> Scene {
    make_single_scene(aspect, Shape::new(shape_maker::cube(1.), Array::zeros(3),
                                         Array::zeros(3), Array::zeros(3), SHAPE_OP))
}

