// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.

pub trait ClientSupplier {
    fn get_client(
    &mut self,
    input: crate::material_providers::operation::get_client::GetClientInput,
  ) -> Result<
    crate::material_providers::operation::get_client::GetClientOutput,
    crate::material_providers::types::error::Error,
  >;
}

#[derive(::std::clone::Clone)]
pub struct ClientSupplierRef {
  pub inner: ::std::rc::Rc<std::cell::RefCell<dyn ClientSupplier>>
}

impl ::std::cmp::PartialEq for ClientSupplierRef {
    fn eq(&self, other: &ClientSupplierRef) -> bool {
        ::std::rc::Rc::ptr_eq(&self.inner, &other.inner)
    }
}

impl ::std::fmt::Debug for ClientSupplierRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<ClientSupplierRef>")
    }
}

mod get_client;