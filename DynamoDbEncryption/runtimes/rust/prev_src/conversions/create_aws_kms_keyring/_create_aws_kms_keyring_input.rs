// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::create_aws_kms_keyring::CreateAwsKmsKeyringInput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::CreateAwsKmsKeyringInput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::CreateAwsKmsKeyringInput::CreateAwsKmsKeyringInput {
        kmsKeyId: crate::standard_library_conversions::ostring_to_dafny(&value.kms_key_id) .Extract(),
 kmsClient: kms::conversions::client::to_dafny(&value.kms_client.clone().unwrap())
,
 grantTokens: ::std::rc::Rc::new(match &value.grant_tokens {
    Some(x) => crate::r#_Wrappers_Compile::Option::Some { value :
        ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(x,
            |e| dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&e),
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
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::CreateAwsKmsKeyringInput,
    >,
) -> crate::operation::create_aws_kms_keyring::CreateAwsKmsKeyringInput {
    crate::operation::create_aws_kms_keyring::CreateAwsKmsKeyringInput::builder()
        .set_kms_key_id(Some( dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(dafny_value.kmsKeyId()) ))
 .set_kms_client(Some( kms::conversions::client::from_dafny(dafny_value.kmsClient().clone())
 ))
 .set_grant_tokens(match (*dafny_value.grantTokens()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(
            ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(value,
                |e| dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(e),
            )
        ),
    _ => None
}
)
        .build()
        .unwrap()
}