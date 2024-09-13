// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::material_providers::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef,
) -> ::dafny_runtime::Object<
  dyn crate::r#software::amazon::cryptography::keystore::internaldafny::types::ICryptographicMaterialsManager,
> {
  let wrap = CryptographicMaterialsManagerWrapper {
      obj: value.clone(),
  };
  let inner = ::std::rc::Rc::new(::std::cell::UnsafeCell::new(wrap));
  ::dafny_runtime::Object (Some(inner) )
}

pub struct CryptographicMaterialsManagerWrapper {
  obj: crate::material_providers::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef,
}

impl ::dafny_runtime::UpcastObject<dyn ::std::any::Any> for CryptographicMaterialsManagerWrapper {
  ::dafny_runtime::UpcastObjectFn!(dyn ::std::any::Any);
}

#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Object<
      dyn crate::r#software::amazon::cryptography::keystore::internaldafny::types::ISimpleResource,
    >,
) -> crate::material_providers::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef {
    let wrap = ICryptographicMaterialsManagerDafnyWrapper {
        obj: dafny_value.clone(),
    };
    crate::material_providers::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef {
      inner: ::std::rc::Rc::new(::std::cell::RefCell::new(wrap))
    }
}

#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ICryptographicMaterialsManagerDafnyWrapper {
  pub(crate) obj: ::dafny_runtime::Object<
      dyn crate::r#software::amazon::cryptography::keystore::internaldafny::types::ICryptographicMaterialsManager,
  >,
}


impl crate::software::amazon::cryptography::keystore::internaldafny::types::ICryptographicMaterialsManager
  for CryptographicMaterialsManagerWrapper
{
  fn r#_GetEncryptionMaterials_k(
      &mut self,
      input: &::std::rc::Rc<
      crate::r#software::amazon::cryptography::keystore::internaldafny::types::GetEncryptionMaterialsInput,
      >,
  ) -> ::std::rc::Rc<
      crate::Wrappers::Result<
          ::std::rc::Rc<
          crate::r#software::amazon::cryptography::keystore::internaldafny::types::GetEncryptionMaterialsOutput,
          >,
          ::std::rc::Rc<crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error>,
      >,
  >
  {
      let inner_input =
          crate::material_providers::conversions::get_encryption_materials::_get_encryption_materials_input::from_dafny(
              input.clone(),
          );
      let inner_result = self.obj.inner.borrow_mut().get_encryption_materials(inner_input);
      let result = match inner_result {
          Ok(x) => crate::Wrappers::Result::Success {
              value: crate::material_providers::conversions::get_encryption_materials::_get_encryption_materials_output::to_dafny(
                  x,
              ),
          },
          Err(x) => crate::Wrappers::Result::Failure {
              error: crate::material_providers::conversions::error::to_dafny(x),
          },
      };
      ::std::rc::Rc::new(result)
  }
}

impl crate::material_providers::types::cryptographic_materials_manager::CryptographicMaterialsManager for ICryptographicMaterialsManagerDafnyWrapper {
  fn get_encryption_materials(
      &mut self,
      input: crate::material_providers::operation::get_encryption_materials::GetEncryptionMaterialsInput,
  ) -> Result<
      crate::material_providers::operation::get_encryption_materials::GetEncryptionMaterialsOutput,
      crate::material_providers::types::error::Error,
  > {
      let inner_input =
          crate::material_providers::conversions::get_encryption_materials::_get_encryption_materials_input::to_dafny(input);
      let inner_result = ::dafny_runtime::md!(self.obj.clone()).GetEncryptionMaterials(&inner_input);
      if matches!(
          inner_result.as_ref(),
          crate::Wrappers::Result::Success { .. }
      ) {
          Ok(
              crate::material_providers::conversions::get_encryption_materials::_get_encryption_materials_output::from_dafny(
                  inner_result.value().clone(),
              ),
          )
      } else {
          Err(crate::material_providers::conversions::error::from_dafny(
              inner_result.error().clone(),
          ))
      }
  }
}


impl crate::software::amazon::cryptography::keystore::internaldafny::types::ICryptographicMaterialsManager
  for CryptographicMaterialsManagerWrapper
{
  fn r#_DecryptMaterials_k(
      &mut self,
      input: &::std::rc::Rc<
      crate::r#software::amazon::cryptography::keystore::internaldafny::types::DecryptMaterialsInput,
      >,
  ) -> ::std::rc::Rc<
      crate::Wrappers::Result<
          ::std::rc::Rc<
          crate::r#software::amazon::cryptography::keystore::internaldafny::types::DecryptMaterialsOutput,
          >,
          ::std::rc::Rc<crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error>,
      >,
  >
  {
      let inner_input =
          crate::material_providers::conversions::decrypt_materials::_decrypt_materials_input::from_dafny(
              input.clone(),
          );
      let inner_result = self.obj.inner.borrow_mut().decrypt_materials(inner_input);
      let result = match inner_result {
          Ok(x) => crate::Wrappers::Result::Success {
              value: crate::material_providers::conversions::decrypt_materials::_decrypt_materials_output::to_dafny(
                  x,
              ),
          },
          Err(x) => crate::Wrappers::Result::Failure {
              error: crate::material_providers::conversions::error::to_dafny(x),
          },
      };
      ::std::rc::Rc::new(result)
  }
}

impl crate::material_providers::types::cryptographic_materials_manager::CryptographicMaterialsManager for ICryptographicMaterialsManagerDafnyWrapper {
  fn decrypt_materials(
      &mut self,
      input: crate::material_providers::operation::decrypt_materials::DecryptMaterialsInput,
  ) -> Result<
      crate::material_providers::operation::decrypt_materials::DecryptMaterialsOutput,
      crate::material_providers::types::error::Error,
  > {
      let inner_input =
          crate::material_providers::conversions::decrypt_materials::_decrypt_materials_input::to_dafny(input);
      let inner_result = ::dafny_runtime::md!(self.obj.clone()).DecryptMaterials(&inner_input);
      if matches!(
          inner_result.as_ref(),
          crate::Wrappers::Result::Success { .. }
      ) {
          Ok(
              crate::material_providers::conversions::decrypt_materials::_decrypt_materials_output::from_dafny(
                  inner_result.value().clone(),
              ),
          )
      } else {
          Err(crate::material_providers::conversions::error::from_dafny(
              inner_result.error().clone(),
          ))
      }
  }
}
