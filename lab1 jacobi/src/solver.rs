//! Методы для решения и найденных собственных чисел и векторов для матрицы в алгоритме Якоби
use nalgebra::{DMatrix, DVector};

use crate::error::JacobiError;
use crate::jacobi::{off_sum, rotate_a, rotate_v, rotation_cos_sin};

pub struct JacobiSolver {
    pub max_iterations: usize,
    pub tolerance: f32,
}

impl JacobiSolver {
    /// Создание структуры JacobiSolver с дефолтными параметрами (кол-во итераций и точность)
    pub fn new() -> Self {
        Self {
            max_iterations: 1000,
            tolerance: 1e-6,
        }
    }

    /// Задание точности алгоритма
    ///
    /// # Аргументы
    ///
    /// * `self` - референс текущего модуля, скрывающее название текущего типа
    /// * `eps` - точность
    ///
    /// # Возвращаемые параметры
    ///
    /// `Self` - JacobiSolver с заданной точностью
    ///
    /// # Паника
    ///
    /// Метод паникует, если точность неположительная или точность не является конечным числом
    pub fn with_tolerance(mut self, eps: f32) -> Self {
        assert!(eps > 0.0, "eps should be a positive number");
        assert!(eps.is_finite(), "eps should be finite");

        self.tolerance = eps;
        self
    }

    /// Задание максимального кол-ва итераций
    ///
    /// # Аргументы
    ///
    /// * `self` - референс текущего модуля, скрывающее название текущего типа
    /// * `iters` - кол-во итераций
    ///
    /// # Возвращаемые параметры
    ///
    /// `Self` - JacobiSolver с заданной точностью
    ///
    /// # Паника
    ///
    /// Паникует, когда кол-во итераций неположительно
    pub fn with_max_iterations(mut self, iters: usize) -> Self {
        assert!(iters > 0, "iters should be a positive number");

        self.max_iterations = iters;
        self
    }

    /// Точка решения алгоритма Якоби
    ///
    /// Алгоритм прекращается либо по окончанию итераций (не сошлось), либо сумма квадратов
    /// недиагональных элементов == 0, то есть все внедиагональные элементы занулились
    ///
    /// # Аргументы
    ///
    /// * `self` - референс текущего модуля, скрывающее название текущего типа
    /// * `a` - симметричная матрица, с которой работает алгоритм
    ///
    /// # Возвращаемые значения
    ///
    /// * `Ok((work.diagonal()), v)` - при успехе возвращаются собственные числа и матрицу
    ///   собственных векторов
    ///
    /// * `Err(...)` - алгоритм не сошелся, возвращает ошибку с кол-во итераций и суммой квадратов
    ///
    /// # Паника
    ///
    /// Метод паникует при выходе из границ цикла
    pub fn solve(&self, a: &DMatrix<f32>) -> Result<(DVector<f32>, DMatrix<f32>), JacobiError> {
        check_square(a)?;
        check_finite(a)?;
        check_symmetric(a)?;

        let n = a.nrows();

        if n < 2 {
            return Ok((a.diagonal(), DMatrix::identity(n, n)));
        }

        let mut work = a.clone();
        let mut v: DMatrix<f32> = DMatrix::identity(n, n);

        let mut s = off_sum(a);

        if s < self.tolerance {
            return Ok((work.diagonal(), v));
        }

        for _ in 0..self.max_iterations {
            for p in 0..n - 1 {
                for q in p + 1..n {
                    let a_pp = work[(p, p)];
                    let a_qq = work[(q, q)];
                    let a_pq = work[(p, q)];

                    if let Some((cos, sin)) = rotation_cos_sin(a_pp, a_qq, a_pq) {
                        let tg = sin / cos;
                        s -= rotate_a(&mut work, p, q, tg, cos, sin);
                        rotate_v(&mut v, p, q, cos, sin);
                    }
                }
            }

            if s.max(0.0) < self.tolerance {
                return Ok((work.diagonal(), v));
            }
        }

        Err(JacobiError::DidNotConverge {
            iterations: self.max_iterations,
            residual: s.max(0.0),
        })
    }
}

impl Default for JacobiSolver {
    fn default() -> Self {
        Self::new()
    }
}

/// Проверяет, является ли матрица квадратной
///
/// # Аргументы
///
/// * `a` - матрица для проверки
///
/// # Возвращаемые значения
///
/// * `Ok(())` - успех, матрица квадратична
///
/// * `Err(JacobiError::NotSquare)` - ошибка, матрица не квадратична
///
fn check_square(a: &DMatrix<f32>) -> Result<(), JacobiError> {
    if !a.is_square() {
        return Err(JacobiError::NotSquare)
    }
    Ok(())
}

/// Проверяет, являются ли все элементы матрицы конечными числами (или же NaN и тд)
///
/// # Аргументы
///
/// * `a` - матрица для проверки
///
/// # Возвращаемые значения
///
/// * `Ok(())` - успех, все числа конечны
///
/// * `Err(JacobiError::NonFiniteElements)` - ошибка, есть хоть одно неконечное число
///
fn check_finite(a: &DMatrix<f32>) -> Result<(), JacobiError> {
    if a.iter().all(|&x| x.is_finite()) {
        Ok(())
    } else {
        Err(JacobiError::NonFiniteElements)
    }
}

/// Проверяет, является ли матрица симметричной, то есть Aij == Aji с определенной точностью
///
/// # Аргументы
///
/// * `a` - матрица для проверки
///
/// # Возвращаемые значения
///
/// * `Ok(())` - успех, матрица симметрична
///
/// * `Err(JacobiError::NotSymmetric)` - ошибка, симметрии нет
///
fn check_symmetric(a: &DMatrix<f32>) -> Result<(), JacobiError> {
    let eps = 1e-4;
    for i in 0..a.nrows() {
        for j in i + 1..a.ncols() {
            let diff = (a[(i, j)] - a[(j, i)]).abs();
            if diff > eps {
                return Err(JacobiError::NotSymmetric)
            }
        }
    }
    Ok(())
}
