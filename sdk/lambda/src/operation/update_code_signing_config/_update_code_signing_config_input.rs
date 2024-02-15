// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateCodeSigningConfigInput {
    /// <p>The The Amazon Resource Name (ARN) of the code signing configuration.</p>
    pub code_signing_config_arn: ::std::option::Option<::std::string::String>,
    /// <p>Descriptive name for this code signing configuration.</p>
    pub description: ::std::option::Option<::std::string::String>,
    /// <p>Signing profiles for this code signing configuration.</p>
    pub allowed_publishers: ::std::option::Option<crate::types::AllowedPublishers>,
    /// <p>The code signing policy.</p>
    pub code_signing_policies: ::std::option::Option<crate::types::CodeSigningPolicies>,
}
impl UpdateCodeSigningConfigInput {
    /// <p>The The Amazon Resource Name (ARN) of the code signing configuration.</p>
    pub fn code_signing_config_arn(&self) -> ::std::option::Option<&str> {
        self.code_signing_config_arn.as_deref()
    }
    /// <p>Descriptive name for this code signing configuration.</p>
    pub fn description(&self) -> ::std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p>Signing profiles for this code signing configuration.</p>
    pub fn allowed_publishers(&self) -> ::std::option::Option<&crate::types::AllowedPublishers> {
        self.allowed_publishers.as_ref()
    }
    /// <p>The code signing policy.</p>
    pub fn code_signing_policies(&self) -> ::std::option::Option<&crate::types::CodeSigningPolicies> {
        self.code_signing_policies.as_ref()
    }
}
impl UpdateCodeSigningConfigInput {
    /// Creates a new builder-style object to manufacture [`UpdateCodeSigningConfigInput`](crate::operation::update_code_signing_config::UpdateCodeSigningConfigInput).
    pub fn builder() -> crate::operation::update_code_signing_config::builders::UpdateCodeSigningConfigInputBuilder {
        crate::operation::update_code_signing_config::builders::UpdateCodeSigningConfigInputBuilder::default()
    }
}

/// A builder for [`UpdateCodeSigningConfigInput`](crate::operation::update_code_signing_config::UpdateCodeSigningConfigInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct UpdateCodeSigningConfigInputBuilder {
    pub(crate) code_signing_config_arn: ::std::option::Option<::std::string::String>,
    pub(crate) description: ::std::option::Option<::std::string::String>,
    pub(crate) allowed_publishers: ::std::option::Option<crate::types::AllowedPublishers>,
    pub(crate) code_signing_policies: ::std::option::Option<crate::types::CodeSigningPolicies>,
}
impl UpdateCodeSigningConfigInputBuilder {
    /// <p>The The Amazon Resource Name (ARN) of the code signing configuration.</p>
    /// This field is required.
    pub fn code_signing_config_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.code_signing_config_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The The Amazon Resource Name (ARN) of the code signing configuration.</p>
    pub fn set_code_signing_config_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.code_signing_config_arn = input;
        self
    }
    /// <p>The The Amazon Resource Name (ARN) of the code signing configuration.</p>
    pub fn get_code_signing_config_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.code_signing_config_arn
    }
    /// <p>Descriptive name for this code signing configuration.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Descriptive name for this code signing configuration.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// <p>Descriptive name for this code signing configuration.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        &self.description
    }
    /// <p>Signing profiles for this code signing configuration.</p>
    pub fn allowed_publishers(mut self, input: crate::types::AllowedPublishers) -> Self {
        self.allowed_publishers = ::std::option::Option::Some(input);
        self
    }
    /// <p>Signing profiles for this code signing configuration.</p>
    pub fn set_allowed_publishers(mut self, input: ::std::option::Option<crate::types::AllowedPublishers>) -> Self {
        self.allowed_publishers = input;
        self
    }
    /// <p>Signing profiles for this code signing configuration.</p>
    pub fn get_allowed_publishers(&self) -> &::std::option::Option<crate::types::AllowedPublishers> {
        &self.allowed_publishers
    }
    /// <p>The code signing policy.</p>
    pub fn code_signing_policies(mut self, input: crate::types::CodeSigningPolicies) -> Self {
        self.code_signing_policies = ::std::option::Option::Some(input);
        self
    }
    /// <p>The code signing policy.</p>
    pub fn set_code_signing_policies(mut self, input: ::std::option::Option<crate::types::CodeSigningPolicies>) -> Self {
        self.code_signing_policies = input;
        self
    }
    /// <p>The code signing policy.</p>
    pub fn get_code_signing_policies(&self) -> &::std::option::Option<crate::types::CodeSigningPolicies> {
        &self.code_signing_policies
    }
    /// Consumes the builder and constructs a [`UpdateCodeSigningConfigInput`](crate::operation::update_code_signing_config::UpdateCodeSigningConfigInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::update_code_signing_config::UpdateCodeSigningConfigInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::update_code_signing_config::UpdateCodeSigningConfigInput {
            code_signing_config_arn: self.code_signing_config_arn,
            description: self.description,
            allowed_publishers: self.allowed_publishers,
            code_signing_policies: self.code_signing_policies,
        })
    }
}