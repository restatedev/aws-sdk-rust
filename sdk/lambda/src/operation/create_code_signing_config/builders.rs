// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_code_signing_config::_create_code_signing_config_output::CreateCodeSigningConfigOutputBuilder;

pub use crate::operation::create_code_signing_config::_create_code_signing_config_input::CreateCodeSigningConfigInputBuilder;

impl CreateCodeSigningConfigInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_code_signing_config::CreateCodeSigningConfigOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_code_signing_config::CreateCodeSigningConfigError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_code_signing_config();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateCodeSigningConfig`.
///
/// <p>Creates a code signing configuration. A <a href="https://docs.aws.amazon.com/lambda/latest/dg/configuration-codesigning.html">code signing configuration</a> defines a list of allowed signing profiles and defines the code-signing validation policy (action to be taken if deployment validation checks fail).</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateCodeSigningConfigFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_code_signing_config::builders::CreateCodeSigningConfigInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_code_signing_config::CreateCodeSigningConfigOutput,
        crate::operation::create_code_signing_config::CreateCodeSigningConfigError,
    > for CreateCodeSigningConfigFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_code_signing_config::CreateCodeSigningConfigOutput,
            crate::operation::create_code_signing_config::CreateCodeSigningConfigError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateCodeSigningConfigFluentBuilder {
    /// Creates a new `CreateCodeSigningConfig`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateCodeSigningConfig as a reference.
    pub fn as_input(&self) -> &crate::operation::create_code_signing_config::builders::CreateCodeSigningConfigInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_code_signing_config::CreateCodeSigningConfigOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_code_signing_config::CreateCodeSigningConfigError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_code_signing_config::CreateCodeSigningConfig::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_code_signing_config::CreateCodeSigningConfig::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_code_signing_config::CreateCodeSigningConfigOutput,
        crate::operation::create_code_signing_config::CreateCodeSigningConfigError,
        Self,
    > {
        crate::client::customize::CustomizableOperation::new(self)
    }
    pub(crate) fn config_override(mut self, config_override: impl Into<crate::config::Builder>) -> Self {
        self.set_config_override(Some(config_override.into()));
        self
    }

    pub(crate) fn set_config_override(&mut self, config_override: Option<crate::config::Builder>) -> &mut Self {
        self.config_override = config_override;
        self
    }
    /// <p>Descriptive name for this code signing configuration.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>Descriptive name for this code signing configuration.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>Descriptive name for this code signing configuration.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_description()
    }
    /// <p>Signing profiles for this code signing configuration.</p>
    pub fn allowed_publishers(mut self, input: crate::types::AllowedPublishers) -> Self {
        self.inner = self.inner.allowed_publishers(input);
        self
    }
    /// <p>Signing profiles for this code signing configuration.</p>
    pub fn set_allowed_publishers(mut self, input: ::std::option::Option<crate::types::AllowedPublishers>) -> Self {
        self.inner = self.inner.set_allowed_publishers(input);
        self
    }
    /// <p>Signing profiles for this code signing configuration.</p>
    pub fn get_allowed_publishers(&self) -> &::std::option::Option<crate::types::AllowedPublishers> {
        self.inner.get_allowed_publishers()
    }
    /// <p>The code signing policies define the actions to take if the validation checks fail.</p>
    pub fn code_signing_policies(mut self, input: crate::types::CodeSigningPolicies) -> Self {
        self.inner = self.inner.code_signing_policies(input);
        self
    }
    /// <p>The code signing policies define the actions to take if the validation checks fail.</p>
    pub fn set_code_signing_policies(mut self, input: ::std::option::Option<crate::types::CodeSigningPolicies>) -> Self {
        self.inner = self.inner.set_code_signing_policies(input);
        self
    }
    /// <p>The code signing policies define the actions to take if the validation checks fail.</p>
    pub fn get_code_signing_policies(&self) -> &::std::option::Option<crate::types::CodeSigningPolicies> {
        self.inner.get_code_signing_policies()
    }
}