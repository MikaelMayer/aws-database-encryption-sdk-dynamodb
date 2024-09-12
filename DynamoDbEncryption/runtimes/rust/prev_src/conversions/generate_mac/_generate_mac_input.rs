// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::generate_mac::GenerateMacRequest,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::GenerateMacRequest,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::GenerateMacRequest::GenerateMacRequest {
        Message: crate::standard_library_conversions::oblob_to_dafny(&value.message).Extract(),
 KeyId: crate::standard_library_conversions::ostring_to_dafny(&value.key_id) .Extract(),
 MacAlgorithm: kms::conversions::mac_algorithm_spec::to_dafny(value.mac_algorithm.clone().unwrap()),
 GrantTokens: ::std::rc::Rc::new(match &value.grant_tokens {
    Some(x) => crate::r#_Wrappers_Compile::Option::Some { value :
        ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(x,
            |e| dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&e),
        )
    },
    None => crate::r#_Wrappers_Compile::Option::None {}
})
,
 DryRun: crate::standard_library_conversions::obool_to_dafny(&value.dry_run),
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::GenerateMacRequest,
    >,
) -> crate::operation::generate_mac::GenerateMacRequest {
    crate::operation::generate_mac::GenerateMacRequest::builder()
        .set_message(Some(crate::standard_library_conversions::blob_from_dafny(dafny_value.Message().clone())))
 .set_key_id(Some( dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(dafny_value.KeyId()) ))
 .set_mac_algorithm(Some( kms::conversions::mac_algorithm_spec::from_dafny(dafny_value.MacAlgorithm()) ))
 .set_grant_tokens(match (*dafny_value.GrantTokens()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(
            ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(value,
                |e| dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(e),
            )
        ),
    _ => None
}
)
 .set_dry_run(crate::standard_library_conversions::obool_from_dafny(dafny_value.DryRun().clone()))
        .build()
        .unwrap()
}
