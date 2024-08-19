// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_kms::operation::get_public_key::GetPublicKeyInput
) -> ::std::rc::Rc<
    crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GetPublicKeyRequest,
>{
    ::std::rc::Rc::new(crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GetPublicKeyRequest::GetPublicKeyRequest {
        KeyId: crate::kms::standard_library_conversions::ostring_to_dafny(&value.key_id) .Extract(),
 GrantTokens: ::std::rc::Rc::new(match &value.grant_tokens {
    Some(x) => crate::Wrappers::Option::Some { value :
        ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(x,
            |e| dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&e),
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
        crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GetPublicKeyRequest,
    >,
    client: aws_sdk_kms::Client,
) -> aws_sdk_kms::operation::get_public_key::builders::GetPublicKeyFluentBuilder {
    client.get_public_key()
          .set_key_id(Some( dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(dafny_value.KeyId()) ))
 .set_grant_tokens(match (*dafny_value.GrantTokens()).as_ref() {
    crate::Wrappers::Option::Some { value } =>
        Some(
            ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(value,
                |e| dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(e),
            )
        ),
    _ => None
}
)
}