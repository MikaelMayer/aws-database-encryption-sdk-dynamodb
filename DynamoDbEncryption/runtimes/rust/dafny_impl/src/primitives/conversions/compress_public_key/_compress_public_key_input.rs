// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::primitives::operation::compress_public_key::CompressPublicKeyInput,
) -> ::std::rc::Rc<
    crate::r#software::amazon::cryptography::primitives::internaldafny::types::CompressPublicKeyInput,
>{
    ::std::rc::Rc::new(crate::r#software::amazon::cryptography::primitives::internaldafny::types::CompressPublicKeyInput::CompressPublicKeyInput {
        publicKey: crate::primitives::conversions::ecc_public_key::to_dafny(value.public_key.clone().unwrap())
,
 eccCurve: crate::primitives::conversions::ecdh_curve_spec::to_dafny(value.ecc_curve.clone().unwrap()),
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#software::amazon::cryptography::primitives::internaldafny::types::CompressPublicKeyInput,
    >,
) -> crate::primitives::operation::compress_public_key::CompressPublicKeyInput {
    crate::primitives::operation::compress_public_key::CompressPublicKeyInput::builder()
        .set_public_key(Some( crate::primitives::conversions::ecc_public_key::from_dafny(dafny_value.publicKey().clone())
 ))
 .set_ecc_curve(Some( crate::primitives::conversions::ecdh_curve_spec::from_dafny(dafny_value.eccCurve()) ))
        .build()
        .unwrap()
}
