// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Wraps up an arbitrary Rust Error value as a Dafny Error
pub fn to_opaque_error<E: 'static>(value: E) ->
    ::std::rc::Rc<crate::r#software::amazon::cryptography::primitives::internaldafny::types::Error>
{
    let error_obj: ::dafny_runtime::Object<dyn::std::any::Any> = ::dafny_runtime::Object(Some(
        ::std::rc::Rc::new(::std::cell::UnsafeCell::new(value)),
    ));
    ::std::rc::Rc::new(
        crate::r#software::amazon::cryptography::primitives::internaldafny::types::Error::Opaque {
            obj: error_obj,
        },
    )
}

/// Wraps up an arbitrary Rust Error value as a Dafny Result<T, Error>.Failure
pub fn to_opaque_error_result<T: ::dafny_runtime::DafnyType, E: 'static>(value: E) ->
    ::std::rc::Rc<
        crate::Wrappers::Result<
            T,
            ::std::rc::Rc<crate::r#software::amazon::cryptography::primitives::internaldafny::types::Error>
        >
    >
{
    ::std::rc::Rc::new(crate::Wrappers::Result::Failure {
        error: to_opaque_error(value),
    })
}
pub fn to_dafny(
    value: crate::primitives::types::error::Error,
) -> ::std::rc::Rc<crate::r#software::amazon::cryptography::primitives::internaldafny::types::Error> {
    ::std::rc::Rc::new(match value {
        crate::primitives::types::error::Error::AwsCryptographicPrimitivesError { message } =>
    crate::r#software::amazon::cryptography::primitives::internaldafny::types::Error::AwsCryptographicPrimitivesError {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
        crate::primitives::types::error::Error::CollectionOfErrors { list, message } =>
            crate::r#software::amazon::cryptography::primitives::internaldafny::types::Error::CollectionOfErrors {
                message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
                list: ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(&list, |e| to_dafny(e.clone()))
            },
        crate::primitives::types::error::Error::Opaque { obj } =>
            crate::r#software::amazon::cryptography::primitives::internaldafny::types::Error::Opaque {
                obj: ::dafny_runtime::Object(obj.0)
            },
    })
}

#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#software::amazon::cryptography::primitives::internaldafny::types::Error,
    >,
) -> crate::primitives::types::error::Error {
    match ::std::borrow::Borrow::borrow(&dafny_value) {
        crate::r#software::amazon::cryptography::primitives::internaldafny::types::Error::AwsCryptographicPrimitivesError { message } =>
    crate::primitives::types::error::Error::AwsCryptographicPrimitivesError {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
        crate::r#software::amazon::cryptography::primitives::internaldafny::types::Error::CollectionOfErrors { list, message } =>
            crate::primitives::types::error::Error::CollectionOfErrors {
                message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
                list: ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(&list, |e| from_dafny(e.clone()))
            },
        crate::r#software::amazon::cryptography::primitives::internaldafny::types::Error::Opaque { obj } =>
            crate::primitives::types::error::Error::Opaque {
                obj: obj.clone()
            },
    }
}
