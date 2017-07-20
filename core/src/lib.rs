#![crate_type = "lib"]
#![deny(trivial_casts, trivial_numeric_casts, unsafe_code, unstable_features)]
#![warn(missing_debug_implementations, missing_docs, unused_qualifications, unused_import_braces)]

//! This is a library for basic DICOM content reading and writing.
//!
//! ## Example
//!
//! The following code does not depict the current functionalities, and the API
//! is subject to change.
//!
//! ```compile
//! # use dicom_core::{from_path, Result};
//! # fn foo() -> Result<()> {
//! let obj = from_path("0001.dcm")?;
//! let patient_name = obj.element_by_name("PatientName")?.as_str()?;
//! let modality = obj.element_by_name("Modality")?.as_str()?;
//! let pixel_data = obj.pixel_data()?;
//! # Ok(())
//! # }
//! ```

#[macro_use]
extern crate lazy_static;
extern crate byteorder;
extern crate encoding;
#[macro_use]
extern crate quick_error;
extern crate chrono;
extern crate itertools;

pub mod data;
pub mod dictionary;
pub mod error;
pub mod file;
pub mod iterator;
pub mod loader;
pub mod meta;
pub mod object;
pub mod transfer_syntax;

pub use data::value::DicomValue;
pub use data::VR;
pub use data::DataElement as DicomElement;
pub use dictionary::{DataDictionary, StandardDataDictionary};
pub use object::DicomObject;
pub use error::{Error, Result};

pub use object::mem::InMemDicomObject;

mod util;

type DefaultDicomObject = InMemDicomObject<&'static StandardDataDictionary>;


pub use file::{from_path, from_file, to_file};
