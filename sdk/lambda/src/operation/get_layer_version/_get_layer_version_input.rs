// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetLayerVersionInput {
    /// <p>The name or Amazon Resource Name (ARN) of the layer.</p>
    pub layer_name: ::std::option::Option<::std::string::String>,
    /// <p>The version number.</p>
    pub version_number: ::std::option::Option<i64>,
}
impl GetLayerVersionInput {
    /// <p>The name or Amazon Resource Name (ARN) of the layer.</p>
    pub fn layer_name(&self) -> ::std::option::Option<&str> {
        self.layer_name.as_deref()
    }
    /// <p>The version number.</p>
    pub fn version_number(&self) -> ::std::option::Option<i64> {
        self.version_number
    }
}
impl GetLayerVersionInput {
    /// Creates a new builder-style object to manufacture [`GetLayerVersionInput`](crate::operation::get_layer_version::GetLayerVersionInput).
    pub fn builder() -> crate::operation::get_layer_version::builders::GetLayerVersionInputBuilder {
        crate::operation::get_layer_version::builders::GetLayerVersionInputBuilder::default()
    }
}

/// A builder for [`GetLayerVersionInput`](crate::operation::get_layer_version::GetLayerVersionInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct GetLayerVersionInputBuilder {
    pub(crate) layer_name: ::std::option::Option<::std::string::String>,
    pub(crate) version_number: ::std::option::Option<i64>,
}
impl GetLayerVersionInputBuilder {
    /// <p>The name or Amazon Resource Name (ARN) of the layer.</p>
    /// This field is required.
    pub fn layer_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.layer_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name or Amazon Resource Name (ARN) of the layer.</p>
    pub fn set_layer_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.layer_name = input;
        self
    }
    /// <p>The name or Amazon Resource Name (ARN) of the layer.</p>
    pub fn get_layer_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.layer_name
    }
    /// <p>The version number.</p>
    /// This field is required.
    pub fn version_number(mut self, input: i64) -> Self {
        self.version_number = ::std::option::Option::Some(input);
        self
    }
    /// <p>The version number.</p>
    pub fn set_version_number(mut self, input: ::std::option::Option<i64>) -> Self {
        self.version_number = input;
        self
    }
    /// <p>The version number.</p>
    pub fn get_version_number(&self) -> &::std::option::Option<i64> {
        &self.version_number
    }
    /// Consumes the builder and constructs a [`GetLayerVersionInput`](crate::operation::get_layer_version::GetLayerVersionInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::get_layer_version::GetLayerVersionInput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::operation::get_layer_version::GetLayerVersionInput {
            layer_name: self.layer_name,
            version_number: self.version_number,
        })
    }
}