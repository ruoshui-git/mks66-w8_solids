use crate::graphics::{utils::mapper, matrix::Matrix};

// https://developer.mozilla.org/en-US/docs/Web/API/WebGL_API/WebGL_model_view_projection#Perspective_matrix

/// Construct a perspective projection matrix based
/// ## Arguments:
///        fov_rad - Field of view - the angle in radians of what's in view along the Y axis
///        aspect - Aspect Ratio - the ratio of the canvas, typically width / height
///        near - Anything before this point in the Z direction gets clipped (outside of the clip space)
///        far - Anything after this point in the Z direction gets clipped (outside of the clip space)
///
#[rustfmt::skip]
pub fn perspective(fov_rad: f64, aspect: f64, near: f64, far: f64) -> Matrix {

    todo!("According to https://en.wikipedia.org/wiki/Graphics_pipeline#Projection, this projection matrix is the transposed version of what I should be using. Fix it.");

    todo!("Impl clipping");

    let f = 1. / (fov_rad / 2.).tan();
    let range_inv = 1. / (near - far);
    Matrix::new(4, 4, vec![
        f / aspect, 0., 0.,                             0.,
        0.,         f,  0.,                             0.,
        0.,         0., (near + far) * range_inv,       -1.,
        0.,         0., near * far * range_inv * 2.,    0.,
    ])

}

/// Construct an orthographic projection matrix
#[rustfmt::skip]
pub fn orthographic(left: f64, right: f64, bottom: f64, top: f64, near: f64, far: f64) -> Matrix {
    // Each of the parameters represents the plane of the bounding box
    let lr = 1. / (left - right);
    let bt = 1. / (bottom - top);
    let nf = 1. / (near - far);

    let row4col1 = (left + right) * lr;
    let row4col2 = (top + bottom) * bt;
    let row4col3 = (far + near) * nf;
    Matrix::new(4, 4, vec![
        -2. * lr,         0.,        0., 0.,
              0.,   -2. * bt,        0., 0.,
              0.,         0.,   2. * nf, 0.,
        row4col1,   row4col2,  row4col3, 1.,
    ])
}




impl Matrix {

    /// Convert matrix in ndc coordinates to device coordinates
    ///
    /// This should be used only after perspective divide and before rendered onto the canvas
    pub fn ndc_n1to1_to_device(&mut self, width: f64, height: f64) {
        let map_width = mapper(-1., 1., 0., width);
        let map_height = mapper(-1., 1., 0., height);

        for row in self.mut_iter_by_row() {
            row[0] = map_width(-row[0]);
            row[1] = map_height(row[1]);
        }

    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graphics::{
        utils::display_edge_matrix,
        matrix::transform
    };

    #[test]
    fn test_perspective() {
        let mut model = Matrix::new_edge_matrix();
        model.add_sphere((110., 0., 0.), 75.);
        model.add_sphere((-100., 0., 0.), 75.);
        model.add_box((-80., -120., 0.), 75., 75., 75.);
        model.add_torus((-30., -335., 0.), 25., 175.);
        let t = Matrix::ident(4)
        // .mul(&transform::rotatex(30.))
        // .mul(&transform::rotatey(-20.))
        ._mul(&transform::mv(0., 250., 250.)
        
    );
        let model = model._mul(&t);

        // now apply perspective
        let mut model = model._mul(&perspective(90., 1., 1., 500.));
        model.perspective_divide();

        display_edge_matrix(&model, true);
    }
}