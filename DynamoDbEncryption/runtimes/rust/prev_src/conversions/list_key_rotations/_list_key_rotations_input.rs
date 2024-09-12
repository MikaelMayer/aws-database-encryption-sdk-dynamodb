// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::list_key_rotations::ListKeyRotationsRequest,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::ListKeyRotationsRequest,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::ListKeyRotationsRequest::ListKeyRotationsRequest {
        KeyId: crate::standard_library_conversions::ostring_to_dafny(&value.key_id) .Extract(),
 Limit: crate::standard_library_conversions::oint_to_dafny(value.limit),
 Marker: crate::standard_library_conversions::ostring_to_dafny(&value.marker),
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::ListKeyRotationsRequest,
    >,
) -> crate::operation::list_key_rotations::ListKeyRotationsRequest {
    crate::operation::list_key_rotations::ListKeyRotationsRequest::builder()
        .set_key_id(Some( dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(dafny_value.KeyId()) ))
 .set_limit(crate::standard_library_conversions::oint_from_dafny(dafny_value.Limit().clone()))
 .set_marker(crate::standard_library_conversions::ostring_from_dafny(dafny_value.Marker().clone()))
        .build()
        .unwrap()
}
