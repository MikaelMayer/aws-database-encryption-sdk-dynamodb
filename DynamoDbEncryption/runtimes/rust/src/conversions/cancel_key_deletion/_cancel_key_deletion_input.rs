// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::cancel_key_deletion::CancelKeyDeletionRequest,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::CancelKeyDeletionRequest,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::CancelKeyDeletionRequest::CancelKeyDeletionRequest {
        KeyId: crate::standard_library_conversions::ostring_to_dafny(&value.key_id) .Extract(),
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::CancelKeyDeletionRequest,
    >,
) -> crate::operation::cancel_key_deletion::CancelKeyDeletionRequest {
    crate::operation::cancel_key_deletion::CancelKeyDeletionRequest::builder()
        .set_key_id(Some( dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(dafny_value.KeyId()) ))
        .build()
        .unwrap()
}
