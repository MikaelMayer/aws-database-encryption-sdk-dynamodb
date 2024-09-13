// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::material_providers::types::cryptographic_materials_cache::CryptographicMaterialsCacheRef,
) -> ::dafny_runtime::Object<
  dyn crate::r#software::amazon::cryptography::keystore::internaldafny::types::ICryptographicMaterialsCache,
> {
  let wrap = CryptographicMaterialsCacheWrapper {
      obj: value.clone(),
  };
  let inner = ::std::rc::Rc::new(::std::cell::UnsafeCell::new(wrap));
  ::dafny_runtime::Object (Some(inner) )
}

pub struct CryptographicMaterialsCacheWrapper {
  obj: crate::material_providers::types::cryptographic_materials_cache::CryptographicMaterialsCacheRef,
}

impl ::dafny_runtime::UpcastObject<dyn ::std::any::Any> for CryptographicMaterialsCacheWrapper {
  ::dafny_runtime::UpcastObjectFn!(dyn ::std::any::Any);
}

#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Object<
      dyn crate::r#software::amazon::cryptography::keystore::internaldafny::types::ISimpleResource,
    >,
) -> crate::material_providers::types::cryptographic_materials_cache::CryptographicMaterialsCacheRef {
    let wrap = ICryptographicMaterialsCacheDafnyWrapper {
        obj: dafny_value.clone(),
    };
    crate::material_providers::types::cryptographic_materials_cache::CryptographicMaterialsCacheRef {
      inner: ::std::rc::Rc::new(::std::cell::RefCell::new(wrap))
    }
}

#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ICryptographicMaterialsCacheDafnyWrapper {
  pub(crate) obj: ::dafny_runtime::Object<
      dyn crate::r#software::amazon::cryptography::keystore::internaldafny::types::ICryptographicMaterialsCache,
  >,
}


