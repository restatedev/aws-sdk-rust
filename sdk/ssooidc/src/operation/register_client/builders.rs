// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::register_client::_register_client_output::RegisterClientOutputBuilder;

pub use crate::operation::register_client::_register_client_input::RegisterClientInputBuilder;

impl RegisterClientInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::register_client::RegisterClientOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::register_client::RegisterClientError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.register_client();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `RegisterClient`.
///
/// <p>Registers a client with IAM Identity Center. This allows clients to initiate device authorization. The output should be persisted for reuse through many authentication requests.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct RegisterClientFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::register_client::builders::RegisterClientInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::register_client::RegisterClientOutput,
        crate::operation::register_client::RegisterClientError,
    > for RegisterClientFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::register_client::RegisterClientOutput,
            crate::operation::register_client::RegisterClientError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl RegisterClientFluentBuilder {
    /// Creates a new `RegisterClient`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the RegisterClient as a reference.
    pub fn as_input(&self) -> &crate::operation::register_client::builders::RegisterClientInputBuilder {
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
        crate::operation::register_client::RegisterClientOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::register_client::RegisterClientError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::register_client::RegisterClient::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::register_client::RegisterClient::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::register_client::RegisterClientOutput,
        crate::operation::register_client::RegisterClientError,
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
    /// <p>The friendly name of the client.</p>
    pub fn client_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_name(input.into());
        self
    }
    /// <p>The friendly name of the client.</p>
    pub fn set_client_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_name(input);
        self
    }
    /// <p>The friendly name of the client.</p>
    pub fn get_client_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_name()
    }
    /// <p>The type of client. The service supports only <code>public</code> as a client type. Anything other than public will be rejected by the service.</p>
    pub fn client_type(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_type(input.into());
        self
    }
    /// <p>The type of client. The service supports only <code>public</code> as a client type. Anything other than public will be rejected by the service.</p>
    pub fn set_client_type(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_type(input);
        self
    }
    /// <p>The type of client. The service supports only <code>public</code> as a client type. Anything other than public will be rejected by the service.</p>
    pub fn get_client_type(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_type()
    }
    /// Appends an item to `scopes`.
    ///
    /// To override the contents of this collection use [`set_scopes`](Self::set_scopes).
    ///
    /// <p>The list of scopes that are defined by the client. Upon authorization, this list is used to restrict permissions when granting an access token.</p>
    pub fn scopes(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.scopes(input.into());
        self
    }
    /// <p>The list of scopes that are defined by the client. Upon authorization, this list is used to restrict permissions when granting an access token.</p>
    pub fn set_scopes(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_scopes(input);
        self
    }
    /// <p>The list of scopes that are defined by the client. Upon authorization, this list is used to restrict permissions when granting an access token.</p>
    pub fn get_scopes(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_scopes()
    }
}