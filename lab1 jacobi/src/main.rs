use std::io;

use nalgebra::DMatrix;

use jacobi::console_io;
use jacobi::matrix;
use jacobi::matrix::MatrixConfig;
use jacobi::{off_sum, JacobiSolver};

fn read_input_matrix() -> Result<DMatrix<f32>, io::Error> {
    match console_io::read_source_choice()? {
        1 => {
            let dims = console_io::read_dims()?;
            if dims == 0 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "размерность должна быть не меньше 1",
                ));
            }
            console_io::read_matrix_from_console(dims)
        }
        2 => {
            let dims = console_io::read_dims()?;
            if dims == 0 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "размерность должна быть не меньше 1",
                ));
            }
            Ok(matrix::create_symmetric_matrix(MatrixConfig { dims }))
        }
        3 => {
            let path = console_io::read_path()?;
            console_io::read_matrix_from_file(&path)
        }
        other => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("пункт меню должен быть 1, 2 или 3, получено {other}"),
        )),
    }
}

/// Главный двигатель процесса
///
/// # Examples
/// ```
/// let a = 1;
/// assert!(a, 1);
/// ```
/// 
fn main() {
    let mtrx = match read_input_matrix() {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Ошибка ввода: {e}");
            return;
        }
    };

    let (rows, cols) = (mtrx.nrows(), mtrx.ncols());
    println!("Исходная матрица A ({rows}x{cols}):");
    matrix::print_matrix(&mtrx);

    let s0 = off_sum(&mtrx);
    println!("S0 (сумма квадратов недиагональных элементов) = {s0:.6e}");

    match JacobiSolver::new().solve(&mtrx) {
        Ok((eigenvalues, eigenvectors)) => {
            console_io::print_solution(&eigenvalues, &eigenvectors);
            console_io::print_residual(&mtrx, &eigenvalues, &eigenvectors);
        }
        Err(e) => {
            eprintln!("Ошибка решения: {e}");
        }
    }
}
