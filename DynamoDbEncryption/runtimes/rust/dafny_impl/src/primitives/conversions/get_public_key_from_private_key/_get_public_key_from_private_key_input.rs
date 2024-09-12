// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::primitives::operation::get_public_key_from_private_key::GetPublicKeyFromPrivateKeyInput,
) -> ::std::rc::Rc<
    crate::software::amazon::cryptography::primitives::internaldafny::types::GetPublicKeyFromPrivateKeyInput,
>{
    ::std::rc::Rc::new(crate::software::amazon::cryptography::primitives::internaldafny::types::GetPublicKeyFromPrivateKeyInput::GetPublicKeyFromPrivateKeyInput {
        eccCurve: crate::primitives::conversions::ecdh_curve_spec::to_dafny(value.ecc_curve.clone().unwrap()),
 privateKey: crate::primitives::conversions::ecc_private_key::to_dafny(&value.private_key.clone().unwrap())
,
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::software::amazon::cryptography::primitives::internaldafny::types::GetPublicKeyFromPrivateKeyInput,
    >,
) -> crate::primitives::operation::get_public_key_from_private_key::GetPublicKeyFromPrivateKeyInput {
    crate::primitives::operation::get_public_key_from_private_key::GetPublicKeyFromPrivateKeyInput::builder()
        .set_ecc_curve(Some( crate::primitives::conversions::ecdh_curve_spec::from_dafny(dafny_value.eccCurve()) ))
 .set_private_key(Some( crate::primitives::conversions::ecc_private_key::from_dafny(dafny_value.privateKey().clone())
 ))
        .build()
        .unwrap()
}
