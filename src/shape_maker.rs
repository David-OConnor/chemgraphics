use std::collections::HashMap;
//use std::f32::consts::PI;

use ndarray::prelude::*;

use transforms;
use types::{Vertex, Mesh, Normal, Shape};

fn add(left: &Vec<u32>, val: u32) -> Vec<u32> {
    left.iter().map(|item| item + val).collect()
}

//const τ: f32 = 2. * PI;

// We'll define y as vertical, and z as forward/back.  All shapes are given
// four coordinates. Leave

// Nodes are set up here so that 0 is at their center; this is used for scaling,
// rotation, and positioning in the world.

pub fn make_normals(vertices: &HashMap<u32, Vertex>, faces: &Vec<Array1<u32>>) -> Vec<Normal> {
    let mut normals = Vec::new();
    for face in faces {
        // todo make sure these aren't reversed!
        let line1 = vertices[&face[1]].subtract(&vertices[&face[0]]);
        let line2 = vertices[&face[2]].subtract(&vertices[&face[0]]);
        normals.push(line1.cross(&line2));
    }

    normals
}


pub fn _combine_meshes(mut base: Mesh, meshes: Vec<(Mesh, [f32; 3])>) -> Mesh{
    // The array in the meshes tuple is position offset for that shape.
    let mut id_addition = base.vertices.len() as u32;
    for (mesh, offset) in &meshes {
        for (id, vertex) in &mesh.vertices {
            // For the roof, modify the ids to be unique.
            base.vertices.insert(
                id + id_addition,
                Vertex::new(vertex.position[0] + offset[0], vertex.position[1] + offset[1],
                            vertex.position[2] + offset[2]
                )
            );
        }

        for face in &mesh.faces_vert {
            base.faces_vert.push(add(face, id_addition));
        }

        for normal in &mesh.normals {  // todo rotate normals!
            base.normals.push(normal.clone());
        }

        id_addition += mesh.vertices.len() as u32;
    }

    base.make_tris();
    base
}

pub fn box_(lens: (f32, f32, f32)) -> Mesh {
    // Make a rectangular prism.  Use negative lengths to draw in the opposite
    // direction.

    let coords = [
        // Front
        [-1., -1., -1.],
        [1., -1., -1.],
        [1., 1., -1.],
        [-1., 1., -1.],

        // Back
        [-1., -1., 1.],
        [1., -1., 1.],
        [1., 1., 1.],
        [-1., 1., 1.],
    ];

    let mut vertices = HashMap::new();
    for (id, coord) in coords.iter().enumerate() {
        vertices.insert(id as u32, Vertex::new(
            coord[0] * lens.0 / 2., coord[1] * lens.1 / 2.,
            coord[2] * lens.2 / 2.
        ));
    }

    let faces_vert = vec![  // Vertex indices for each face.
        vec![0, 1, 2, 3],  // Front
        vec![4, 5, 6, 7],  // Back
        vec![3, 2, 6, 7],  // Top
        vec![0, 1, 5, 4],  // Bottom
        vec![0, 4, 7, 3],  // Left
        vec![1, 5, 6, 2],  // Right
    ];

    let face_colors = vec![
       [1., 1., 0., 1.],  // Front
       [0., 1., 1., 1.],  // Back
       [1., 0., 0., 1.],  // Top
       [0., 0., 1., 1.],  // Bottom
       [1., 0., 1., 1.],  // Left
       [0., 1., 0., 1.],  // Right
    ];

    //  Normals correspond to faces.
    let normals = vec![
        Normal::new(0., 0., -1.),
        Normal::new(0., 0., 1.),
        Normal::new(0., 1., 0.),
        Normal::new(0., -1., 0.),
        Normal::new(-1., 0., 0.),
        Normal::new(1., 0., 0.),
    ];

    Mesh::new(vertices, faces_vert, face_colors, normals)
}

pub fn _rect_pyramid(lens: (f32, f32, f32)) -> Mesh {
    let coords = [
        // Base  (Center of this shape is the center of the base square)
        [-1., 0., -1.],
        [1., 0., -1.],
        [1., 0., 1.],
        [-1., 0., 1.],

        // Top
        [0., 1., 0.],
    ];

    let mut vertices = HashMap::new();
    for (id, coord) in coords.iter().enumerate() {
        vertices.insert(id as u32, Vertex::new(
            coord[0] / 2. * lens.0, coord[1] / 2. * lens.1,
            coord[2] / 2. * lens.2
        ));
    }

    let faces_vert = vec![  // Vertex indices for each face.
        vec![0, 1, 2, 3],  // Base
        vec![0, 1, 4],  // Front
        vec![1, 2, 4],  // Right
        vec![2, 3, 4],  // Back
        vec![3, 0, 4],  // Left
    ];

    let face_colors = vec![
        [1., 1., 0., 1.],
        [0., 1., 1., 1.],
        [1., 0., 0., 1.],
        [0., 0., 1., 1.],
        [1., 0., 1., 1.],

    ];

    // Normals correspond to faces.
    // Note that these don't need to be normalized here; the shader will do it.
    let normals = vec![
        Normal::new(0., -1., 0.),
        Normal::new(0., lens.2, -lens.1),
        Normal::new(-lens.2, lens.1, 0.),
        Normal::new(0., lens.2, lens.1),
        Normal::new(lens.2, lens.1, 0.),
    ];

    Mesh::new(vertices, faces_vert, face_colors, normals)
}

pub fn _house(lens: (f32, f32, f32)) -> Mesh {
    // We'll modify base in-place, then return it.
    let base = box_(lens);

    let roof = _rect_pyramid(
        // Let the roof overhang the base by a little.
        // Make the roof height a portion of the base height.
        (lens.0 * 1.2, lens.1 / 3., lens.2 * 1.2),
    );

    _combine_meshes(base, vec![(roof, [0., lens.1 / 2., 0.])])
}

pub fn cube(side_len: f32) -> Mesh {
    // Convenience function.
    // We'll still treat the center as the center of the base portion.
    box_((side_len, side_len, side_len))
}

fn avg_normals(normals: Vec<Normal>) -> Normal {
    let x = normals.iter().fold(0., |acc, norm| acc + norm.normal[0]);
    let y = normals.iter().fold(0., |acc, norm| acc + norm.normal[1]);
    let z = normals.iter().fold(0., |acc, norm| acc + norm.normal[2]);

    let len = normals.len() as f32;
    Normal::new(x/len , y/len, z/len)
}

#[cfg(test)]
mod tests {
    use super::*;

}