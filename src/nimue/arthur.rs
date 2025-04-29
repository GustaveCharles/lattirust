use ark_serialize::CanonicalSerialize;
use ark_std::rand::{CryptoRng, RngCore};
use nimue::{Arthur, ByteWriter, DuplexHash, IOPatternError};

use crate::linear_algebra::{Matrix, Scalar, SymmetricMatrix, Vector};
use crate::nimue::serialization::ToBytes;

pub trait SerArthur<H, R>
where
    H: DuplexHash<u8>,
    R: RngCore + CryptoRng,
    Self: ByteWriter,
{
    fn absorb_serializable<S: ToBytes>(&mut self, msg: &S) -> Result<(), IOPatternError> {
        match &msg.to_bytes() {
            Ok(bytes) => self.add_bytes(bytes.as_slice()),
            Err(e) => Err(IOPatternError::from(format! {"{:?}", e})),
        }
    }

    fn absorb_canonical_serializable<S: CanonicalSerialize>(
        &mut self,
        msg: &S,
    ) -> Result<(), IOPatternError> {
        let mut bytes = vec![];
        match msg.serialize_compressed(&mut bytes) {
            Ok(()) => self.add_bytes(bytes.as_slice()),
            Err(e) => Err(IOPatternError::from(e.to_string())),
        }
    }

    fn absorb_vector<F: Scalar>(&mut self, vec: &Vector<F>) -> Result<(), IOPatternError>
    where
        Vector<F>: ToBytes,
    {
        self.absorb_serializable(vec)
    }

    fn absorb_vec<F: Scalar>(&mut self, vec: &Vec<F>) -> Result<(), IOPatternError>
    where
        Vec<F>: ToBytes,
    {
        self.absorb_serializable(vec)
    }

    fn absorb_vector_canonical<F: Scalar>(&mut self, vec: &Vector<F>) -> Result<(), IOPatternError>
    where
        Vector<F>: CanonicalSerialize,
    {
        self.absorb_canonical_serializable(vec)
    }

    fn absorb_vectors<F: Scalar>(&mut self, vecs: &Vec<Vector<F>>) -> Result<(), IOPatternError>
    where
        Vec<Vector<F>>: ToBytes,
    {
        self.absorb_serializable(vecs)
    }

    fn absorb_symmetric_matrix<F: Clone>(
        &mut self,
        mat: &SymmetricMatrix<F>,
    ) -> Result<(), IOPatternError>
    where
        SymmetricMatrix<F>: ToBytes,
    {
        self.absorb_serializable(mat)
    }

    fn absorb_matrix<F: Scalar>(&mut self, mat: &Matrix<F>) -> Result<(), IOPatternError>
    where
        Matrix<F>: CanonicalSerialize,
    {
        self.absorb_canonical_serializable(mat)
    }

    fn absorb_matrix_ser<F: Scalar>(&mut self, mat: &Matrix<F>) -> Result<(), IOPatternError>
    where
        Matrix<F>: ToBytes,
    {
        self.absorb_serializable(mat)
    }
}

impl<H: DuplexHash<u8>, R: RngCore + CryptoRng> SerArthur<H, R> for Arthur<H, u8, R> {}
