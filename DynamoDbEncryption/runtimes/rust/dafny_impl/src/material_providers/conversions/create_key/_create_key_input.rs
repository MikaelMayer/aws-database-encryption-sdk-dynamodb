// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::material_providers::operation::create_key::CreateKeyInput,
) -> ::std::rc::Rc<
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::CreateKeyInput,
>{
    ::std::rc::Rc::new(crate::r#software::amazon::cryptography::keystore::internaldafny::types::CreateKeyInput::CreateKeyInput {
        branchKeyIdentifier: crate::ddb::standard_library_conversions::ostring_to_dafny(&value.branch_key_identifier),
 encryptionContext:
::std::rc::Rc::new(match &value.encryption_context {
    Some(x) => crate::Wrappers::Option::Some { value :
        ::dafny_runtime::dafny_runtime_conversions::hashmap_to_dafny_map(x,
            |k| ::std::rc::Rc::new(dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(&k.as_bytes().to_vec(), |b| *b)),
            |v| ::std::rc::Rc::new(dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(&v.as_bytes().to_vec(), |b| *b)),
        )
    },
    None => crate::Wrappers::Option::None {}
})
,
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#software::amazon::cryptography::keystore::internaldafny::types::CreateKeyInput,
    >,
) -> crate::material_providers::operation::create_key::CreateKeyInput {
    crate::material_providers::operation::create_key::CreateKeyInput::builder()
        .set_branch_key_identifier(crate::ddb::standard_library_conversions::ostring_from_dafny(dafny_value.branchKeyIdentifier().clone()))
 .set_encryption_context(match (*dafny_value.encryptionContext()).as_ref() {
    crate::Wrappers::Option::Some { value } =>
        Some(
            ::dafny_runtime::dafny_runtime_conversions::dafny_map_to_hashmap(value,
                |k| ::std::string::String::from_utf8(dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(&k.as_ref(), |b| *b)).unwrap(),
                |v| ::std::string::String::from_utf8(dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(&v.as_ref(), |b| *b)).unwrap(),
            )
        ),
    _ => None
}
)
        .build()
        .unwrap()
}
