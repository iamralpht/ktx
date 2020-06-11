use std;
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Texture<T> {
    TwoDim(T),
    Cubemap {
        all: T,
        x: T,
        x_neg: T,
        y: T,
        y_neg: T,
        z: T,
        z_neg: T,
    },
}

impl std::ops::Deref for Texture<&[u8]> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        match *self {
            Self::TwoDim(slice) => slice,
            Self::Cubemap { all, .. } => all,
        }
    }
}

impl std::ops::Deref for Texture<Vec<u8>> {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::TwoDim(slice) => slice,
            Self::Cubemap { all, .. } => all,
        }
    }
}
