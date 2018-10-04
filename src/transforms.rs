use types::{Camera, Vec4};

pub fn dot_mv4(M: [[f32; 4]; 4], v: Vec4) -> Vec4 {
    // Dot a len-4 matrix with a vec.
    Vec4::new(
        v.x*M[0][0] + v.y*M[0][1] + v.z*M[0][2] + v.w*M[0][3],
        v.x*M[1][0] + v.y*M[1][1] + v.z*M[1][2] + v.w*M[1][3],
        v.x*M[2][0] + v.y*M[2][1] + v.z*M[2][2] + v.w*M[2][3],
        v.x*M[3][0] + v.y*M[3][1] + v.z*M[3][2] + v.w*M[3][3]
    )
}

pub fn dot_mm4(M0: [[f32; 4]; 4], M1: [[f32; 4]; 4]) -> [[f32; 4]; 4] {
    // Dot a len-4 matrix with another matrix.
    [
        // Row 0
        [M0[0][0]*M1[0][0] + M0[0][1]*M1[1][0] + M0[0][2]*M1[2][0] + M0[0][3]*M1[3][0],
        M0[0][0]*M1[0][1] + M0[0][1]*M1[1][1] + M0[0][2]*M1[2][1] + M0[0][3]*M1[3][1],
        M0[0][0]*M1[0][2] + M0[0][1]*M1[1][2] + M0[0][2]*M1[2][2] + M0[0][3]*M1[3][2],
        M0[0][0]*M1[0][3] + M0[0][1]*M1[1][3] + M0[0][2]*M1[2][3] + M0[0][3]*M1[3][3]],

        // Row 1
        [M0[1][0]*M1[0][0] + M0[1][1]*M1[1][0] + M0[1][2]*M1[2][0] + M0[1][3]*M1[3][0],
        M0[1][0]*M1[0][1] + M0[1][1]*M1[1][1] + M0[1][2]*M1[2][1] + M0[1][3]*M1[3][1],
        M0[1][0]*M1[0][2] + M0[1][1]*M1[1][2] + M0[1][2]*M1[2][2] + M0[1][3]*M1[3][2],
        M0[1][0]*M1[0][3] + M0[1][1]*M1[1][3] + M0[1][2]*M1[2][3] + M0[1][3]*M1[3][3]],

        // Row 2
        [M0[2][0]*M1[0][0] + M0[2][1]*M1[1][0] + M0[2][2]*M1[2][0] + M0[2][3]*M1[3][0],
        M0[2][0]*M1[0][1] + M0[2][1]*M1[1][1] + M0[2][2]*M1[2][1] + M0[2][3]*M1[3][1],
        M0[2][0]*M1[0][2] + M0[2][1]*M1[1][2] + M0[2][2]*M1[2][2] + M0[2][3]*M1[3][2],
        M0[2][0]*M1[0][3] + M0[2][1]*M1[1][3] + M0[2][2]*M1[2][3] + M0[2][3]*M1[3][3]],

        // Row 3
        [M0[3][0]*M1[0][0] + M0[3][1]*M1[1][0] + M0[3][2]*M1[2][0] + M0[3][3]*M1[3][0],
        M0[3][0]*M1[0][1] + M0[3][1]*M1[1][1] + M0[3][2]*M1[2][1] + M0[3][3]*M1[3][1],
        M0[3][0]*M1[0][2] + M0[3][1]*M1[1][2] + M0[3][2]*M1[2][2] + M0[3][3]*M1[3][2],
        M0[3][0]*M1[0][3] + M0[3][1]*M1[1][3] + M0[3][2]*M1[2][3] + M0[3][3]*M1[3][3]],
    ]
}

pub fn I4() -> [[f32; 4]; 4] {
    [
        [1., 0., 0., 0.],
        [0., 1., 0., 0.],
        [0., 0., 1., 0.],
        [0., 0., 0., 1.],
    ]
}

pub fn rotate(θ: &Vec4) -> [[f32; 4]; 4] {
    // Homogenous rotation matrix.
    // todo quaternions??
    let cos_x = θ.x.cos();
    let sin_x = θ.x.sin();
    let cos_y = θ.y.cos();
    let sin_y = θ.y.sin();
    let cos_z = θ.z.cos();
    let sin_z = θ.z.sin();

    let R_x = [
        [1., 0., 0., 0.],
        [0., cos_x, -sin_x, 0.,],
        [0., sin_x, cos_x, 0.,],
        [0., 0., 0., 1.],
    ];

    let R_y = [
        [cos_y, 0., sin_y, 0.],
        [0., 1., 0., 0.],
        [-sin_y, 0., cos_y, 0.],
        [0., 0., 0., 1.],
    ];

    let R_z = [
        [cos_z, -sin_z, 0., 0.],
        [sin_z, cos_z, 0., 0.],
        [0., 0., 1., 0.],
        [0., 0., 0., 1.],
    ];

    dot_mm4(R_x, dot_mm4(R_y, R_z))
}

fn translate(position: &Vec4) -> [[f32; 4]; 4] {
    // Return a homogenous translation matrix.
    [
        [1., 0., 0., position.x],
        [0., 1., 0., position.y],
        [0., 0., 1., position.z],
        [0., 0., 0., 1.],
    ]
}

fn scale(val: f32) -> [[f32; 4]; 4] {
    // Return a homogenous scale matrix.
    [
        [val, 0., 0., 0.],
        [0., val, 0., 0.],
        [0., 0., val, 0.],
        [0., 0., 0., 1.],
    ]
}


pub fn proj(cam: &Camera) -> [[f32; 4]; 4] {
    // Return a homogenous matrix of the type used by vulkan.

    // Apply this transform:
    // https://matthewwellings.com/blog/the-new-vulkan-coordinate-system/
    // To the matrix here:
    // http://www.songho.ca/opengl/gl_projectionmatrix.html

    let t = (cam.fov / 2.).tan() * cam.near;
    let b = -t;
    let r = t * cam.aspect;
    let l = -t * cam.aspect;
    let n = cam.near;
    let f = cam.far;

    [
        [2.*n / (r - l), 0., (r+l) / (2.*(r-l)), (r+l) / (2.*(r-l))],
        [0., -2.*n / (t-b), (t+b) / (2.*(t-b)), (t+b) / (2.*(t-b))],
        [0., 0., -(f+n) / (2.*(f-n)), -(2.*f*n) / (f-n) + -(f+n) / (2.*(f-n))],
        [0., 0., -0.5, -0.5],
    ]
}


pub fn model(position: &Vec4, orientation: &Vec4,
        scale_val: f32) -> [[f32; 4]; 4] {
    // Return a model matrix that transforms, rotates, and scales, using homogenous
    // coordinates.  Position first.
    let T = translate(position);
    let R = rotate(orientation);
    let S = scale(scale_val);
    
    dot_mm4(T, dot_mm4(R, S))
}

pub fn view(position: &Vec4, θ: &Vec4) -> [[f32; 4]; 4] {
    // Homogenous view matrix.  Position last.
    let T = translate(position);
    let R = rotate(θ);

    dot_mm4(R, T)
}


#[cfg(test)]
mod tests {
    use super::*;
}
