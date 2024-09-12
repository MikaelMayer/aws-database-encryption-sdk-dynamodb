// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::ecdsa_verify::EcdsaVerifyOutput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::ECDSAVerifyOutput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::ECDSAVerifyOutput::ECDSAVerifyOutput {
        success: crate::standard_library_conversions::obool_to_dafny(&value.success),
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::ECDSAVerifyOutput,
    >,
) -> crate::operation::ecdsa_verify::EcdsaVerifyOutput {
    crate::operation::ecdsa_verify::EcdsaVerifyOutput::builder()
        .set_success(Some( dafny_value.success() .clone() ))
        .build()
        .unwrap()
}
