// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::valid_encryption_materials_transition::ValidEncryptionMaterialsTransitionInput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::ValidEncryptionMaterialsTransitionInput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::ValidEncryptionMaterialsTransitionInput::ValidEncryptionMaterialsTransitionInput {
        start: material_providers::conversions::encryption_materials::to_dafny(&value.start.clone().unwrap())
,
 stop: material_providers::conversions::encryption_materials::to_dafny(&value.stop.clone().unwrap())
,
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::ValidEncryptionMaterialsTransitionInput,
    >,
) -> crate::operation::valid_encryption_materials_transition::ValidEncryptionMaterialsTransitionInput {
    crate::operation::valid_encryption_materials_transition::ValidEncryptionMaterialsTransitionInput::builder()
        .set_start(Some( material_providers::conversions::encryption_materials::from_dafny(dafny_value.start().clone())
 ))
 .set_stop(Some( material_providers::conversions::encryption_materials::from_dafny(dafny_value.stop().clone())
 ))
        .build()
        .unwrap()
}
