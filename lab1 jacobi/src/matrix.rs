use nalgebra::DMatrix;
#[derive(Default, Debug)]
pub struct MatrixConfig {
    pub dims: usize,
}

/// Создание случайной симметричной матрицы путем сложения случайной матрицы с ее транспонированной
///
/// # Аргументы
///
/// * `matrix` - конфиг матрицы: порядок матрицы
///
/// # Возвращаемое значение
///
/// * `Dmatrix<f32>` - симметричная матрица с десятичными значениями
///
pub fn create_symmetric_matrix(matrix: MatrixConfig) -> DMatrix<f32> {
    let mtrx: DMatrix<f32> = DMatrix::new_random(matrix.dims, matrix.dims);
    let transposed = mtrx.transpose();

    mtrx + transposed
}

/// Выводит в консоль матрицу
pub fn print_matrix(mtrx: &DMatrix<f32>) {
    println!("mtrx = {:.2}", &mtrx);
}