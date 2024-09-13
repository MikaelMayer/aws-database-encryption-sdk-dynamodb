// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.

pub trait CryptographicMaterialsManager {
    fn get_encryption_materials(
    &mut self,
    input: crate::operation::get_encryption_materials::GetEncryptionMaterialsInput,
  ) -> Result<
    crate::operation::get_encryption_materials::GetEncryptionMaterialsOutput,
    crate::types::error::Error,
  >;

  fn decrypt_materials(
    &mut self,
    input: crate::operation::decrypt_materials::DecryptMaterialsInput,
  ) -> Result<
    crate::operation::decrypt_materials::DecryptMaterialsOutput,
    crate::types::error::Error,
  >;
}

#[derive(::std::clone::Clone)]
pub struct CryptographicMaterialsManagerRef {
  pub inner: ::std::rc::Rc<std::cell::RefCell<dyn CryptographicMaterialsManager>>
}

impl ::std::cmp::PartialEq for CryptographicMaterialsManagerRef {
    fn eq(&self, other: &CryptographicMaterialsManagerRef) -> bool {
        ::std::rc::Rc::ptr_eq(&self.inner, &other.inner)
    }
}

impl ::std::fmt::Debug for CryptographicMaterialsManagerRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<CryptographicMaterialsManagerRef>")
    }
}

mod get_encryption_materials;

mod decrypt_materials;