impl crate::software::amazon::cryptography::keystore::internaldafny::types::ICryptographicMaterialsCache
  for CryptographicMaterialsCacheWrapper
{
  fn r#_PutCacheEntry_k(
      &mut self,
      input: &::std::rc::Rc<
      crate::r#software::amazon::cryptography::keystore::internaldafny::types::PutCacheEntryInput,
      >,
  ) -> ::std::rc::Rc<
      crate::Wrappers::Result<
          ::std::rc::Rc<
          crate::r#software::amazon::cryptography::keystore::internaldafny::types::Unit,
          >,
          ::std::rc::Rc<crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error>,
      >,
  >
  {
      let inner_input =
          crate::material_providers::conversions::put_cache_entry::_put_cache_entry_input::from_dafny(
              input.clone(),
          );
      let inner_result = self.obj.inner.borrow_mut().put_cache_entry(inner_input);
      let result = match inner_result {
          Ok(x) => crate::Wrappers::Result::Success {
              value: crate::material_providers::conversions::put_cache_entry::_put_cache_entry_output::to_dafny(
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

impl crate::material_providers::types::cryptographic_materials_cache::CryptographicMaterialsCache for ICryptographicMaterialsCacheDafnyWrapper {
  fn put_cache_entry(
      &mut self,
      input: crate::material_providers::operation::put_cache_entry::PutCacheEntryInput,
  ) -> Result<
      crate::material_providers::operation::put_cache_entry::Unit,
      crate::material_providers::types::error::Error,
  > {
      let inner_input =
          crate::material_providers::conversions::put_cache_entry::_put_cache_entry_input::to_dafny(input);
      let inner_result = ::dafny_runtime::md!(self.obj.clone()).PutCacheEntry(&inner_input);
      if matches!(
          inner_result.as_ref(),
          crate::Wrappers::Result::Success { .. }
      ) {
          Ok(
              crate::material_providers::conversions::put_cache_entry::_put_cache_entry_output::from_dafny(
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


impl crate::software::amazon::cryptography::keystore::internaldafny::types::ICryptographicMaterialsCache
  for CryptographicMaterialsCacheWrapper
{
  fn r#_UpdateUsageMetadata_k(
      &mut self,
      input: &::std::rc::Rc<
      crate::r#software::amazon::cryptography::keystore::internaldafny::types::UpdateUsageMetadataInput,
      >,
  ) -> ::std::rc::Rc<
      crate::Wrappers::Result<
          ::std::rc::Rc<
          crate::r#software::amazon::cryptography::keystore::internaldafny::types::Unit,
          >,
          ::std::rc::Rc<crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error>,
      >,
  >
  {
      let inner_input =
          crate::material_providers::conversions::update_usage_metadata::_update_usage_metadata_input::from_dafny(
              input.clone(),
          );
      let inner_result = self.obj.inner.borrow_mut().update_usage_metadata(inner_input);
      let result = match inner_result {
          Ok(x) => crate::Wrappers::Result::Success {
              value: crate::material_providers::conversions::update_usage_metadata::_update_usage_metadata_output::to_dafny(
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

impl crate::material_providers::types::cryptographic_materials_cache::CryptographicMaterialsCache for ICryptographicMaterialsCacheDafnyWrapper {
  fn update_usage_metadata(
      &mut self,
      input: crate::material_providers::operation::update_usage_metadata::UpdateUsageMetadataInput,
  ) -> Result<
      crate::material_providers::operation::update_usage_metadata::Unit,
      crate::material_providers::types::error::Error,
  > {
      let inner_input =
          crate::material_providers::conversions::update_usage_metadata::_update_usage_metadata_input::to_dafny(input);
      let inner_result = ::dafny_runtime::md!(self.obj.clone()).UpdateUsageMetadata(&inner_input);
      if matches!(
          inner_result.as_ref(),
          crate::Wrappers::Result::Success { .. }
      ) {
          Ok(
              crate::material_providers::conversions::update_usage_metadata::_update_usage_metadata_output::from_dafny(
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


impl crate::software::amazon::cryptography::keystore::internaldafny::types::ICryptographicMaterialsCache
  for CryptographicMaterialsCacheWrapper
{
  fn r#_GetCacheEntry_k(
      &mut self,
      input: &::std::rc::Rc<
      crate::r#software::amazon::cryptography::keystore::internaldafny::types::GetCacheEntryInput,
      >,
  ) -> ::std::rc::Rc<
      crate::Wrappers::Result<
          ::std::rc::Rc<
          crate::r#software::amazon::cryptography::keystore::internaldafny::types::GetCacheEntryOutput,
          >,
          ::std::rc::Rc<crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error>,
      >,
  >
  {
      let inner_input =
          crate::material_providers::conversions::get_cache_entry::_get_cache_entry_input::from_dafny(
              input.clone(),
          );
      let inner_result = self.obj.inner.borrow_mut().get_cache_entry(inner_input);
      let result = match inner_result {
          Ok(x) => crate::Wrappers::Result::Success {
              value: crate::material_providers::conversions::get_cache_entry::_get_cache_entry_output::to_dafny(
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

impl crate::material_providers::types::cryptographic_materials_cache::CryptographicMaterialsCache for ICryptographicMaterialsCacheDafnyWrapper {
  fn get_cache_entry(
      &mut self,
      input: crate::material_providers::operation::get_cache_entry::GetCacheEntryInput,
  ) -> Result<
      crate::material_providers::operation::get_cache_entry::GetCacheEntryOutput,
      crate::material_providers::types::error::Error,
  > {
      let inner_input =
          crate::material_providers::conversions::get_cache_entry::_get_cache_entry_input::to_dafny(input);
      let inner_result = ::dafny_runtime::md!(self.obj.clone()).GetCacheEntry(&inner_input);
      if matches!(
          inner_result.as_ref(),
          crate::Wrappers::Result::Success { .. }
      ) {
          Ok(
              crate::material_providers::conversions::get_cache_entry::_get_cache_entry_output::from_dafny(
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


impl crate::software::amazon::cryptography::keystore::internaldafny::types::ICryptographicMaterialsCache
  for CryptographicMaterialsCacheWrapper
{
  fn r#_DeleteCacheEntry_k(
      &mut self,
      input: &::std::rc::Rc<
      crate::r#software::amazon::cryptography::keystore::internaldafny::types::DeleteCacheEntryInput,
      >,
  ) -> ::std::rc::Rc<
      crate::Wrappers::Result<
          ::std::rc::Rc<
          crate::r#software::amazon::cryptography::keystore::internaldafny::types::Unit,
          >,
          ::std::rc::Rc<crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error>,
      >,
  >
  {
      let inner_input =
          crate::material_providers::conversions::delete_cache_entry::_delete_cache_entry_input::from_dafny(
              input.clone(),
          );
      let inner_result = self.obj.inner.borrow_mut().delete_cache_entry(inner_input);
      let result = match inner_result {
          Ok(x) => crate::Wrappers::Result::Success {
              value: crate::material_providers::conversions::delete_cache_entry::_delete_cache_entry_output::to_dafny(
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

impl crate::material_providers::types::cryptographic_materials_cache::CryptographicMaterialsCache for ICryptographicMaterialsCacheDafnyWrapper {
  fn delete_cache_entry(
      &mut self,
      input: crate::material_providers::operation::delete_cache_entry::DeleteCacheEntryInput,
  ) -> Result<
      crate::material_providers::operation::delete_cache_entry::Unit,
      crate::material_providers::types::error::Error,
  > {
      let inner_input =
          crate::material_providers::conversions::delete_cache_entry::_delete_cache_entry_input::to_dafny(input);
      let inner_result = ::dafny_runtime::md!(self.obj.clone()).DeleteCacheEntry(&inner_input);
      if matches!(
          inner_result.as_ref(),
          crate::Wrappers::Result::Success { .. }
      ) {
          Ok(
              crate::material_providers::conversions::delete_cache_entry::_delete_cache_entry_output::from_dafny(
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
