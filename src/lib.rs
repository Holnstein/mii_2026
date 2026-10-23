pub mod matrix;
pub mod console_io;
pub mod jacobi;
pub mod error;
pub mod solver;

pub use error::JacobiError;
pub use jacobi::off_sum;
pub use solver::JacobiSolver;