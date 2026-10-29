pub mod matrix;
pub mod console_io;
pub mod jacobi;
pub mod error;
pub mod solver;

pub use error::JacobiError;
pub use jacobi::off_sum;
pub use solver::JacobiSolver;

#[cfg(test)]
mod tests {
    use approx::relative_eq;
    use nalgebra::DMatrix;
    use crate::jacobi::{rotate_a, rotation_cos_sin};
    use super::*;

    #[test]
    fn off_sum_of_2x2() {
        let a = DMatrix::from_row_slice(2, 2, &[1.0, 3.0, 3.0, 5.0]);
        assert!((off_sum(&a) - 9.0).abs() < 1e-6);
    }

    #[test]
    fn off_sum_of_1x1_is_zero() {
        let a = DMatrix::from_row_slice(1, 1, &[42.0]);
        assert_eq!(off_sum(&a), 0.0);
    }

    #[test]
    fn rotation_check_cos_and_sin_example_from_lab1() {
        let (cos, sin) = rotation_cos_sin(7.0, 19.0, 13.0).unwrap();
        assert!((cos - 0.842).abs() < 1e-2);
        assert!((sin - 0.539).abs() < 1e-2);
    }

    #[test]
    /// Проверка формулы (7): при выборе угла по (9)-(11)
    fn rotation_check_seven_equation_from_lab1() {
        let cases = [
            (7.0, 19.0, 13.0),               // пример из методички, C > 0
            (19.0, 7.0, 13.0),               // C < 0
            (7.0, 19.0, -13.0),              // a_pq < 0
            (100.0, -100.0, 50.0),           // C = 0
        ];

        for (a_pp, a_qq, a_pq) in cases {
            let (cos, sin) = rotation_cos_sin(a_pp, a_qq, a_pq)
                .expect("a_pq != 0, должен быть Some");

            // формула (6): A_pq' = (cos^2 - sin^2)·a_pq + sin * cos * (a_pp - a_qq)
            let new_a_pq = (cos * cos - sin * sin) * a_pq
                + sin * cos * (a_pp - a_qq);

            let eps = 1e-5;

            assert!(
                new_a_pq.abs() < eps,
                "A_pq' = {new_a_pq}, ожидалось около 0 с точностью {eps} для \
                 a_pp = {a_pp}, a_qq = {a_qq}, a_pq = {a_pq}"
            );
        }
    }

    #[test]
    // проверка поворота матрицы A, будет падать на маленьких eps из-за сравнения матриц
    // по методичке
    fn rotate_a_example_lab1() {
        let mut a = DMatrix::from_row_slice(5, 5, &[
            1.0, 4.0, 7.0, 10.0, 13.0,
            4.0, 7.0, 10.0, 13.0, 16.0,
            7.0, 10.0, 13.0, 16.0, 19.0,
            10.0, 13.0, 16.0, 19.0, 22.0,
            13.0, 16.0, 19.0, 22.0, 25.0,
        ]);

        let (cos, sin) = rotation_cos_sin(a[(1, 1)], a[(3, 3)], a[(1, 3)]).unwrap();
        let tg = sin / cos;

        rotate_a(&mut a, 1, 3, tg, cos, sin);

        assert_eq!(a[(1, 3)], 0.0, "A[1][3] должен быть 0");
        assert_eq!(a[(3, 1)], 0.0, "A[4][2] должен быть 0");

        let a_itog = DMatrix::from_row_slice(5, 5, &[
            1.0, -2.02, 7.0, 10.58, 13.0,
            -2.02, -1.32, -0.2, 0.0, 1.62,
            7.0, -0.2, 13.0, 18.86, 19.0,
            10.58, 0.0, 18.86, 27.32, 27.15,
            13.0, 1.62, 19.0, 27.15, 25.0,
        ]);

        for r in 0..a.nrows() {
            for c in 0..a.ncols() {
                assert!(
                    (a[(r, c)] - a[(c, r)]).abs() < 1e-5,
                    "симметрия нарушена в [{r}][{c}]: {} vs {}",
                    a[(r, c)], a[(c, r)]
                );
            }
        }

        let is_equal = relative_eq!(a, a_itog, epsilon = 1e-2);
        assert!(is_equal, "a={a} a itog = {a_itog}");
    }

}