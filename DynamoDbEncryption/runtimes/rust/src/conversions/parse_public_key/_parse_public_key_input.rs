// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::parse_public_key::ParsePublicKeyInput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::ParsePublicKeyInput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::ParsePublicKeyInput::ParsePublicKeyInput {
        publicKey: crate::standard_library_conversions::oblob_to_dafny(&value.public_key).Extract(),
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::ParsePublicKeyInput,
    >,
) -> crate::operation::parse_public_key::ParsePublicKeyInput {
    crate::operation::parse_public_key::ParsePublicKeyInput::builder()
        .set_public_key(Some(crate::standard_library_conversions::blob_from_dafny(dafny_value.publicKey().clone())))
        .build()
        .unwrap()
}
