mod generic_matrix;
pub mod inner_products;
mod matrix;
pub mod serialization;
mod sparse_matrix;
mod symmetric_matrix;
mod vector;

pub type Vector<T> = vector::Vector<T>;
pub type SVector<T, const N: usize> = vector::SVector<T, N>;
pub type RowVector<T> = vector::RowVector<T>;
pub type Matrix<T> = matrix::Matrix<T>;
pub type SymmetricMatrix<T> = symmetric_matrix::SymmetricMatrix<T>;
pub type SparseMatrix<T> = sparse_matrix::SparseMatrix<T>;
pub trait Scalar = nalgebra::Scalar;
