// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::material_providers::types::branch_key_id_supplier::BranchKeyIdSupplierRef,
) -> ::dafny_runtime::Object<
  dyn crate::r#software::amazon::cryptography::keystore::internaldafny::types::IBranchKeyIdSupplier,
> {
  let wrap = BranchKeyIdSupplierWrapper {
      obj: value.clone(),
  };
  let inner = ::std::rc::Rc::new(::std::cell::UnsafeCell::new(wrap));
  ::dafny_runtime::Object (Some(inner) )
}

pub struct BranchKeyIdSupplierWrapper {
  obj: crate::material_providers::types::branch_key_id_supplier::BranchKeyIdSupplierRef,
}

impl ::dafny_runtime::UpcastObject<dyn ::std::any::Any> for BranchKeyIdSupplierWrapper {
  ::dafny_runtime::UpcastObjectFn!(dyn ::std::any::Any);
}

#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Object<
      dyn crate::r#software::amazon::cryptography::keystore::internaldafny::types::ISimpleResource,
    >,
) -> crate::material_providers::types::branch_key_id_supplier::BranchKeyIdSupplierRef {
    let wrap = IBranchKeyIdSupplierDafnyWrapper {
        obj: dafny_value.clone(),
    };
    crate::material_providers::types::branch_key_id_supplier::BranchKeyIdSupplierRef {
      inner: ::std::rc::Rc::new(::std::cell::RefCell::new(wrap))
    }
}

#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct IBranchKeyIdSupplierDafnyWrapper {
  pub(crate) obj: ::dafny_runtime::Object<
      dyn crate::r#software::amazon::cryptography::keystore::internaldafny::types::IBranchKeyIdSupplier,
  >,
}


impl crate::software::amazon::cryptography::keystore::internaldafny::types::IBranchKeyIdSupplier
  for BranchKeyIdSupplierWrapper
{
  fn r#_GetBranchKeyId_k(
      &mut self,
      input: &::std::rc::Rc<
      crate::r#software::amazon::cryptography::keystore::internaldafny::types::GetBranchKeyIdInput,
      >,
  ) -> ::std::rc::Rc<
      crate::Wrappers::Result<
          ::std::rc::Rc<
          crate::r#software::amazon::cryptography::keystore::internaldafny::types::GetBranchKeyIdOutput,
          >,
          ::std::rc::Rc<crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error>,
      >,
  >
  {
      let inner_input =
          crate::material_providers::conversions::get_branch_key_id::_get_branch_key_id_input::from_dafny(
              input.clone(),
          );
      let inner_result = self.obj.inner.borrow_mut().get_branch_key_id(inner_input);
      let result = match inner_result {
          Ok(x) => crate::Wrappers::Result::Success {
              value: crate::material_providers::conversions::get_branch_key_id::_get_branch_key_id_output::to_dafny(
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

impl crate::material_providers::types::branch_key_id_supplier::BranchKeyIdSupplier for IBranchKeyIdSupplierDafnyWrapper {
  fn get_branch_key_id(
      &mut self,
      input: crate::material_providers::operation::get_branch_key_id::GetBranchKeyIdInput,
  ) -> Result<
      crate::material_providers::operation::get_branch_key_id::GetBranchKeyIdOutput,
      crate::material_providers::types::error::Error,
  > {
      let inner_input =
          crate::material_providers::conversions::get_branch_key_id::_get_branch_key_id_input::to_dafny(input);
      let inner_result = ::dafny_runtime::md!(self.obj.clone()).GetBranchKeyId(&inner_input);
      if matches!(
          inner_result.as_ref(),
          crate::Wrappers::Result::Success { .. }
      ) {
          Ok(
              crate::material_providers::conversions::get_branch_key_id::_get_branch_key_id_output::from_dafny(
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
