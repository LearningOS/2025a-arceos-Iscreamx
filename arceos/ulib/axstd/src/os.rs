//! OS-specific functionality.

/// ArceOS-specific definitions.
pub mod arceos {
    pub use arceos_api as api;
    #[doc(no_inline)]
    pub use arceos_api::modules;
}

pub fn get_random() -> u128 {
    arceos_api::misc::get_random()
}
