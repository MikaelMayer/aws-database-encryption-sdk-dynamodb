// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::ecdsa_sign::EcdsaSignOutput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::ECDSASignOutput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::ECDSASignOutput::ECDSASignOutput {
        signature: crate::standard_library_conversions::oblob_to_dafny(&value.signature).Extract(),
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::ECDSASignOutput,
    >,
) -> crate::operation::ecdsa_sign::EcdsaSignOutput {
    crate::operation::ecdsa_sign::EcdsaSignOutput::builder()
        .set_signature(Some(crate::standard_library_conversions::blob_from_dafny(dafny_value.signature().clone())))
        .build()
        .unwrap()
}
