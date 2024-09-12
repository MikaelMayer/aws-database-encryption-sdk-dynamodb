// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::generate_ecc_key_pair::GenerateEccKeyPairInput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::GenerateECCKeyPairInput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::GenerateECCKeyPairInput::GenerateECCKeyPairInput {
        eccCurve: primitives::conversions::ecdh_curve_spec::to_dafny(value.ecc_curve.clone().unwrap()),
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::GenerateECCKeyPairInput,
    >,
) -> crate::operation::generate_ecc_key_pair::GenerateEccKeyPairInput {
    crate::operation::generate_ecc_key_pair::GenerateEccKeyPairInput::builder()
        .set_ecc_curve(Some( primitives::conversions::ecdh_curve_spec::from_dafny(dafny_value.eccCurve()) ))
        .build()
        .unwrap()
}
