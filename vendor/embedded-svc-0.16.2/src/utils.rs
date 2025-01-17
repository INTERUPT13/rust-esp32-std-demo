#[cfg(feature = "alloc")]
pub mod anyerror;

// Uncomment once domain 0.6.2 which has no_std support is released
//#[cfg(feature = "experimental")]
//pub mod captive;

#[cfg(all(feature = "experimental", feature = "alloc", feature = "use_serde"))]
pub mod ghota;

#[cfg(all(feature = "experimental", feature = "alloc", feature = "use_serde"))]
pub mod rest;
