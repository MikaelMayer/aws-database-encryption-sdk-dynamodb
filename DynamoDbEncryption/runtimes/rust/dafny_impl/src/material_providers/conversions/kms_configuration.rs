// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::material_providers::types::KmsConfiguration,
) -> ::std::rc::Rc<
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::KMSConfiguration,
> {
    ::std::rc::Rc::new(match value {
        crate::material_providers::types::KmsConfiguration::KmsKeyArn(x) =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::KMSConfiguration::kmsKeyArn {
        kmsKeyArn: dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&x),
    },
crate::material_providers::types::KmsConfiguration::KmsMrKeyArn(x) =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::KMSConfiguration::kmsMRKeyArn {
        kmsMRKeyArn: dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&x),
    },
crate::material_providers::types::KmsConfiguration::Discovery(x) =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::KMSConfiguration::discovery {
        discovery: crate::material_providers::conversions::discovery::to_dafny(x.clone())
,
    },
crate::material_providers::types::KmsConfiguration::MrDiscovery(x) =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::KMSConfiguration::mrDiscovery {
        mrDiscovery: crate::material_providers::conversions::mr_discovery::to_dafny(x.clone())
,
    },
        crate::material_providers::types::KmsConfiguration::Unknown => unreachable!(),
    })
}

#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#software::amazon::cryptography::keystore::internaldafny::types::KMSConfiguration,
    >,
) -> crate::material_providers::types::KmsConfiguration {
    match &::std::rc::Rc::unwrap_or_clone(dafny_value) {
        crate::r#software::amazon::cryptography::keystore::internaldafny::types::KMSConfiguration::kmsKeyArn {
    kmsKeyArn: x @ _,
} => crate::material_providers::types::KmsConfiguration::KmsKeyArn(dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(x)),
crate::r#software::amazon::cryptography::keystore::internaldafny::types::KMSConfiguration::kmsMRKeyArn {
    kmsMRKeyArn: x @ _,
} => crate::material_providers::types::KmsConfiguration::KmsMrKeyArn(dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(x)),
crate::r#software::amazon::cryptography::keystore::internaldafny::types::KMSConfiguration::discovery {
    discovery: x @ _,
} => crate::material_providers::types::KmsConfiguration::Discovery(crate::material_providers::conversions::discovery::from_dafny(x.clone())
),
crate::r#software::amazon::cryptography::keystore::internaldafny::types::KMSConfiguration::mrDiscovery {
    mrDiscovery: x @ _,
} => crate::material_providers::types::KmsConfiguration::MrDiscovery(crate::material_providers::conversions::mr_discovery::from_dafny(x.clone())
),
    }
}
