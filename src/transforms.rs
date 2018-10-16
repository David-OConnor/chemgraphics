use ops::{dot, transpose, mul_arr};
use types::{Camera, Vec4};


pub fn _dot_mv4(M: [[f32; 4]; 4], v: Vec4) -> Vec4 {
    // Dot a len-4 matrix with a vec.
    Vec4::new(
        v.x*M[0][0] + v.y*M[0][1] + v.z*M[0][2] + v.w*M[0][3],
        v.x*M[1][0] + v.y*M[1][1] + v.z*M[1][2] + v.w*M[1][3],
        v.x*M[2][0] + v.y*M[2][1] + v.z*M[2][2] + v.w*M[2][3],
        v.x*M[3][0] + v.y*M[3][1] + v.z*M[3][2] + v.w*M[3][3]
    )
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


//    dot(transpose(R_z), dot(transpose(R_y), transpose(R_x)))
    dot(R_z, dot(R_y, R_x))
}

pub fn translate(position: &[f32; 3]) -> [[f32; 4]; 4] {
    // Return a homogenous translation matrix.
    transpose([
        [1., 0., 0., position[0]],
        [0., 1., 0., position[1]],
        [0., 0., 1., position[2]],
        [0., 0., 0., 1.],
    ])
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

    // x and y map from -1 to +1, left to right, and top to bottom respectively.
    // z maps from 0 to 1, back to front.

    // This seems to be the (only?) Reasonable OpenGL-style projection matrix I've
    // found online... But it's missing the changes Vulkan introduces.
    // https://matthewwellings.com/blog/the-new-vulkan-coordinate-system/

    // Ref drawing in CG notebook. It's remarkably difficult to find reliable and clear
    // info on this online.

    let a = 1. / (cam.fov / 2.).tan();
    let n = cam.near;  // n and f are code-shorteners.
    let f = cam.far;

//    [
//        [a / cam.aspect, 0., 0., 0.],
//        [0., -a, 0., 0.],
////        [0., 0., f / (f-n), f*n / (f-n)],
//        // todo temporary linear mapping
//        [0., 0., (f + n)/(f-n), -2.*f*n / (f-n)],
//        [0., 0., 1., 0.]  // Preserve z, since we need to divide x and y by it after matrix multiplication.
//    ]
//

    [
        [a / cam.aspect, 0., 0., 0.],
        [0., -a, 0., 0.],
        [0., 0., f / (f-n), -f*n/(f-n)],
        // u_scale is, ultimately, not really used.
        // This row allows us to divide by z after taking the dot product,
        // as part of our scaling operation.
        [0., 0., 1., 0.],  //
    ]
}

pub fn model(position: &[f32; 3], orientation: &[f32; 3], scale_val: f32) -> [[f32; 4]; 4] {
    // Return a model matrix that transforms, rotates, and scales.  Position last
    let R = rotate(orientation);
    let S = scale(scale_val);
    let T = translate(position);

    dot(T, dot(R, S))
}

pub fn view(position: &[f32; 3], θ: &[f32; 3]) -> [[f32; 4]; 4] {
    // Homogenous view matrix.  Position first
    let inv_posit = mul_arr(position, -1.);
    let inv_θ = mul_arr(θ, -1.);

    let T = translate(&inv_posit);
    let R = rotate(&inv_θ);

    dot(R, T)
}


#[cfg(test)]
mod tests {
    use super::*;

    use std::f32::consts::PI;
    use ops::{dot_v, div_arr4};
    use types::Camera;

    // These transforms assume a vulkan coordinate system: Y points down,
    // Z points away. X and Y Range from -1 to 1. Z ranges from 0 to 1.

    const ϵ: f32 = 1e-7;
    const τ: f32 = 2. * PI;

    fn arr_close(arr1: [f32; 4], arr2: [f32; 4]) -> bool {
        (arr1[0] - arr2[0]).abs() < ϵ &&
        (arr1[1] - arr2[1]).abs() < ϵ &&
        (arr1[2] - arr2[2]).abs() < ϵ &&
        (arr1[3] - arr2[3]).abs() < ϵ
    }

    #[test]
    fn proj_corners() {
        // Ensure points on the corners of the frustrum map to corners on the
        // clipspace (projected) cuboid. This doesn't ensure the projection matrix
        // is correct; but can indicate a bogus one.
        let cam = Camera {
            position: [0., 0., 0.],
            θ: [0., 0., 0.],
            fov: τ / 4.,
            aspect: 1.,
            near: 1.,
            far: 100.
        };

        let P = proj(&cam);
        let wf = 200.;  // Width of the FOV at the far end. Also height.
        let wn = 2.;  // ... at the near end.

        // These four corners are for the top of the frustum.
        let fwd_left = [-wf / 2., wf / 2., cam.far, 1.];
        let fwd_right = [wf / 2., wf / 2., cam.far, 1.];
        let aft_left = [-wn / 2., wn / 2., cam.near, 1.];
        let aft_right = [wn / 2., wn / 2., cam.near, 1.];

        // These are equivlants for the bottom.
        let fwd_left_btm = [-wf / 2., -wf / 2., cam.far, 1.];
        let fwd_right_btm = [wf / 2., -wf / 2., cam.far, 1.];
        let aft_left_btm = [-wn / 2., -wn / 2., cam.near, 1.];
        let aft_right_btm = [wn / 2., -wn / 2., cam.near, 1.];

        let fl = dot_v(&P, fwd_left);
        let fr = dot_v(&P, fwd_right);
        let al = dot_v(&P, aft_left);
        let ar = dot_v(&P, aft_right);

        let fl_btm = dot_v(&P, fwd_left_btm);
        let fr_btm = dot_v(&P, fwd_right_btm);
        let al_btm = dot_v(&P, aft_left_btm);
        let ar_btm = dot_v(&P, aft_right_btm);


        // We divide by W, which is a the (unmodified) z component.
        assert!(arr_close(div_arr4(&fl, fl[3]), [-1., -1., 1., 1.]));
        assert!(arr_close(div_arr4(&fr, fr[3]), [1., -1., 1., 1.]));
        assert!(arr_close(div_arr4(&al, al[3]), [-1., -1., 0., 1.]));
        assert!(arr_close(div_arr4(&ar, ar[3]), [1., -1., 0., 1.]));

        assert!(arr_close(div_arr4(&fl_btm, fl_btm[3]), [-1., 1., 1., 1.]));
        assert!(arr_close(div_arr4(&fr_btm, fr_btm[3]), [1., 1., 1., 1.]));
        assert!(arr_close(div_arr4(&al_btm, al_btm[3]), [-1., 1., 0., 1.]));
        assert!(arr_close(div_arr4(&ar_btm, ar_btm[3]), [1., 1., 0., 1.]));
    }

//    #[test]
//    fn depth_buffer() {
//        // Make sure the nonlinear depth of the projection matrix works.
//
//    }
}
