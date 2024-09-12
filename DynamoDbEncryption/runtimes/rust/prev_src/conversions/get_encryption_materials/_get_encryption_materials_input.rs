// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::get_encryption_materials::GetEncryptionMaterialsInput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::GetEncryptionMaterialsInput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::GetEncryptionMaterialsInput::GetEncryptionMaterialsInput {
        encryptionContext: ::dafny_runtime::dafny_runtime_conversions::hashmap_to_dafny_map(&value.encryption_context.clone().unwrap(),
    |k| ::std::rc::Rc::new(dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(&k.as_bytes().to_vec(), |b| *b)),
    |v| ::std::rc::Rc::new(dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(&v.as_bytes().to_vec(), |b| *b)),
)
,
 commitmentPolicy: material_providers::conversions::commitment_policy::to_dafny(&value.commitment_policy.clone().unwrap())
,
 algorithmSuiteId: ::std::rc::Rc::new(match &value.algorithm_suite_id {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: material_providers::conversions::algorithm_suite_id::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 maxPlaintextLength: crate::standard_library_conversions::olong_to_dafny(&value.max_plaintext_length),
 requiredEncryptionContextKeys: ::std::rc::Rc::new(match &value.required_encryption_context_keys {
    Some(x) => crate::r#_Wrappers_Compile::Option::Some { value :
        ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(x,
            |e| ::std::rc::Rc::new(dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(&e.as_bytes().to_vec(), |b| *b)),
        )
    },
    None => crate::r#_Wrappers_Compile::Option::None {}
})
,
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::GetEncryptionMaterialsInput,
    >,
) -> crate::operation::get_encryption_materials::GetEncryptionMaterialsInput {
    crate::operation::get_encryption_materials::GetEncryptionMaterialsInput::builder()
        .set_encryption_context(Some( ::dafny_runtime::dafny_runtime_conversions::dafny_map_to_hashmap(&dafny_value.encryptionContext(),
    |k| ::std::string::String::from_utf8(dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(&k.as_ref(), |b| *b)).unwrap(),
    |v| ::std::string::String::from_utf8(dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(&v.as_ref(), |b| *b)).unwrap(),
)
 ))
 .set_commitment_policy(Some( material_providers::conversions::commitment_policy::from_dafny(dafny_value.commitmentPolicy().clone())
 ))
 .set_algorithm_suite_id(match (*dafny_value.algorithmSuiteId()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(material_providers::conversions::algorithm_suite_id::from_dafny(value.clone())),
    _ => None,
}
)
 .set_max_plaintext_length(crate::standard_library_conversions::olong_from_dafny(dafny_value.maxPlaintextLength().clone()))
 .set_required_encryption_context_keys(match (*dafny_value.requiredEncryptionContextKeys()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(
            ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(value,
                |e| ::std::string::String::from_utf8(dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(&e.as_ref(), |b| *b)).unwrap(),
            )
        ),
    _ => None
}
)
        .build()
        .unwrap()
}
