// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::primitives::operation::get_rsa_key_modulus_length::GetRsaKeyModulusLengthInput,
) -> ::std::rc::Rc<
    crate::r#software::amazon::cryptography::primitives::internaldafny::types::GetRSAKeyModulusLengthInput,
>{
    ::std::rc::Rc::new(crate::r#software::amazon::cryptography::primitives::internaldafny::types::GetRSAKeyModulusLengthInput::GetRSAKeyModulusLengthInput {
        publicKey: crate::ddb::standard_library_conversions::oblob_to_dafny(&value.public_key).Extract(),
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#software::amazon::cryptography::primitives::internaldafny::types::GetRSAKeyModulusLengthInput,
    >,
) -> crate::primitives::operation::get_rsa_key_modulus_length::GetRsaKeyModulusLengthInput {
    crate::primitives::operation::get_rsa_key_modulus_length::GetRsaKeyModulusLengthInput::builder()
        .set_public_key(Some(crate::ddb::standard_library_conversions::blob_from_dafny(dafny_value.publicKey().clone())))
        .build()
        .unwrap()
}
