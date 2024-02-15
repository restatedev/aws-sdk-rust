// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListVersionsByFunctionOutput {
    /// <p>The pagination token that's included if more results are available.</p>
    pub next_marker: ::std::option::Option<::std::string::String>,
    /// <p>A list of Lambda function versions.</p>
    pub versions: ::std::option::Option<::std::vec::Vec<crate::types::FunctionConfiguration>>,
    _request_id: Option<String>,
}
impl ListVersionsByFunctionOutput {
    /// <p>The pagination token that's included if more results are available.</p>
    pub fn next_marker(&self) -> ::std::option::Option<&str> {
        self.next_marker.as_deref()
    }
    /// <p>A list of Lambda function versions.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.versions.is_none()`.
    pub fn versions(&self) -> &[crate::types::FunctionConfiguration] {
        self.versions.as_deref().unwrap_or_default()
    }
}
impl ::aws_types::request_id::RequestId for ListVersionsByFunctionOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListVersionsByFunctionOutput {
    /// Creates a new builder-style object to manufacture [`ListVersionsByFunctionOutput`](crate::operation::list_versions_by_function::ListVersionsByFunctionOutput).
    pub fn builder() -> crate::operation::list_versions_by_function::builders::ListVersionsByFunctionOutputBuilder {
        crate::operation::list_versions_by_function::builders::ListVersionsByFunctionOutputBuilder::default()
    }
}

/// A builder for [`ListVersionsByFunctionOutput`](crate::operation::list_versions_by_function::ListVersionsByFunctionOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ListVersionsByFunctionOutputBuilder {
    pub(crate) next_marker: ::std::option::Option<::std::string::String>,
    pub(crate) versions: ::std::option::Option<::std::vec::Vec<crate::types::FunctionConfiguration>>,
    _request_id: Option<String>,
}
impl ListVersionsByFunctionOutputBuilder {
    /// <p>The pagination token that's included if more results are available.</p>
    pub fn next_marker(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_marker = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The pagination token that's included if more results are available.</p>
    pub fn set_next_marker(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_marker = input;
        self
    }
    /// <p>The pagination token that's included if more results are available.</p>
    pub fn get_next_marker(&self) -> &::std::option::Option<::std::string::String> {
        &self.next_marker
    }
    /// Appends an item to `versions`.
    ///
    /// To override the contents of this collection use [`set_versions`](Self::set_versions).
    ///
    /// <p>A list of Lambda function versions.</p>
    pub fn versions(mut self, input: crate::types::FunctionConfiguration) -> Self {
        let mut v = self.versions.unwrap_or_default();
        v.push(input);
        self.versions = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of Lambda function versions.</p>
    pub fn set_versions(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::FunctionConfiguration>>) -> Self {
        self.versions = input;
        self
    }
    /// <p>A list of Lambda function versions.</p>
    pub fn get_versions(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::FunctionConfiguration>> {
        &self.versions
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`ListVersionsByFunctionOutput`](crate::operation::list_versions_by_function::ListVersionsByFunctionOutput).
    pub fn build(self) -> crate::operation::list_versions_by_function::ListVersionsByFunctionOutput {
        crate::operation::list_versions_by_function::ListVersionsByFunctionOutput {
            next_marker: self.next_marker,
            versions: self.versions,
            _request_id: self._request_id,
        }
    }
}