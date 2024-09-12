// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::hkdf_expand::HkdfExpandOutput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::HkdfExpandOutput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::HkdfExpandOutput::HkdfExpandOutput {
        okm: crate::standard_library_conversions::oblob_to_dafny(&value.okm).Extract(),
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::HkdfExpandOutput,
    >,
) -> crate::operation::hkdf_expand::HkdfExpandOutput {
    crate::operation::hkdf_expand::HkdfExpandOutput::builder()
        .set_okm(Some(crate::standard_library_conversions::blob_from_dafny(dafny_value.okm().clone())))
        .build()
        .unwrap()
}
