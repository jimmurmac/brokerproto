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
/// impl Default for Foo {
///     fn default() -> Self {
///         Foo {
///             name: String::new(),
///             value: 0,
///         }
///     }
/// }
/// 
/// impl StructDeserializer for Foo {}
/// impl StructSerializer for Foo {}
/// 
/// fn main() {
///     let foo = Foo::new("test".to_string(), 42);
///     let json_str = foo.make_string_from_struct().unwrap();
///     assert_eq!(json_str, r#"{"name":"test","value":42}"#);
/// 
///     let deserialized_foo = Foo::make_struct_from_string(&json_str).unwrap();
///     assert_eq!(deserialized_foo.get_name(), "test");
///     assert_eq!(deserialized_foo.get_value(), 42);
/// }

#[allow(dead_code)]
pub trait StructDeserializer {
    fn make_struct_from_string<'a, T>(json_str: &'a str) -> Result<T, serde_json::Error>
    where
        T: std::marker::Sized + serde::Deserialize<'a>,
    {
        serde_json::from_str(json_str)
    }
}

#[allow(dead_code)]
pub trait StructSerializer {
    fn make_string_from_struct(&self) -> Result<String, serde_json::Error>
    where
        Self: std::marker::Sized + serde::Serialize,
    {
        serde_json::to_string(&self)
    }
}

/*  --------------------------------------------------------------------------
    Unit Tests
    ------------------------------------------------------------------------- */

mod tests {
   


    #[test]
    fn serialize_foo() {

        // use crate::serialization_helpers::{StructDeserializer, StructSerializer};
        use super::{StructDeserializer, StructSerializer};
        use serde::{Deserialize, Serialize};

        #[derive(Serialize, Deserialize, Debug)]
        pub struct Foo {
            name: String,
            value: i32,
        }

        impl Foo {

            pub fn new(name: String, value: i32) -> Self {
                Foo { name, value }
            }

            pub fn get_name(&self) -> &str {
                &self.name
            }

            pub fn get_value(&self) -> i32 {
                self.value
            }
        }

        impl Default for Foo {
            fn default() -> Self {
                Foo { name: String::new(), value: 0 }
            }
        }

        impl StructDeserializer for Foo {}
        impl StructSerializer for Foo {}

        let foo = Foo::new("test".to_string(), 42);
        assert_eq!(foo.get_name(), "test");
        assert_eq!(foo.get_value(), 42);
        let json_str = foo.make_string_from_struct().unwrap();
        assert_eq!(json_str, r#"{"name":"test","value":42}"#);

        let deserialized_foo: Foo = Foo::make_struct_from_string(&json_str).unwrap();
        assert_eq!(deserialized_foo.get_name(), "test");
        assert_eq!(deserialized_foo.get_value(), 42);
    }
}
