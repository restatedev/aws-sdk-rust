// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateFunctionUrlConfig`](crate::operation::update_function_url_config::builders::UpdateFunctionUrlConfigFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`function_name(impl Into<String>)`](crate::operation::update_function_url_config::builders::UpdateFunctionUrlConfigFluentBuilder::function_name) / [`set_function_name(Option<String>)`](crate::operation::update_function_url_config::builders::UpdateFunctionUrlConfigFluentBuilder::set_function_name):<br>required: **true**<br><p>The name of the Lambda function.</p> <p class="title"><b>Name formats</b></p> <ul>  <li>   <p><b>Function name</b> – <code>my-function</code>.</p></li>  <li>   <p><b>Function ARN</b> – <code>arn:aws:lambda:us-west-2:123456789012:function:my-function</code>.</p></li>  <li>   <p><b>Partial ARN</b> – <code>123456789012:function:my-function</code>.</p></li> </ul> <p>The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p><br>
    ///   - [`qualifier(impl Into<String>)`](crate::operation::update_function_url_config::builders::UpdateFunctionUrlConfigFluentBuilder::qualifier) / [`set_qualifier(Option<String>)`](crate::operation::update_function_url_config::builders::UpdateFunctionUrlConfigFluentBuilder::set_qualifier):<br>required: **false**<br><p>The alias name.</p><br>
    ///   - [`auth_type(FunctionUrlAuthType)`](crate::operation::update_function_url_config::builders::UpdateFunctionUrlConfigFluentBuilder::auth_type) / [`set_auth_type(Option<FunctionUrlAuthType>)`](crate::operation::update_function_url_config::builders::UpdateFunctionUrlConfigFluentBuilder::set_auth_type):<br>required: **false**<br><p>The type of authentication that your function URL uses. Set to <code>AWS_IAM</code> if you want to restrict access to authenticated users only. Set to <code>NONE</code> if you want to bypass IAM authentication to create a public endpoint. For more information, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/urls-auth.html">Security and auth model for Lambda function URLs</a>.</p><br>
    ///   - [`cors(Cors)`](crate::operation::update_function_url_config::builders::UpdateFunctionUrlConfigFluentBuilder::cors) / [`set_cors(Option<Cors>)`](crate::operation::update_function_url_config::builders::UpdateFunctionUrlConfigFluentBuilder::set_cors):<br>required: **false**<br><p>The <a href="https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS">cross-origin resource sharing (CORS)</a> settings for your function URL.</p><br>
    ///   - [`invoke_mode(InvokeMode)`](crate::operation::update_function_url_config::builders::UpdateFunctionUrlConfigFluentBuilder::invoke_mode) / [`set_invoke_mode(Option<InvokeMode>)`](crate::operation::update_function_url_config::builders::UpdateFunctionUrlConfigFluentBuilder::set_invoke_mode):<br>required: **false**<br><p>Use one of the following options:</p> <ul>  <li>   <p><code>BUFFERED</code> – This is the default option. Lambda invokes your function using the <code>Invoke</code> API operation. Invocation results are available when the payload is complete. The maximum payload size is 6 MB.</p></li>  <li>   <p><code>RESPONSE_STREAM</code> – Your function streams payload results as they become available. Lambda invokes your function using the <code>InvokeWithResponseStream</code> API operation. The maximum response payload size is 20 MB, however, you can <a href="https://docs.aws.amazon.com/servicequotas/latest/userguide/request-quota-increase.html">request a quota increase</a>.</p></li> </ul><br>
    /// - On success, responds with [`UpdateFunctionUrlConfigOutput`](crate::operation::update_function_url_config::UpdateFunctionUrlConfigOutput) with field(s):
    ///   - [`function_url(String)`](crate::operation::update_function_url_config::UpdateFunctionUrlConfigOutput::function_url): <p>The HTTP URL endpoint for your function.</p>
    ///   - [`function_arn(String)`](crate::operation::update_function_url_config::UpdateFunctionUrlConfigOutput::function_arn): <p>The Amazon Resource Name (ARN) of your function.</p>
    ///   - [`auth_type(FunctionUrlAuthType)`](crate::operation::update_function_url_config::UpdateFunctionUrlConfigOutput::auth_type): <p>The type of authentication that your function URL uses. Set to <code>AWS_IAM</code> if you want to restrict access to authenticated users only. Set to <code>NONE</code> if you want to bypass IAM authentication to create a public endpoint. For more information, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/urls-auth.html">Security and auth model for Lambda function URLs</a>.</p>
    ///   - [`cors(Option<Cors>)`](crate::operation::update_function_url_config::UpdateFunctionUrlConfigOutput::cors): <p>The <a href="https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS">cross-origin resource sharing (CORS)</a> settings for your function URL.</p>
    ///   - [`creation_time(String)`](crate::operation::update_function_url_config::UpdateFunctionUrlConfigOutput::creation_time): <p>When the function URL was created, in <a href="https://www.w3.org/TR/NOTE-datetime">ISO-8601 format</a> (YYYY-MM-DDThh:mm:ss.sTZD).</p>
    ///   - [`last_modified_time(String)`](crate::operation::update_function_url_config::UpdateFunctionUrlConfigOutput::last_modified_time): <p>When the function URL configuration was last updated, in <a href="https://www.w3.org/TR/NOTE-datetime">ISO-8601 format</a> (YYYY-MM-DDThh:mm:ss.sTZD).</p>
    ///   - [`invoke_mode(Option<InvokeMode>)`](crate::operation::update_function_url_config::UpdateFunctionUrlConfigOutput::invoke_mode): <p>Use one of the following options:</p> <ul>  <li>   <p><code>BUFFERED</code> – This is the default option. Lambda invokes your function using the <code>Invoke</code> API operation. Invocation results are available when the payload is complete. The maximum payload size is 6 MB.</p></li>  <li>   <p><code>RESPONSE_STREAM</code> – Your function streams payload results as they become available. Lambda invokes your function using the <code>InvokeWithResponseStream</code> API operation. The maximum response payload size is 20 MB, however, you can <a href="https://docs.aws.amazon.com/servicequotas/latest/userguide/request-quota-increase.html">request a quota increase</a>.</p></li> </ul>
    /// - On failure, responds with [`SdkError<UpdateFunctionUrlConfigError>`](crate::operation::update_function_url_config::UpdateFunctionUrlConfigError)
    pub fn update_function_url_config(&self) -> crate::operation::update_function_url_config::builders::UpdateFunctionUrlConfigFluentBuilder {
        crate::operation::update_function_url_config::builders::UpdateFunctionUrlConfigFluentBuilder::new(self.handle.clone())
    }
}