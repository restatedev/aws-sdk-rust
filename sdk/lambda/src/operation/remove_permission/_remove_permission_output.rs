// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct RemovePermissionOutput {
    _request_id: Option<String>,
}
impl ::aws_types::request_id::RequestId for RemovePermissionOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl RemovePermissionOutput {
    /// Creates a new builder-style object to manufacture [`RemovePermissionOutput`](crate::operation::remove_permission::RemovePermissionOutput).
    pub fn builder() -> crate::operation::remove_permission::builders::RemovePermissionOutputBuilder {
        crate::operation::remove_permission::builders::RemovePermissionOutputBuilder::default()
    }
}

/// A builder for [`RemovePermissionOutput`](crate::operation::remove_permission::RemovePermissionOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct RemovePermissionOutputBuilder {
    _request_id: Option<String>,
}
impl RemovePermissionOutputBuilder {
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`RemovePermissionOutput`](crate::operation::remove_permission::RemovePermissionOutput).
    pub fn build(self) -> crate::operation::remove_permission::RemovePermissionOutput {
        crate::operation::remove_permission::RemovePermissionOutput {
            _request_id: self._request_id,
        }
    }
}