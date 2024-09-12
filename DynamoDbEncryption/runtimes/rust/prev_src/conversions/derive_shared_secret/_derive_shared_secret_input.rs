// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::derive_shared_secret::DeriveSharedSecretInput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::DeriveSharedSecretInput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::DeriveSharedSecretInput::DeriveSharedSecretInput {
        eccCurve: primitives::conversions::ecdh_curve_spec::to_dafny(value.ecc_curve.clone().unwrap()),
 privateKey: primitives::conversions::ecc_private_key::to_dafny(&value.private_key.clone().unwrap())
,
 publicKey: primitives::conversions::ecc_public_key::to_dafny(&value.public_key.clone().unwrap())
,
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::DeriveSharedSecretInput,
    >,
) -> crate::operation::derive_shared_secret::DeriveSharedSecretInput {
    crate::operation::derive_shared_secret::DeriveSharedSecretInput::builder()
        .set_ecc_curve(Some( primitives::conversions::ecdh_curve_spec::from_dafny(dafny_value.eccCurve()) ))
 .set_private_key(Some( primitives::conversions::ecc_private_key::from_dafny(dafny_value.privateKey().clone())
 ))
 .set_public_key(Some( primitives::conversions::ecc_public_key::from_dafny(dafny_value.publicKey().clone())
 ))
        .build()
        .unwrap()
}
