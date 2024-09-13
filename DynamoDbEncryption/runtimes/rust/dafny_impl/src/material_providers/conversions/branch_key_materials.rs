// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::material_providers::types::BranchKeyMaterials,
) -> ::std::rc::Rc<
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::BranchKeyMaterials,
> {
    ::std::rc::Rc::new(to_dafny_plain(value))
}

#[allow(dead_code)]
pub fn to_dafny_plain(
    value: crate::material_providers::types::BranchKeyMaterials,
) -> crate::r#software::amazon::cryptography::keystore::internaldafny::types::BranchKeyMaterials {
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::BranchKeyMaterials::BranchKeyMaterials {
        branchKeyIdentifier: crate::ddb::standard_library_conversions::ostring_to_dafny(&value.branch_key_identifier) .Extract(),
 branchKeyVersion: ::std::rc::Rc::new((match value.branch_key_version {
  Some(s) => crate::Wrappers::Option::Some { value: dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(&s.as_bytes().to_vec(), |b| *b) },
  None => crate::Wrappers::Option::None {},
}).Extract()),
 encryptionContext: ::dafny_runtime::dafny_runtime_conversions::hashmap_to_dafny_map(&value.encryption_context.clone().unwrap(),
    |k| ::std::rc::Rc::new(dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(&k.as_bytes().to_vec(), |b| *b)),
    |v| ::std::rc::Rc::new(dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(&v.as_bytes().to_vec(), |b| *b)),
)
,
 branchKey: crate::ddb::standard_library_conversions::oblob_to_dafny(&value.branch_key).Extract(),
    }
}

#[allow(dead_code)]
pub fn option_to_dafny(
  value: ::std::option::Option<crate::material_providers::types::BranchKeyMaterials>,
) -> ::std::rc::Rc<crate::Wrappers::Option<::std::rc::Rc<
  crate::r#software::amazon::cryptography::keystore::internaldafny::types::BranchKeyMaterials,
>>>{
    ::std::rc::Rc::new(match value {
        ::std::option::Option::None => crate::Wrappers::Option::None {},
        ::std::option::Option::Some(x) => crate::Wrappers::Option::Some {
            value: ::std::rc::Rc::new(to_dafny_plain(x)),
        },
    })
}

#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#software::amazon::cryptography::keystore::internaldafny::types::BranchKeyMaterials,
    >,
) -> crate::material_providers::types::BranchKeyMaterials {
    plain_from_dafny(&*dafny_value)
}

#[allow(dead_code)]
pub fn plain_from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::keystore::internaldafny::types::BranchKeyMaterials,
) -> crate::material_providers::types::BranchKeyMaterials {
    match dafny_value {
        crate::r#software::amazon::cryptography::keystore::internaldafny::types::BranchKeyMaterials::BranchKeyMaterials {..} =>
            crate::material_providers::types::BranchKeyMaterials::builder()
                .set_branch_key_identifier(Some( dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(dafny_value.branchKeyIdentifier()) ))
 .set_branch_key_version(Some(::std::string::String::from_utf8(dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(&dafny_value.branchKeyVersion().as_ref(), |b| *b)).unwrap()))
 .set_encryption_context(Some( ::dafny_runtime::dafny_runtime_conversions::dafny_map_to_hashmap(&dafny_value.encryptionContext(),
    |k| ::std::string::String::from_utf8(dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(&k.as_ref(), |b| *b)).unwrap(),
    |v| ::std::string::String::from_utf8(dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(&v.as_ref(), |b| *b)).unwrap(),
)
 ))
 .set_branch_key(Some(crate::ddb::standard_library_conversions::blob_from_dafny(dafny_value.branchKey().clone())))
                .build()
                .unwrap()
    }
}

#[allow(dead_code)]
pub fn option_from_dafny(
    dafny_value: ::std::rc::Rc<crate::Wrappers::Option<::std::rc::Rc<
        crate::r#software::amazon::cryptography::keystore::internaldafny::types::BranchKeyMaterials,
    >>>,
) -> ::std::option::Option<crate::material_providers::types::BranchKeyMaterials> {
    match &*dafny_value {
        crate::Wrappers::Option::Some { value } => {
            ::std::option::Option::Some(plain_from_dafny(value))
        }
        _ => ::std::option::Option::None,
    }
}
