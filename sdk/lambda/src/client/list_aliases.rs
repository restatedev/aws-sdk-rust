// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListAliases`](crate::operation::list_aliases::builders::ListAliasesFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_aliases::builders::ListAliasesFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`function_name(impl Into<String>)`](crate::operation::list_aliases::builders::ListAliasesFluentBuilder::function_name) / [`set_function_name(Option<String>)`](crate::operation::list_aliases::builders::ListAliasesFluentBuilder::set_function_name):<br>required: **true**<br><p>The name of the Lambda function.</p> <p class="title"><b>Name formats</b></p> <ul>  <li>   <p><b>Function name</b> - <code>MyFunction</code>.</p></li>  <li>   <p><b>Function ARN</b> - <code>arn:aws:lambda:us-west-2:123456789012:function:MyFunction</code>.</p></li>  <li>   <p><b>Partial ARN</b> - <code>123456789012:function:MyFunction</code>.</p></li> </ul> <p>The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p><br>
    ///   - [`function_version(impl Into<String>)`](crate::operation::list_aliases::builders::ListAliasesFluentBuilder::function_version) / [`set_function_version(Option<String>)`](crate::operation::list_aliases::builders::ListAliasesFluentBuilder::set_function_version):<br>required: **false**<br><p>Specify a function version to only list aliases that invoke that version.</p><br>
    ///   - [`marker(impl Into<String>)`](crate::operation::list_aliases::builders::ListAliasesFluentBuilder::marker) / [`set_marker(Option<String>)`](crate::operation::list_aliases::builders::ListAliasesFluentBuilder::set_marker):<br>required: **false**<br><p>Specify the pagination token that's returned by a previous request to retrieve the next page of results.</p><br>
    ///   - [`max_items(i32)`](crate::operation::list_aliases::builders::ListAliasesFluentBuilder::max_items) / [`set_max_items(Option<i32>)`](crate::operation::list_aliases::builders::ListAliasesFluentBuilder::set_max_items):<br>required: **false**<br><p>Limit the number of aliases returned.</p><br>
    /// - On success, responds with [`ListAliasesOutput`](crate::operation::list_aliases::ListAliasesOutput) with field(s):
    ///   - [`next_marker(Option<String>)`](crate::operation::list_aliases::ListAliasesOutput::next_marker): <p>The pagination token that's included if more results are available.</p>
    ///   - [`aliases(Option<Vec::<AliasConfiguration>>)`](crate::operation::list_aliases::ListAliasesOutput::aliases): <p>A list of aliases.</p>
    /// - On failure, responds with [`SdkError<ListAliasesError>`](crate::operation::list_aliases::ListAliasesError)
    pub fn list_aliases(&self) -> crate::operation::list_aliases::builders::ListAliasesFluentBuilder {
        crate::operation::list_aliases::builders::ListAliasesFluentBuilder::new(self.handle.clone())
    }
}