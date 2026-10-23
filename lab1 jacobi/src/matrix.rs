use nalgebra::DMatrix;
#[derive(Default, Debug)]
pub struct MatrixConfig {
    pub dims: usize,
}

pub fn create_symmetric_matrix(matrix: MatrixConfig) -> DMatrix<f32> {
    let mtrx: DMatrix<f32> = DMatrix::new_random(matrix.dims, matrix.dims);
    let transposed = mtrx.transpose();

    mtrx + transposed
}

pub fn print_matrix(mtrx: &DMatrix<f32>) {
    println!("mtrx = {:.2}", &mtrx);
}