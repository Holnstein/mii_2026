use jacobi::{JacobiError, JacobiSolver};
use nalgebra::DMatrix;

#[test]
fn solve_satisfies_eigen_equation() {
    let a = DMatrix::from_row_slice(3, 3, &[
        2.0, 1.0, 0.0,
        1.0, 2.0, 1.0,
        0.0, 1.0, 2.0,
    ]);
    let (values, vectors) = JacobiSolver::new().solve(&a).unwrap();

    for i in 0..values.len() {
        let v = vectors.column(i);
        let av = &a * v;
        for k in 0..v.len() {
            assert!((av[k] - values[i] * v[k]).abs() < JacobiSolver::new().tolerance);
        }
    }
}

#[test]
fn non_square_matrix() {
    let a = DMatrix::from_row_slice(2, 1, &[1.0, 3.0]);

    assert_eq!(JacobiSolver::new().solve(&a), Err(JacobiError::NotSquare));
}

#[test]
fn non_symmetric_matrix() {
    let a = DMatrix::from_row_slice(2, 2, &[
        1.0, 1.0,
        0.9, 1.0
    ]);
    assert_eq!(JacobiSolver::new().solve(&a), Err(JacobiError::NotSymmetric));
}

#[test]
fn non_finite_elements_matrix() {
    let a = DMatrix::from_row_slice(2, 2, &[f32::INFINITY, 1.0, 2.0, 3.0]);

    assert_eq!(JacobiSolver::new().solve(&a), Err(JacobiError::NonFiniteElements));
}

