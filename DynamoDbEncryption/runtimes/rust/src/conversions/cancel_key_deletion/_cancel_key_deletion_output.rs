// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::cancel_key_deletion::CancelKeyDeletionResponse,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::CancelKeyDeletionResponse,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::CancelKeyDeletionResponse::CancelKeyDeletionResponse {
        KeyId: crate::standard_library_conversions::ostring_to_dafny(&value.key_id),
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::CancelKeyDeletionResponse,
    >,
) -> crate::operation::cancel_key_deletion::CancelKeyDeletionResponse {
    crate::operation::cancel_key_deletion::CancelKeyDeletionResponse::builder()
        .set_key_id(crate::standard_library_conversions::ostring_from_dafny(dafny_value.KeyId().clone()))
        .build()
        .unwrap()
}
