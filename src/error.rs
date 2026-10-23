use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum JacobiError {
    NotSquare,
    NotSymmetric,
    NonFiniteElements,
    DidNotConverge { iterations: usize, residual: f32 },
    EmptyMatrix,
}

impl fmt::Display for JacobiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotSquare => write!(f, "матрица не квадратная"),
            Self::NotSymmetric => write!(f, "матрица не симметрична"),
            Self::NonFiniteElements => write!(f, "матрица содержит NaN или Inf значения"),
            Self::DidNotConverge {iterations, residual} => write!(
                f, "не сошлось за {iterations} итераций (res: {residual})"
            ),
            Self::EmptyMatrix => write!(f, "матрица пустая"),
        }
    }
}

impl Error for JacobiError {}