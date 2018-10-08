use std::collections::HashMap;
use std::ops::{Add, Sub, Mul};

use ndarray::prelude::*;

// todo ndarrays, or builtin arrays? We need to enforce length of items.

//#[derive(Debug)]
//pub struct Pt2D {
//    pub x: f32,
//    pub y: f32,
//}

#[derive(Copy, Clone, Debug)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Vec4 {
        Vec4 {x, y, z, w}
    }

    pub fn from_array(vals: &[f32; 3]) -> Self {
        // Create a Vec4 from a non-homogenous array.
        Self {x: vals[0], y: vals[1], z: vals[2], w: 1.}
    }

    fn to_array(self) -> [f32; 4] {
        // todo temp to keep compat with dot product during transition
        [self.x, self.y, self.z, self.w]
    }

    pub fn mul(&self, val: f32) -> Self {
    // Can't get operator overload working due to other not beign a Vec3.
        Vec4 {x: self.x * val, y: self.y * val, z: self.z * val, w: self.w * val}
    }

    pub fn _div(&self, val: f32) -> Self {
            Vec4 {x: self.x / val, y: self.y / val, z: self.z / val, w: self.w * val}
    }
}

impl Add for Vec4 {
    type Output = Self;

    fn add(self, other: Vec4) -> Vec4 {
        Vec4 {x: self.x + other.x, y: self.y + other.y, z: self.z + other.z, w: self.w + other.w}
    }
}

impl Sub for Vec4 {
    type Output = Self;

    fn sub(self, other: Vec4) -> Vec4 {
        Vec4 {x: self.x - other.x, y: self.y - other.y, z: self.z - other.z, w: self.w + other.w}
    }
}


#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    // Only used in meshes, for now.
    pub position: (f32, f32, f32),
}

impl Vertex {
    pub fn new(x: f32, y: f32, z: f32) -> Vertex {
        Vertex{ position: (x, y, z) }
    }

    pub fn subtract(&self, other: &Vertex) -> Vertex {
        Vertex::new(self.position.0 - other.position.0, self.position.1 - other.position.1,
                    self.position.2 - other.position.2)
    }

