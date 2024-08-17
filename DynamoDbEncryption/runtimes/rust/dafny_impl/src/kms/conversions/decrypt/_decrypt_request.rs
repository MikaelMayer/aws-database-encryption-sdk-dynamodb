// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_kms::operation::decrypt::DecryptInput,
) -> ::std::rc::Rc<
    crate::r#software::amazon::cryptography::services::kms::internaldafny::types::DecryptRequest,
> {
    ::std::rc::Rc::new(crate::r#software::amazon::cryptography::services::kms::internaldafny::types::DecryptRequest::DecryptRequest {
        CiphertextBlob: crate::kms::standard_library_conversions::oblob_to_dafny(&value.ciphertext_blob).Extract(),
 EncryptionContext:
::std::rc::Rc::new(match &value.encryption_context {
    Some(x) => crate::Wrappers::Option::Some { value :
        ::dafny_runtime::dafny_runtime_conversions::hashmap_to_dafny_map(x,
            |k| dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&k),
            |v| dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&v),
        )
    },
    None => crate::Wrappers::Option::None {}
})
,
 GrantTokens: ::std::rc::Rc::new(match &value.grant_tokens {
    Some(x) => crate::Wrappers::Option::Some { value :
        ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(x,
            |e| dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&e),
        )
    },
    None => crate::Wrappers::Option::None {}
})
,
 KeyId: crate::kms::standard_library_conversions::ostring_to_dafny(&value.key_id),
 EncryptionAlgorithm: ::std::rc::Rc::new(match &value.encryption_algorithm {
    Some(x) => crate::Wrappers::Option::Some { value: crate::kms::conversions::encryption_algorithm_spec::to_dafny(x.clone()) },
    None => crate::Wrappers::Option::None { }
})
,
 Recipient: ::std::rc::Rc::new(match &value.recipient {
    Some(x) => crate::Wrappers::Option::Some { value: crate::kms::conversions::recipient_info::to_dafny(&x) },
    None => crate::Wrappers::Option::None { }
})
,
 DryRun: crate::kms::standard_library_conversions::obool_to_dafny(&value.dry_run),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#software::amazon::cryptography::services::kms::internaldafny::types::DecryptRequest,
    >,
    client: aws_sdk_kms::Client,
) -> aws_sdk_kms::operation::decrypt::builders::DecryptFluentBuilder {
    client.decrypt()
          .set_ciphertext_blob(Some(crate::kms::standard_library_conversions::blob_from_dafny(dafny_value.CiphertextBlob().clone())))
 .set_encryption_context(match (*dafny_value.EncryptionContext()).as_ref() {
    crate::Wrappers::Option::Some { value } =>
        Some(
            ::dafny_runtime::dafny_runtime_conversions::dafny_map_to_hashmap(value,
                |k| dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(k),
                |v| dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(v),
            )
        ),
    _ => None
}
)
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
 .set_key_id(crate::kms::standard_library_conversions::ostring_from_dafny(dafny_value.KeyId().clone()))
 .set_encryption_algorithm(match &**dafny_value.EncryptionAlgorithm() {
    crate::Wrappers::Option::Some { value } => Some(
        crate::kms::conversions::encryption_algorithm_spec::from_dafny(value)
    ),
    _ => None,
}
)
 .set_recipient(match (*dafny_value.Recipient()).as_ref() {
    crate::Wrappers::Option::Some { value } =>
        Some(crate::kms::conversions::recipient_info::from_dafny(value.clone())),
    _ => None,
}
)
 .set_dry_run(crate::kms::standard_library_conversions::obool_from_dafny(dafny_value.DryRun().clone()))
}
