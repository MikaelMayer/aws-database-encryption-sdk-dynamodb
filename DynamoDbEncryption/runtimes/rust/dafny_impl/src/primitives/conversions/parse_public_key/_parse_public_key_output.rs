// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::primitives::operation::parse_public_key::ParsePublicKeyOutput,
) -> ::std::rc::Rc<
    crate::r#software::amazon::cryptography::primitives::internaldafny::types::ParsePublicKeyOutput,
>{
    ::std::rc::Rc::new(crate::r#software::amazon::cryptography::primitives::internaldafny::types::ParsePublicKeyOutput::ParsePublicKeyOutput {
        publicKey: crate::primitives::conversions::ecc_public_key::to_dafny(value.public_key.clone().unwrap())
,
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#software::amazon::cryptography::primitives::internaldafny::types::ParsePublicKeyOutput,
    >,
) -> crate::primitives::operation::parse_public_key::ParsePublicKeyOutput {
    crate::primitives::operation::parse_public_key::ParsePublicKeyOutput::builder()
        .set_public_key(Some( crate::primitives::conversions::ecc_public_key::from_dafny(dafny_value.publicKey().clone())
 ))
        .build()
        .unwrap()
}
