// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::initialize_encryption_materials::EncryptionMaterials,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::EncryptionMaterials,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::EncryptionMaterials::EncryptionMaterials {
        algorithmSuite: material_providers::conversions::algorithm_suite_info::to_dafny(&value.algorithm_suite.clone().unwrap())
,
 encryptionContext: ::dafny_runtime::dafny_runtime_conversions::hashmap_to_dafny_map(&value.encryption_context.clone().unwrap(),
    |k| ::std::rc::Rc::new(dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(&k.as_bytes().to_vec(), |b| *b)),
    |v| ::std::rc::Rc::new(dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(&v.as_bytes().to_vec(), |b| *b)),
)
,
 encryptedDataKeys: ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(&value.encrypted_data_keys.clone().unwrap(),
    |e| material_providers::conversions::encrypted_data_key::to_dafny(e.clone())
,
)
,
 requiredEncryptionContextKeys: ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(&value.required_encryption_context_keys.clone().unwrap(),
    |e| ::std::rc::Rc::new(dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(&e.as_bytes().to_vec(), |b| *b)),
)
,
 plaintextDataKey: crate::standard_library_conversions::oblob_to_dafny(&value.plaintext_data_key),
 signingKey: crate::standard_library_conversions::oblob_to_dafny(&value.signing_key),
 symmetricSigningKeys: ::std::rc::Rc::new(match &value.symmetric_signing_keys {
    Some(x) => crate::r#_Wrappers_Compile::Option::Some { value :
        ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(x,
            |e| crate::standard_library_conversions::blob_to_dafny(&e),
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
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::EncryptionMaterials,
    >,
) -> crate::operation::initialize_encryption_materials::EncryptionMaterials {
    crate::operation::initialize_encryption_materials::EncryptionMaterials::builder()
        .set_algorithm_suite(Some( material_providers::conversions::algorithm_suite_info::from_dafny(dafny_value.algorithmSuite().clone())
 ))
 .set_encryption_context(Some( ::dafny_runtime::dafny_runtime_conversions::dafny_map_to_hashmap(&dafny_value.encryptionContext(),
    |k| ::std::string::String::from_utf8(dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(&k.as_ref(), |b| *b)).unwrap(),
    |v| ::std::string::String::from_utf8(dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(&v.as_ref(), |b| *b)).unwrap(),
)
 ))
 .set_encrypted_data_keys(Some( ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(dafny_value.encryptedDataKeys(),
    |e| material_providers::conversions::encrypted_data_key::from_dafny(e.clone())
,
)
 ))
 .set_required_encryption_context_keys(Some( ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(dafny_value.requiredEncryptionContextKeys(),
    |e| ::std::string::String::from_utf8(dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(&e.as_ref(), |b| *b)).unwrap(),
)
 ))
 .set_plaintext_data_key(crate::standard_library_conversions::oblob_from_dafny(dafny_value.plaintextDataKey().clone()))
 .set_signing_key(crate::standard_library_conversions::oblob_from_dafny(dafny_value.signingKey().clone()))
 .set_symmetric_signing_keys(match (*dafny_value.symmetricSigningKeys()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(
            ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(value,
                |e| crate::standard_library_conversions::blob_from_dafny(e.clone()),
            )
        ),
    _ => None
}
)
        .build()
        .unwrap()
}
