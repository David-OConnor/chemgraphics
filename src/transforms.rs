use types::{Camera, Vec4};

pub fn transpose(M: [[f32; 4]; 4]) -> [[f32; 4]; 4] {
    [
        [M[0][0], M[1][0], M[2][0], M[3][0]],
        [M[0][1], M[1][1], M[2][1], M[3][1]],
        [M[0][2], M[1][2], M[2][2], M[3][2]],
        [M[0][3], M[1][3], M[2][3], M[3][3]],
    ]
}

pub fn _dot_mv4(M: [[f32; 4]; 4], v: Vec4) -> Vec4 {
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
        [
            M0[0][0]*M1[0][0] + M0[0][1]*M1[1][0] + M0[0][2]*M1[2][0] + M0[0][3]*M1[3][0],
            M0[0][0]*M1[0][1] + M0[0][1]*M1[1][1] + M0[0][2]*M1[2][1] + M0[0][3]*M1[3][1],
            M0[0][0]*M1[0][2] + M0[0][1]*M1[1][2] + M0[0][2]*M1[2][2] + M0[0][3]*M1[3][2],
            M0[0][0]*M1[0][3] + M0[0][1]*M1[1][3] + M0[0][2]*M1[2][3] + M0[0][3]*M1[3][3]
        ],

        // Row 1
        [
            M0[1][0]*M1[0][0] + M0[1][1]*M1[1][0] + M0[1][2]*M1[2][0] + M0[1][3]*M1[3][0],
            M0[1][0]*M1[0][1] + M0[1][1]*M1[1][1] + M0[1][2]*M1[2][1] + M0[1][3]*M1[3][1],
            M0[1][0]*M1[0][2] + M0[1][1]*M1[1][2] + M0[1][2]*M1[2][2] + M0[1][3]*M1[3][2],
            M0[1][0]*M1[0][3] + M0[1][1]*M1[1][3] + M0[1][2]*M1[2][3] + M0[1][3]*M1[3][3]
        ],

        // Row 2
        [
            M0[2][0]*M1[0][0] + M0[2][1]*M1[1][0] + M0[2][2]*M1[2][0] + M0[2][3]*M1[3][0],
            M0[2][0]*M1[0][1] + M0[2][1]*M1[1][1] + M0[2][2]*M1[2][1] + M0[2][3]*M1[3][1],
            M0[2][0]*M1[0][2] + M0[2][1]*M1[1][2] + M0[2][2]*M1[2][2] + M0[2][3]*M1[3][2],
            M0[2][0]*M1[0][3] + M0[2][1]*M1[1][3] + M0[2][2]*M1[2][3] + M0[2][3]*M1[3][3]
        ],

        // Row 3
        [
            M0[3][0]*M1[0][0] + M0[3][1]*M1[1][0] + M0[3][2]*M1[2][0] + M0[3][3]*M1[3][0],
            M0[3][0]*M1[0][1] + M0[3][1]*M1[1][1] + M0[3][2]*M1[2][1] + M0[3][3]*M1[3][1],
            M0[3][0]*M1[0][2] + M0[3][1]*M1[1][2] + M0[3][2]*M1[2][2] + M0[3][3]*M1[3][2],
            M0[3][0]*M1[0][3] + M0[3][1]*M1[1][3] + M0[3][2]*M1[2][3] + M0[3][3]*M1[3][3]
        ],
    ]
}

pub fn _I4() -> [[f32; 4]; 4] {
    [
        [1., 0., 0., 0.],
        [0., 1., 0., 0.],
        [0., 0., 1., 0.],
        [0., 0., 0., 1.],
    ]
}

pub fn rotate(θ: &[f32; 3]) -> [[f32; 4]; 4] {
    // Homogenous rotation matrix.
    // todo quaternions??
    let cos_x = θ[0].cos();
    let sin_x = θ[0].sin();
    let cos_y = θ[1].cos();
    let sin_y = θ[1].sin();
    let cos_z = θ[2].cos();
    let sin_z = θ[2].sin();

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

    dot_mm4(R_z, dot_mm4(R_y, R_x))
}

fn translate(position: &[f32; 3]) -> [[f32; 4]; 4] {
    // Return a homogenous translation matrix.
    [
        [1., 0., 0., position[0]],
        [0., 1., 0., position[1]],
        [0., 0., 1., position[2]],
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


//    [
//        [2.*n / (r - l), 0., (r+l) / (2.*(r-l)), (r+l) / (2.*(r-l))],
//        [0., -2.*n / (t-b), (t+b) / (2.*(t-b)), (t+b) / (2.*(t-b))],
//        [0., 0., -(f+n) / (2.*(f-n)), -(2.*f*n) / (f-n) + -(f+n) / (2.*(f-n))],
//        [0., 0., -0.5, -0.5],
//    ]
////    // todo test
//    let r = -l;
//    let t = -b;
//
//
//    [
//        [1./r, 0., 0., 0.],
//        [0., 1./t, 0., 0.],
//        [0., 0., -2./(f-n), -(f+n) / (f-n)],
//        [0., 0., 0., 1.],
//    ]

    [
        [2.*n / (r - l), 0., (r+l) / (r-l) / 2., 0.],
        [0., -2.*n / (t-b), (t+b) / (t-b) / 2., (b+t) / (t-b) / 2.],
        [0., 0., -(f+n) / (f-n) / 2., -(2.*f*n) / (f-n) + (-f-n) / (f-n) / 2.],
        // u_scale is, ultimately, not really used.
        // This row allows us to divide by z after taking the dot product,
        // as part of our scaling operation.
        [0., 0., 0.5, 1.],  //
    ]
}

pub fn model(position: &[f32; 3], orientation: &[f32; 3], scale_val: f32) -> [[f32; 4]; 4] {
    // Return a model matrix that transforms, rotates, and scales, using homogenous
    // coordinates.  Position last
    let R = transpose(rotate(orientation));
    let S = transpose(scale(scale_val));
    let T = transpose(translate(position));

    dot_mm4(T, dot_mm4(R, S))
}

pub fn view(position: &[f32; 3], θ: &[f32; 3]) -> [[f32; 4]; 4] {
    // Homogenous view matrix.  Position first
    let inv_posit = [-position[0], -position[1], -position[2]];

    let T = transpose(translate(&inv_posit));
    let R = transpose(rotate(θ));

    dot_mm4(R, T)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dot_A() {

        let a = [
            [1., 2., 3., 4.],
            [4., 2., 1., -1.],
            [0., -4., 1., 3.],
            [9., 1., -5., 1.],
        ];

        let b = [
            [-1., 7., -2., 4.],
            [4., 2., 1., -1.],
            [0., -4., 1., 1.],
            [4., 1., 10., -2.],
        ];

        let expected = [
            [23., 3., 43., -3.],
            [0., 27., -15., 17.],
            [-4., -9., 27., -1.],
            [-1., 86., -12., 28.],
        ];

        assert_eq!(dot_mm4(a, b), expected);
    }
}
