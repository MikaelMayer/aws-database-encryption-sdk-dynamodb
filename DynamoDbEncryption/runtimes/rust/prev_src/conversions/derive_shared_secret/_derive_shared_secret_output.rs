// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::derive_shared_secret::DeriveSharedSecretOutput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::DeriveSharedSecretOutput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::DeriveSharedSecretOutput::DeriveSharedSecretOutput {
        sharedSecret: crate::standard_library_conversions::oblob_to_dafny(&value.shared_secret).Extract(),
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::DeriveSharedSecretOutput,
    >,
) -> crate::operation::derive_shared_secret::DeriveSharedSecretOutput {
    crate::operation::derive_shared_secret::DeriveSharedSecretOutput::builder()
        .set_shared_secret(Some(crate::standard_library_conversions::blob_from_dafny(dafny_value.sharedSecret().clone())))
        .build()
        .unwrap()
}
