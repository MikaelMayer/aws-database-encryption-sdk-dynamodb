// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::digest::DigestOutput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::DigestOutput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::DigestOutput::DigestOutput {
        digest: crate::standard_library_conversions::oblob_to_dafny(&value.digest).Extract(),
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::DigestOutput,
    >,
) -> crate::operation::digest::DigestOutput {
    crate::operation::digest::DigestOutput::builder()
        .set_digest(Some(crate::standard_library_conversions::blob_from_dafny(dafny_value.digest().clone())))
        .build()
        .unwrap()
}
