// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::get_rsa_key_modulus_length::GetRsaKeyModulusLengthOutput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::GetRSAKeyModulusLengthOutput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::GetRSAKeyModulusLengthOutput::GetRSAKeyModulusLengthOutput {
        length: value.length.clone(),
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::GetRSAKeyModulusLengthOutput,
    >,
) -> crate::operation::get_rsa_key_modulus_length::GetRsaKeyModulusLengthOutput {
    crate::operation::get_rsa_key_modulus_length::GetRsaKeyModulusLengthOutput::builder()
        .set_length(crate::standard_library_conversions::oint_from_dafny(dafny_value.length().clone()))
        .build()
        .unwrap()
}
