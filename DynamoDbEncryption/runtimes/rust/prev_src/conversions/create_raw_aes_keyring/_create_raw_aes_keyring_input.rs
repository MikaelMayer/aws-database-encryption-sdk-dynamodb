// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::create_raw_aes_keyring::CreateRawAesKeyringInput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::CreateRawAesKeyringInput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::CreateRawAesKeyringInput::CreateRawAesKeyringInput {
        keyNamespace: crate::standard_library_conversions::ostring_to_dafny(&value.key_namespace) .Extract(),
 keyName: crate::standard_library_conversions::ostring_to_dafny(&value.key_name) .Extract(),
 wrappingKey: crate::standard_library_conversions::oblob_to_dafny(&value.wrapping_key).Extract(),
 wrappingAlg: material_providers::conversions::aes_wrapping_alg::to_dafny(value.wrapping_alg.clone().unwrap()),
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::CreateRawAesKeyringInput,
    >,
) -> crate::operation::create_raw_aes_keyring::CreateRawAesKeyringInput {
    crate::operation::create_raw_aes_keyring::CreateRawAesKeyringInput::builder()
        .set_key_namespace(Some( dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(dafny_value.keyNamespace()) ))
 .set_key_name(Some( dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(dafny_value.keyName()) ))
 .set_wrapping_key(Some(crate::standard_library_conversions::blob_from_dafny(dafny_value.wrappingKey().clone())))
 .set_wrapping_alg(Some( material_providers::conversions::aes_wrapping_alg::from_dafny(dafny_value.wrappingAlg()) ))
        .build()
        .unwrap()
}
