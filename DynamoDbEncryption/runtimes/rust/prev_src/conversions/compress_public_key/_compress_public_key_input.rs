// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::compress_public_key::CompressPublicKeyInput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::CompressPublicKeyInput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::CompressPublicKeyInput::CompressPublicKeyInput {
        publicKey: primitives::conversions::ecc_public_key::to_dafny(&value.public_key.clone().unwrap())
,
 eccCurve: primitives::conversions::ecdh_curve_spec::to_dafny(value.ecc_curve.clone().unwrap()),
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::CompressPublicKeyInput,
    >,
) -> crate::operation::compress_public_key::CompressPublicKeyInput {
    crate::operation::compress_public_key::CompressPublicKeyInput::builder()
        .set_public_key(Some( primitives::conversions::ecc_public_key::from_dafny(dafny_value.publicKey().clone())
 ))
 .set_ecc_curve(Some( primitives::conversions::ecdh_curve_spec::from_dafny(dafny_value.eccCurve()) ))
        .build()
        .unwrap()
}
