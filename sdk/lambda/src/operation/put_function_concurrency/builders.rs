// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::put_function_concurrency::_put_function_concurrency_output::PutFunctionConcurrencyOutputBuilder;

pub use crate::operation::put_function_concurrency::_put_function_concurrency_input::PutFunctionConcurrencyInputBuilder;

impl PutFunctionConcurrencyInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::put_function_concurrency::PutFunctionConcurrencyOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::put_function_concurrency::PutFunctionConcurrencyError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.put_function_concurrency();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `PutFunctionConcurrency`.
///
/// <p>Sets the maximum number of simultaneous executions for a function, and reserves capacity for that concurrency level.</p>
/// <p>Concurrency settings apply to the function as a whole, including all published versions and the unpublished version. Reserving concurrency both ensures that your function has capacity to process the specified number of events simultaneously, and prevents it from scaling beyond that level. Use <code>GetFunction</code> to see the current setting for a function.</p>
/// <p>Use <code>GetAccountSettings</code> to see your Regional concurrency limit. You can reserve concurrency for as many functions as you like, as long as you leave at least 100 simultaneous executions unreserved for functions that aren't configured with a per-function limit. For more information, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/invocation-scaling.html">Lambda function scaling</a>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct PutFunctionConcurrencyFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::put_function_concurrency::builders::PutFunctionConcurrencyInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::put_function_concurrency::PutFunctionConcurrencyOutput,
        crate::operation::put_function_concurrency::PutFunctionConcurrencyError,
    > for PutFunctionConcurrencyFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::put_function_concurrency::PutFunctionConcurrencyOutput,
            crate::operation::put_function_concurrency::PutFunctionConcurrencyError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl PutFunctionConcurrencyFluentBuilder {
    /// Creates a new `PutFunctionConcurrency`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the PutFunctionConcurrency as a reference.
    pub fn as_input(&self) -> &crate::operation::put_function_concurrency::builders::PutFunctionConcurrencyInputBuilder {
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
        crate::operation::put_function_concurrency::PutFunctionConcurrencyOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::put_function_concurrency::PutFunctionConcurrencyError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::put_function_concurrency::PutFunctionConcurrency::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::put_function_concurrency::PutFunctionConcurrency::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::put_function_concurrency::PutFunctionConcurrencyOutput,
        crate::operation::put_function_concurrency::PutFunctionConcurrencyError,
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
    /// <p>The name of the Lambda function.</p>
    /// <p class="title"><b>Name formats</b></p>
    /// <ul>
    /// <li>
    /// <p><b>Function name</b> – <code>my-function</code>.</p></li>
    /// <li>
    /// <p><b>Function ARN</b> – <code>arn:aws:lambda:us-west-2:123456789012:function:my-function</code>.</p></li>
    /// <li>
    /// <p><b>Partial ARN</b> – <code>123456789012:function:my-function</code>.</p></li>
    /// </ul>
    /// <p>The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    pub fn function_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.function_name(input.into());
        self
    }
    /// <p>The name of the Lambda function.</p>
    /// <p class="title"><b>Name formats</b></p>
    /// <ul>
    /// <li>
    /// <p><b>Function name</b> – <code>my-function</code>.</p></li>
    /// <li>
    /// <p><b>Function ARN</b> – <code>arn:aws:lambda:us-west-2:123456789012:function:my-function</code>.</p></li>
    /// <li>
    /// <p><b>Partial ARN</b> – <code>123456789012:function:my-function</code>.</p></li>
    /// </ul>
    /// <p>The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    pub fn set_function_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_function_name(input);
        self
    }
    /// <p>The name of the Lambda function.</p>
    /// <p class="title"><b>Name formats</b></p>
    /// <ul>
    /// <li>
    /// <p><b>Function name</b> – <code>my-function</code>.</p></li>
    /// <li>
    /// <p><b>Function ARN</b> – <code>arn:aws:lambda:us-west-2:123456789012:function:my-function</code>.</p></li>
    /// <li>
    /// <p><b>Partial ARN</b> – <code>123456789012:function:my-function</code>.</p></li>
    /// </ul>
    /// <p>The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    pub fn get_function_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_function_name()
    }
    /// <p>The number of simultaneous executions to reserve for the function.</p>
    pub fn reserved_concurrent_executions(mut self, input: i32) -> Self {
        self.inner = self.inner.reserved_concurrent_executions(input);
        self
    }
    /// <p>The number of simultaneous executions to reserve for the function.</p>
    pub fn set_reserved_concurrent_executions(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_reserved_concurrent_executions(input);
        self
    }
    /// <p>The number of simultaneous executions to reserve for the function.</p>
    pub fn get_reserved_concurrent_executions(&self) -> &::std::option::Option<i32> {
        self.inner.get_reserved_concurrent_executions()
    }
}