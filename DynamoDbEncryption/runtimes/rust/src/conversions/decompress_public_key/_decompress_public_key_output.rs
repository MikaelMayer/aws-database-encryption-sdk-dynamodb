// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::decompress_public_key::DecompressPublicKeyOutput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::DecompressPublicKeyOutput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::DecompressPublicKeyOutput::DecompressPublicKeyOutput {
        publicKey: primitives::conversions::ecc_public_key::to_dafny(&value.public_key.clone().unwrap())
,
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::DecompressPublicKeyOutput,
    >,
) -> crate::operation::decompress_public_key::DecompressPublicKeyOutput {
    crate::operation::decompress_public_key::DecompressPublicKeyOutput::builder()
        .set_public_key(Some( primitives::conversions::ecc_public_key::from_dafny(dafny_value.publicKey().clone())
 ))
        .build()
        .unwrap()
}
