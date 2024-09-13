// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::material_providers::operation::create_key_store::CreateKeyStoreInput,
) -> ::std::rc::Rc<
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::CreateKeyStoreInput,
>{
    ::std::rc::Rc::new(crate::r#software::amazon::cryptography::keystore::internaldafny::types::CreateKeyStoreInput::CreateKeyStoreInput {

    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#software::amazon::cryptography::keystore::internaldafny::types::CreateKeyStoreInput,
    >,
) -> crate::material_providers::operation::create_key_store::CreateKeyStoreInput {
    crate::material_providers::operation::create_key_store::CreateKeyStoreInput::builder()

        .build()
        .unwrap()
}
