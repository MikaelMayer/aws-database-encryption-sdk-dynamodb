// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.

pub trait BranchKeyIdSupplier {
    fn get_branch_key_id(
    &mut self,
    input: crate::operation::get_branch_key_id::GetBranchKeyIdInput,
  ) -> Result<
    crate::operation::get_branch_key_id::GetBranchKeyIdOutput,
    crate::types::error::Error,
  >;
}

#[derive(::std::clone::Clone)]
pub struct BranchKeyIdSupplierRef {
  pub inner: ::std::rc::Rc<std::cell::RefCell<dyn BranchKeyIdSupplier>>
}

impl ::std::cmp::PartialEq for BranchKeyIdSupplierRef {
    fn eq(&self, other: &BranchKeyIdSupplierRef) -> bool {
        ::std::rc::Rc::ptr_eq(&self.inner, &other.inner)
    }
}

impl ::std::fmt::Debug for BranchKeyIdSupplierRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<BranchKeyIdSupplierRef>")
    }
}

mod get_branch_key_id;