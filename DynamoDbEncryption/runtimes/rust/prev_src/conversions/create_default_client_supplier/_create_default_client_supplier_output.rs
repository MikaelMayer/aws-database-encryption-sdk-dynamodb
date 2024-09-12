// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::create_default_client_supplier::CreateDefaultClientSupplierOutput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::CreateDefaultClientSupplierOutput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::CreateDefaultClientSupplierOutput::CreateDefaultClientSupplierOutput {
        client: material_providers::conversions::client_supplier::to_dafny(value.client.clone().unwrap())
,
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::CreateDefaultClientSupplierOutput,
    >,
) -> crate::operation::create_default_client_supplier::CreateDefaultClientSupplierOutput {
    crate::operation::create_default_client_supplier::CreateDefaultClientSupplierOutput::builder()
        .set_client(Some( material_providers::conversions::client_supplier::from_dafny(dafny_value.client().clone())
 ))
        .build()
        .unwrap()
}
