// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::create_aws_kms_mrk_discovery_multi_keyring::CreateKeyringOutput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::CreateKeyringOutput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::CreateKeyringOutput::CreateKeyringOutput {
        keyring: material_providers::conversions::keyring::to_dafny(value.keyring.clone().unwrap())
,
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::CreateKeyringOutput,
    >,
) -> crate::operation::create_aws_kms_mrk_discovery_multi_keyring::CreateKeyringOutput {
    crate::operation::create_aws_kms_mrk_discovery_multi_keyring::CreateKeyringOutput::builder()
        .set_keyring(Some( material_providers::conversions::keyring::from_dafny(dafny_value.keyring().clone())
 ))
        .build()
        .unwrap()
}
