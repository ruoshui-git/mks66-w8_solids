use super::Matrix;
///! Note: all matrices here are row-major (transposed compared to what we have from classes)
///! since my engine uses row-major point matrices

// generate transformation matrices
/// Generate a translation matrix with (dx, dy, dz)
pub fn mv(dx: f64, dy: f64, dz: f64) -> Matrix {
    let mut m = Matrix::ident(4);

    m.set(3, 0, dx);
    m.set(3, 1, dy);
    m.set(3, 2, dz);
    m
}

/// Generate a scale matrix with (sx, sy, sz)
pub fn scale(sx: f64, sy: f64, sz: f64) -> Matrix {
    let mut m = Matrix::ident(4);
    m.set(0, 0, sx);
    m.set(1, 1, sy);
    m.set(2, 2, sz);
    m
}

#[rustfmt::skip]
    pub fn rotatex(angle_deg: f64) -> Matrix {
        // let mut m = Matrix::ident(4);
        // m.set(1, 1, angle_deg.to_radians().cos());
        // m.set(2, 2, angle_deg.to_radians().cos());
        // m.set(1, 2, -angle_deg.to_radians().sin());
        // m.set(2, 1, angle_deg.to_radians().sin());
        // m
        let a = angle_deg.to_radians();
        Matrix::new(
            4,
            4,
            vec![ 
                1.0,        0.0,        0.0,      0.0, 
                0.0,        a.cos(),    a.sin(), 0.0, 
                0.0,        -a.sin(),    a.cos(),  0.0, 
                0.0,        0.0,        0.0,      1.0,
            ],
        )
    }

#[rustfmt::skip]
    pub fn rotatey(angle_deg: f64) -> Matrix {
        // let mut m = Matrix::ident(4);
        // m.set(0, 0, angle_deg.to_radians().cos());
        // m.set(0, 2, angle_deg.to_radians().sin());
        // m.set(2, 0, -angle_deg.to_radians().sin());
        // m.set(2, 2, angle_deg.to_radians().cos());
        // m
        let a = angle_deg.to_radians();
        Matrix::new(
            4,
            4,
            vec![ 
                a.cos(),    0.0, -a.sin(),  0.0, 
                0.0,        1.0, 0.0,       0.0, 
                a.sin(),    0.0, a.cos(),   0.0, 
                0.0,        0.0, 0.0,       1.0,
            ],
        )
    }

#[rustfmt::skip]
pub fn rotatez(angle_deg: f64) -> Matrix {
    let a = angle_deg.to_radians();
    Matrix::new(4, 4, vec![
        a.cos(),    a.sin(),   0., 0.,
        -a.sin(),    a.cos(),   0., 0.,
        0.,         0.,         1., 0.,
        0.,         0.,         0., 1.,
    ])
    // m.set(0, 0, angle_deg.to_radians().cos());
    // m.set(1, 1, angle_deg.to_radians().cos());
    // m.set(1, 0, angle_deg.to_radians().sin());
    // m.set(0, 1, -angle_deg.to_radians().sin());
}

impl Matrix {
    /// Correct edges after projection by dividing all values of point by w
    pub fn perspective_divide(&mut self) {
        for point in self.mut_iter_by_row() {
            let (x, y, z, w) = (point[0], point[1], point[2], point[3]);
            point[0] = x / w;
            point[1] = y / w;
            point[2] = z / w;
            point[3] = 1.;
        }
    }
}
