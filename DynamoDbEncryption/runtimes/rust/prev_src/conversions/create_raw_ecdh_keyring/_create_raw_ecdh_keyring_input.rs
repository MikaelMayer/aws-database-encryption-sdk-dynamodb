// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::create_raw_ecdh_keyring::CreateRawEcdhKeyringInput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::CreateRawEcdhKeyringInput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::CreateRawEcdhKeyringInput::CreateRawEcdhKeyringInput {
        KeyAgreementScheme: material_providers::conversions::raw_ecdh_static_configurations::to_dafny(&value.key_agreement_scheme.clone().unwrap())
,
 curveSpec: primitives::conversions::ecdh_curve_spec::to_dafny(value.curve_spec.clone().unwrap()),
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::CreateRawEcdhKeyringInput,
    >,
) -> crate::operation::create_raw_ecdh_keyring::CreateRawEcdhKeyringInput {
    crate::operation::create_raw_ecdh_keyring::CreateRawEcdhKeyringInput::builder()
        .set_key_agreement_scheme(Some( material_providers::conversions::raw_ecdh_static_configurations::from_dafny(dafny_value.KeyAgreementScheme().clone())
 ))
 .set_curve_spec(Some( primitives::conversions::ecdh_curve_spec::from_dafny(dafny_value.curveSpec()) ))
        .build()
        .unwrap()
}
