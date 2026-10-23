use std::fs;
use std::io::{self, Write};

use nalgebra::{DMatrix, DVector};

// TODO: написать контракт вручную
// TODO: написать доккомментарий вручную
pub fn read_source_choice() -> Result<u8, io::Error> {
    println!("Источник матрицы:");
    println!("  1 — вручную в консоле (элементы через пробел, по строке на вход)");
    println!("  2 — случайная симметричная (задать размерность)");
    println!("  3 — из файла (csv через ';', txt через пробел; дробная часть '.' или ',')");
    print!("Выбор: ");
    io::stdout().flush()?;

    let mut line = String::new();
    io::stdin().read_line(&mut line)?;

    line.trim().parse::<u8>().map_err(|err| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("'{line}' — не пункт меню: {err}"),
        )
    })
}

// TODO: написать контракт вручную
// TODO: написать доккомментарий вручную
pub fn read_dims() -> Result<usize, io::Error> {
    print!("Введите размерность матрицы (n): ");
    io::stdout().flush()?;

    let mut line = String::new();
    io::stdin().read_line(&mut line)?;

    line.trim().parse::<usize>().map_err(|err| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("'{line}' is not a number: {err}"),
        )
    })
}

// TODO: написать контракт вручную
// TODO: написать доккомментарий вручную
pub fn read_path() -> Result<String, io::Error> {
    print!("Путь к файлу: ");
    io::stdout().flush()?;

    let mut line = String::new();
    io::stdin().read_line(&mut line)?;

    let path = line.trim();
    if path.is_empty() {
        Err(io::Error::new(io::ErrorKind::InvalidInput, "пустой путь"))
    } else {
        Ok(path.to_string())
    }
}

// TODO: написать контракт вручную
// TODO: написать доккомментарий вручную
pub fn read_matrix_from_console(n: usize) -> Result<DMatrix<f32>, io::Error> {
    let mut data = Vec::with_capacity(n * n);

    for row in 0..n {
        println!("Строка {} ({} числа через пробел):", row + 1, n);
        io::stdout().flush()?;

        let mut line = String::new();
        io::stdin().read_line(&mut line)?;

        let numbers = parse_numbers(&line)?;
        if numbers.len() != n {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!(
                    "строка {}: ожидалось {n} чисел, получено {}",
                    row + 1,
                    numbers.len()
                ),
            ));
        }
        data.extend(numbers);
    }

    Ok(DMatrix::from_row_slice(n, n, &data))
}

// TODO: написать контракт вручную
// TODO: написать доккомментарий вручную
pub fn read_matrix_from_file(path: &str) -> Result<DMatrix<f32>, io::Error> {
    let content = fs::read_to_string(path)?;

    let mut data = Vec::new();
    let mut n = 0usize;

    for (idx, line) in content.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let numbers = parse_numbers(line)?;
        if n == 0 {
            n = numbers.len();
        } else if numbers.len() != n {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "строка {}: ожидалось {n} чисел, получено {}",
                    idx + 1,
                    numbers.len()
                ),
            ));
        }
        data.extend(numbers);
    }

    if n == 0 || data.len() != n * n {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "нужна квадратная матрица: строк {n}, всего чисел {}",
                data.len()
            ),
        ));
    }

    Ok(DMatrix::from_row_slice(n, n, &data))
}

// TODO: написать контракт вручную
// TODO: написать доккомментарий вручную
pub fn print_solution(eigenvalues: &DVector<f32>, eigenvectors: &DMatrix<f32>) {
    println!("Собственные числа:");
    for (i, &lambda) in eigenvalues.iter().enumerate() {
        println!("  lambda[{i}] = {lambda:.6}");
    }

    println!("Собственные векторы (столбцы):");
    println!("{eigenvectors:.4}");
}

// TODO: написать контракт вручную
// TODO: написать доккомментарий вручную
pub fn print_residual(
    a: &DMatrix<f32>,
    eigenvalues: &DVector<f32>,
    eigenvectors: &DMatrix<f32>,
) {
    let lambda_diag = DMatrix::from_diagonal(eigenvalues);
    let residual = (a * eigenvectors - eigenvectors * lambda_diag).norm();
    println!("Невязка ||A*V - V*diag(lambda)|| = {residual:.6e}");
}

// TODO: написать контракт вручную
// TODO: написать доккомментарий вручную
fn parse_numbers(line: &str) -> Result<Vec<f32>, io::Error> {
    line.split(|c: char| c == ';' || c.is_whitespace())
        .filter(|token| !token.is_empty())
        .map(|token| {
            token.replace(',', ".").parse::<f32>().map_err(|err| {
                io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("'{token}' — не число: {err}"),
                )
            })
        })
        .collect()
}

// TODO: написать тест вручную
