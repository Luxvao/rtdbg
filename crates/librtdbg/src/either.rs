#[derive(Debug, Clone)]
pub enum Either<A, B> {
    First(A),
    Second(B),
}

impl<A, B> Either<A, B> {
    pub fn map<U, V>(self, f1: impl FnOnce(A) -> U, f2: impl FnOnce(B) -> V) -> Either<U, V> {
        match self {
            Either::First(a) => Either::First(f1(a)),
            Either::Second(b) => Either::Second(f2(b)),
        }
    }

    pub fn as_ref(&self) -> Either<&A, &B> {
        match self {
            Either::First(a) => Either::First(a),
            Either::Second(b) => Either::Second(b),
        }
    }

    pub fn as_mut(&mut self) -> Either<&mut A, &mut B> {
        match self {
            Either::First(a) => Either::First(a),
            Either::Second(b) => Either::Second(b),
        }
    }
}
