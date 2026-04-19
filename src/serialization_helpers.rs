/* ==========================================================================
 File:           serialization_helpers.rs

 Description:    traits for serializing and deserializing Rust structs

 History:        Jim Murphy 04/26/2026 - Created
 Copyright ©    2026 Jim Murphy All rights reserved.
========================================================================== */

/// Serialization and deserialization traits for Rust structs.
/// This module defines traits for serializing and deserializing Rust structs to and from strings.
/// The StructDeserializer trait provides a method for creating structs from JSON strings.
/// 
/// # Example
/// ```
/// use serde::{Deserialize, Serialize};
/// 
/// #[derive(Serialize, Deserialize, Debug)]
/// pub struct Foo {
///     pub name: String,
///     pub value: i32,
/// }
/// 
///  impl StructDeserializer for Foo {}
/// 

pub trait StructDeserializer {
    fn make_struct_from_string<'a, T>(json_str: &'a str) -> Result<T, serde_json::Error>
    where
        T: std::marker::Sized + serde::Deserialize<'a>,
    {
        serde_json::from_str(json_str)
    }
}

pub trait StructSerializer {
    fn make_string_from_struct(&self) -> Result<String, serde_json::Error>
    where
        Self: std::marker::Sized + serde::Serialize,
    {
        serde_json::to_string(&self)
    }
}