    pub fn cross(&self, other: &Vertex) -> Normal {
        Normal::new(
            self.position.1 * other.position.2 - self.position.2 * other.position.1,
            self.position.2 * other.position.0 - self.position.0 * other.position.2,
            self.position.0 * other.position.1 - self.position.1 * other.position.0,
        )
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Normal {
    // Only used in meshes, for now.
    pub normal: (f32, f32, f32)
}

impl Normal {
    pub fn new(x: f32, y: f32, z: f32) -> Normal {
        Normal{ normal: (x, y, z) }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct VertAndExtras {
    // Used to pass attributes that go with each vertex to the shader.
    // We do the impl_vertex in render_vulkano, so we don't need to import vulkano
    // in the wasm target.

    // Unlike in Vertex and Normal, use homogenous coordinates here.
    pub position: (f32, f32, f32, f32),
    pub normal: (f32, f32, f32, f32),
    pub specular_intensity: f32,
}

impl VertAndExtras {
    pub fn new(posit: Vertex, norm: Normal, specular_intensity: f32) -> VertAndExtras {
        // Helper function for making position and normal homogenous, and including
        // the shape's position in the vertex's.
        VertAndExtras {
            position: (posit.position.0, posit.position.1, posit.position.2, 1.),
            normal: (norm.normal.0, norm.normal.1, norm.normal.2, 1.),
            specular_intensity,
        }
    }
}

//impl_vertex!(Vertex, position);
//impl_vertex!(Normal, normal);
impl_vertex!(VertAndExtras, position, normal, specular_intensity);

#[derive(Clone, Debug)]
pub struct Mesh {
    pub vertices: HashMap<u32, Vertex>,
    pub faces_vert: Vec<Array1<u32>>,  // Indicies of vertexes.
    pub normals: Vec<Normal>,  // Normals only use the 3d component; not defined for 4d, yet. ?
    pub tris: Array1<u32>,
}

impl Mesh {
    pub fn new(vertices: HashMap<u32, Vertex>,
               faces_vert: Vec<Array1<u32>>, normals: Vec<Normal>) -> Mesh {

        let mut result = Mesh {vertices, faces_vert, normals, tris: array![]};
        result.make_tris();
        result
    }

    pub fn make_tris(&mut self) {
        // Divide faces into triangles of indices. These indices aren't of node
        // ids; rather of cumulative node ids; eg how they'll appear in an index buffer.
        // Result is a 1d array.
        // Important: Faces must be defined in an order of consecutive edges.
        // If we modify/add faces, we must re-run this.
        let mut result = Vec::new();
        let mut current_i = 0;

        for face in &self.faces_vert {
            match face.len() {
                3 => {
                    // Only one triangle.
                    result.push(current_i as u32);
                    result.push(current_i as u32 + 1);
                    result.push(current_i as u32 + 2);
                },
                4 => {
                    // First triangle
                    result.push(current_i as u32);
                    result.push(current_i as u32 + 1);
                    result.push(current_i as u32 + 2);
                    // Second triangle
                    result.push(current_i as u32);
                    result.push(current_i as u32 + 2);
                    result.push(current_i as u32 + 3);
                },
                2 => panic!("Faces must have length 3 or more."),
                _ => panic!("Error: Haven't implemented faces with vertex counds higher than four.")

            }
            current_i += face.len();
        }
        self.tris = Array::from_vec(result)
    }

    pub fn num_face_verts(&self) -> u32 {
        // Find the number of vertices used in drawing faces.  Ie for a box,
        // it's 6 faces x 4 vertices/face.
        self.faces_vert.iter().fold(0, |acc, face| acc + face.len() as u32)
    }

}


#[derive(Clone, Debug)]
pub struct Shape {
    // todo macro constructor that lets you ommit position, rotation, scale.
    // Shape nodes and rotation are relative to an origin of 0.
    pub mesh: Mesh,
    pub position: [f32; 3],
    pub scale: f32,
    pub orientation: [f32; 3],  // Orientation has 3 items.
    pub rotation_speed: [f32; 3],
    pub opacity: f32,
    pub specular_intensity: f32,
}

impl Shape {
    pub fn new(mesh: Mesh, position: [f32; 3], orientation: [f32; 3]) -> Shape {
        Shape{
            mesh,
            position,
            scale: 1.,
            orientation,
            rotation_speed: [0., 0., 0.],
            opacity: 1.,
            specular_intensity: 1. }
    }
}

#[derive(Clone, Debug)]
pub struct Camera {
    // Position shifts all points prior to the camera transform; this is what
    // we adjust with move keys.
    pub position: [f32; 3],
    pub θ: [f32; 3],

    pub fov: f32,  // Vertical field of view in radians.
    pub aspect: f32,  // width / height.
    // near, far, and strange for our 3d and 4d frustrums.  Strange is an
    // experimental extension into the 4th dimension.
    pub near: f32,
    pub far: f32,
}

impl Camera {
    pub fn view_size(&self, far: bool) -> (f32, f32){
        // Calculate the projected window width and height, using basic trig.
        let dist = if far { self.far } else { self.near };

        let width = 2. * dist * (self.fov * self.aspect / 2.).tan();
        let height = 2. * dist * (self.fov / 2.).tan();
        (width, height)
    }
}


#[derive(Clone, Debug)]
pub enum CameraType {
    Single,  // No camera changes; rotate the shape instead
    // Move foward, back, left, right, and look around. No roll look.  Not sure
    // which 4d rotations/movement to allow or block.
    FPS,
    Free, // No restriction on movement
}

#[derive(Clone, Debug)]
pub struct LightSource {
    // A point light source
    pub position: [f32; 3],
    pub color: [f32; 4],
    pub intensity: f32,
}

#[derive(Clone, Debug)]
pub struct Lighting {
    pub ambient_intensity: f32,
    pub diffuse_intensity: f32,
    pub ambient_color: [f32; 4],
    pub diffuse_color: [f32; 4],
    // Direction doesn't have to be normalized; we do that in the shader.
    pub diffuse_direction: [f32; 3],
    pub sources: Vec<LightSource>,
}

#[derive(Clone, Debug)]
pub struct Scene {
    pub shapes: HashMap<u32, Shape>,
    pub cam: Camera,
    pub cam_type: CameraType,
    pub lighting: Lighting,
    pub sensitivities: (f32, f32, f32),  // move, rotate, zoom
}
