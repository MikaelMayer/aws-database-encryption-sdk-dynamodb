// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::decrypt_materials::DecryptMaterialsInput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::DecryptMaterialsInput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::DecryptMaterialsInput::DecryptMaterialsInput {
        algorithmSuiteId: material_providers::conversions::algorithm_suite_id::to_dafny(&value.algorithm_suite_id.clone().unwrap())
,
 commitmentPolicy: material_providers::conversions::commitment_policy::to_dafny(&value.commitment_policy.clone().unwrap())
,
 encryptedDataKeys: ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(&value.encrypted_data_keys.clone().unwrap(),
    |e| material_providers::conversions::encrypted_data_key::to_dafny(e.clone())
,
)
,
 encryptionContext: ::dafny_runtime::dafny_runtime_conversions::hashmap_to_dafny_map(&value.encryption_context.clone().unwrap(),
    |k| ::std::rc::Rc::new(dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(&k.as_bytes().to_vec(), |b| *b)),
    |v| ::std::rc::Rc::new(dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(&v.as_bytes().to_vec(), |b| *b)),
)
,
 reproducedEncryptionContext:
::std::rc::Rc::new(match &value.reproduced_encryption_context {
    Some(x) => crate::r#_Wrappers_Compile::Option::Some { value :
        ::dafny_runtime::dafny_runtime_conversions::hashmap_to_dafny_map(x,
            |k| ::std::rc::Rc::new(dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(&k.as_bytes().to_vec(), |b| *b)),
            |v| ::std::rc::Rc::new(dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(&v.as_bytes().to_vec(), |b| *b)),
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
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::DecryptMaterialsInput,
    >,
) -> crate::operation::decrypt_materials::DecryptMaterialsInput {
    crate::operation::decrypt_materials::DecryptMaterialsInput::builder()
        .set_algorithm_suite_id(Some( material_providers::conversions::algorithm_suite_id::from_dafny(dafny_value.algorithmSuiteId().clone())
 ))
 .set_commitment_policy(Some( material_providers::conversions::commitment_policy::from_dafny(dafny_value.commitmentPolicy().clone())
 ))
 .set_encrypted_data_keys(Some( ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(dafny_value.encryptedDataKeys(),
    |e| material_providers::conversions::encrypted_data_key::from_dafny(e.clone())
,
)
 ))
 .set_encryption_context(Some( ::dafny_runtime::dafny_runtime_conversions::dafny_map_to_hashmap(&dafny_value.encryptionContext(),
    |k| ::std::string::String::from_utf8(dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(&k.as_ref(), |b| *b)).unwrap(),
    |v| ::std::string::String::from_utf8(dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(&v.as_ref(), |b| *b)).unwrap(),
)
 ))
 .set_reproduced_encryption_context(match (*dafny_value.reproducedEncryptionContext()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
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
