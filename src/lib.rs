mod bitpacking;
mod delta_encoding;

pub mod profiles {
    mod ascii;
    mod dna;
    mod iupac;
    mod profile;

    pub use ascii::{Ascii, CaseInsensitiveAscii, CaseSensitiveAscii};
    pub use dna::Dna;
    pub use iupac::Iupac;
    pub use profile::Profile;
}

mod minima;
pub mod rec_iter;
pub mod search;
mod trace;

// Python bindings module
#[cfg(feature = "python")]
mod python;

#[doc(hidden)]
pub mod private {
    pub use crate::minima::prefix_min;
}

#[cfg(feature = "avx512")]
const LANES: usize = 8;
#[cfg(feature = "avx512")]
type S = wide::u64x8;
#[cfg(not(feature = "avx512"))]
const LANES: usize = 4;
#[cfg(not(feature = "avx512"))]
type S = wide::u64x4;
