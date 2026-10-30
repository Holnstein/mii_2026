//! Основной файл запуска алгоритма метода Якоби для поиска собственных чисел и векторов
//! симметричной матрицы
use std::io;
use std::io::Write;
use nalgebra::DMatrix;

use jacobi::console_io;
use jacobi::matrix;
use jacobi::matrix::MatrixConfig;
use jacobi::{off_sum, JacobiSolver};


/// Чтение матрицы с консольного инпута
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

    let tolerance = match console_io::read_tolerance() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Ошибка ввода точности: {e}");
            return;
        }
    };

    let max_iterations = match console_io::read_max_iterations() {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Ошибка ввода итераций: {e}");
            return;
        }
    };

    println!("Параметры: точность = {tolerance}, макс. итераций = {max_iterations}");

    let solver = JacobiSolver::new()
        .with_tolerance(tolerance)
        .with_max_iterations(max_iterations);

    match solver.solve(&mtrx) {
        Ok((eigenvalues, eigenvectors)) => {
            console_io::print_solution(&eigenvalues, &eigenvectors);
            console_io::print_residual(&mtrx, &eigenvalues, &eigenvectors);
        }
        Err(e) => {
            eprintln!("Ошибка решения: {e}");
        }
    }

    println!("Нажмите Enter для выхода...");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}
