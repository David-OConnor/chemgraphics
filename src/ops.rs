use num;

// Operators; For manipulating vectors and matrices that use built-in formats like array.
// Size-specific for maximum performance.

pub fn add_arr(arr1: &[f32; 3], arr2: &[f32; 3]) -> [f32; 3] {
    [arr1[0] + arr2[0], arr1[1] + arr2[1], arr1[2] + arr2[2]]
}

pub fn mul_arr(arr: &[f32; 3], val: f32) -> [f32; 3] {
    [arr[0] * val, arr[1] * val, arr[2] * val]
}

pub fn mul_arr4(arr: &[f32; 4], val: f32) -> [f32; 4] {
    [arr[0] * val, arr[1] * val, arr[2] * val, arr[3] * val]
}

pub fn div_arr4(arr: &[f32; 4], val: f32) -> [f32; 4] {
    [arr[0] / val, arr[1] / val, arr[2] / val, arr[3] / val]
}

pub fn transpose(M: [[f32; 4]; 4]) -> [[f32; 4]; 4] {
    [
        [M[0][0], M[1][0], M[2][0], M[3][0]],
        [M[0][1], M[1][1], M[2][1], M[3][1]],
        [M[0][2], M[1][2], M[2][2], M[3][2]],
        [M[0][3], M[1][3], M[2][3], M[3][3]],
    ]
}

pub fn flatten<F: num::Float>(M: [[F; 4]; 4]) -> [F; 16] {
    [
        M[0][0], M[0][1], M[0][2], M[0][3],
        M[1][0], M[1][1], M[1][2], M[1][3],
        M[2][0], M[2][1], M[2][2], M[2][3],
        M[3][0], M[3][1], M[3][2], M[3][3],
    ]
}

pub fn dot(M0: [[f32; 4]; 4], M1: [[f32; 4]; 4]) -> [[f32; 4]; 4] {
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

pub fn dot_v(M: &[[f32; 4]; 4], v: [f32; 4]) -> [f32; 4] {
    // Dot a len-4 matrix with a vec.
    [
        v[0]*M[0][0] + v[1]*M[0][1] + v[2]*M[0][2] + v[3]*M[0][3],
        v[0]*M[1][0] + v[1]*M[1][1] + v[2]*M[1][2] + v[3]*M[1][3],
        v[0]*M[2][0] + v[1]*M[2][1] + v[2]*M[2][2] + v[3]*M[2][3],
        v[0]*M[3][0] + v[1]*M[3][1] + v[2]*M[3][2] + v[3]*M[3][3]
    ]
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

        assert_eq!(dot(a, b), expected);
    }
}