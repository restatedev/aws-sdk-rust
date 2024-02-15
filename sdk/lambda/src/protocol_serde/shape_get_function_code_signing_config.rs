// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_get_function_code_signing_config_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::get_function_code_signing_config::GetFunctionCodeSigningConfigOutput,
    crate::operation::get_function_code_signing_config::GetFunctionCodeSigningConfigError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::get_function_code_signing_config::GetFunctionCodeSigningConfigError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::get_function_code_signing_config::GetFunctionCodeSigningConfigError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "ResourceNotFoundException" => {
            crate::operation::get_function_code_signing_config::GetFunctionCodeSigningConfigError::ResourceNotFoundException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                    output =
                        crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                            .map_err(crate::operation::get_function_code_signing_config::GetFunctionCodeSigningConfigError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "TooManyRequestsException" => {
            crate::operation::get_function_code_signing_config::GetFunctionCodeSigningConfigError::TooManyRequestsException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::TooManyRequestsExceptionBuilder::default();
                    output =
                        crate::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                            .map_err(crate::operation::get_function_code_signing_config::GetFunctionCodeSigningConfigError::unhandled)?;
                    output = output.set_retry_after_seconds(
                        crate::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
                            crate::operation::get_function_code_signing_config::GetFunctionCodeSigningConfigError::unhandled(
                                "Failed to parse retryAfterSeconds from header `Retry-After",
                            )
                        })?,
                    );
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "ServiceException" => crate::operation::get_function_code_signing_config::GetFunctionCodeSigningConfigError::ServiceException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ServiceExceptionBuilder::default();
                output = crate::protocol_serde::shape_service_exception::de_service_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_function_code_signing_config::GetFunctionCodeSigningConfigError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidParameterValueException" => {
            crate::operation::get_function_code_signing_config::GetFunctionCodeSigningConfigError::InvalidParameterValueException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidParameterValueExceptionBuilder::default();
                    output = crate::protocol_serde::shape_invalid_parameter_value_exception::de_invalid_parameter_value_exception_json_err(
                        _response_body,
                        output,
                    )
                    .map_err(crate::operation::get_function_code_signing_config::GetFunctionCodeSigningConfigError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        _ => crate::operation::get_function_code_signing_config::GetFunctionCodeSigningConfigError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_function_code_signing_config_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::get_function_code_signing_config::GetFunctionCodeSigningConfigOutput,
    crate::operation::get_function_code_signing_config::GetFunctionCodeSigningConfigError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::get_function_code_signing_config::builders::GetFunctionCodeSigningConfigOutputBuilder::default();
        output = crate::protocol_serde::shape_get_function_code_signing_config::de_get_function_code_signing_config(_response_body, output)
            .map_err(crate::operation::get_function_code_signing_config::GetFunctionCodeSigningConfigError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        crate::serde_util::get_function_code_signing_config_output_output_correct_errors(output)
            .build()
            .map_err(crate::operation::get_function_code_signing_config::GetFunctionCodeSigningConfigError::unhandled)?
    })
}

pub(crate) fn de_get_function_code_signing_config(
    value: &[u8],
    mut builder: crate::operation::get_function_code_signing_config::builders::GetFunctionCodeSigningConfigOutputBuilder,
) -> Result<
    crate::operation::get_function_code_signing_config::builders::GetFunctionCodeSigningConfigOutputBuilder,
    ::aws_smithy_json::deserialize::error::DeserializeError,
> {
    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "CodeSigningConfigArn" => {
                    builder = builder.set_code_signing_config_arn(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "FunctionName" => {
                    builder = builder.set_function_name(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
            },
            other => {
                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                    "expected object key or end object, found: {:?}",
                    other
                )))
            }
        }
    }
    if tokens.next().is_some() {
        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}