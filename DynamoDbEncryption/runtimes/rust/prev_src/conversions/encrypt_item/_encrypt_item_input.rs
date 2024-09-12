// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::encrypt_item::EncryptItemInput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::EncryptItemInput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::EncryptItemInput::EncryptItemInput {
        plaintextItem: ::dafny_runtime::dafny_runtime_conversions::hashmap_to_dafny_map(&value.plaintext_item.clone().unwrap(),
    |k| dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&k),
    |v| dynamodb::conversions::attribute_value::to_dafny(v.clone())
,
)
,
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::EncryptItemInput,
    >,
) -> crate::operation::encrypt_item::EncryptItemInput {
    crate::operation::encrypt_item::EncryptItemInput::builder()
        .set_plaintext_item(Some( ::dafny_runtime::dafny_runtime_conversions::dafny_map_to_hashmap(&dafny_value.plaintextItem(),
    |k| dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(k),
    |v| dynamodb::conversions::attribute_value::from_dafny(v.clone())
,
)
 ))
        .build()
        .unwrap()
}
