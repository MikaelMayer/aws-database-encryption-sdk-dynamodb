// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::aes_kdf_counter_mode::AesKdfCtrInput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::AesKdfCtrInput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::AesKdfCtrInput::AesKdfCtrInput {
        ikm: crate::standard_library_conversions::oblob_to_dafny(&value.ikm).Extract(),
 expectedLength: value.expected_length.clone(),
 nonce: crate::standard_library_conversions::oblob_to_dafny(&value.nonce),
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::AesKdfCtrInput,
    >,
) -> crate::operation::aes_kdf_counter_mode::AesKdfCtrInput {
    crate::operation::aes_kdf_counter_mode::AesKdfCtrInput::builder()
        .set_ikm(Some(crate::standard_library_conversions::blob_from_dafny(dafny_value.ikm().clone())))
 .set_expected_length(crate::standard_library_conversions::oint_from_dafny(dafny_value.expectedLength().clone()))
 .set_nonce(crate::standard_library_conversions::oblob_from_dafny(dafny_value.nonce().clone()))
        .build()
        .unwrap()
}
