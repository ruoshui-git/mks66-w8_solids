use super::Matrix;

/// impl Vec<Matrix> with utility fn to make it easier to work with transformation stacks
pub trait MStack<Matrix> {
    /// Get a reference to the top of the matrix stack
    fn get_top(&self) -> &Matrix;
    fn push_matrix(&mut self);
    fn pop_matrix(&mut self);
    fn transform_top(&mut self, trans: &Matrix);
    fn new_stack() -> Self;
}

impl MStack<Matrix> for Vec<Matrix> {
    /// Get a reference to the top of the matrix stack
    fn get_top(&self) -> &Matrix {
        self.last().expect("Error trying to get the last stack")
    }

    fn transform_top(&mut self, trans: &Matrix) {
        *self.last_mut().expect("Error trying to get the last stack") =
            trans * self.get_top();
    }

    fn push_matrix(&mut self) {
        self.push(self.get_top().clone());
    }

    fn pop_matrix(&mut self) {
        self.pop();
    }

    fn new_stack() -> Self {
        vec![Matrix::ident(4)]
    }
}
