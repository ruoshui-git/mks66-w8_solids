
use super::Matrix;

impl Matrix {
    
    /// Make a new edge matrix for drawing on 2d surfaces
    pub fn new_edge_matrix() -> Matrix {
        Matrix {
            nrows: 0,
            ncols: 4,
            data: vec![],
        }
    }

    /// Append an edge in the format [x0, y0, z0, x1, y1, z1]
    pub fn append_edge(&mut self, edge: &[f64]) {
        assert_eq!(6, edge.len(), "Len of edge vec should be 6");
        self.data.extend_from_slice(&edge[0..3]);
        self.data.push(1.0);
        self.data.extend_from_slice(&edge[3..6]);
        self.data.push(1.0);
        self.nrows += 2;
    }

    
}