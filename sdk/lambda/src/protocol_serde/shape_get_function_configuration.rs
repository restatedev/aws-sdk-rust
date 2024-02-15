// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_get_function_configuration_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::get_function_configuration::GetFunctionConfigurationOutput,
    crate::operation::get_function_configuration::GetFunctionConfigurationError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::get_function_configuration::GetFunctionConfigurationError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(crate::operation::get_function_configuration::GetFunctionConfigurationError::unhandled(
                generic,
            ))
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "ResourceNotFoundException" => crate::operation::get_function_configuration::GetFunctionConfigurationError::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_function_configuration::GetFunctionConfigurationError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "TooManyRequestsException" => crate::operation::get_function_configuration::GetFunctionConfigurationError::TooManyRequestsException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::TooManyRequestsExceptionBuilder::default();
                output = crate::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_function_configuration::GetFunctionConfigurationError::unhandled)?;
                output = output.set_retry_after_seconds(
                    crate::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
                        crate::operation::get_function_configuration::GetFunctionConfigurationError::unhandled(
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
        }),
        "ServiceException" => crate::operation::get_function_configuration::GetFunctionConfigurationError::ServiceException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ServiceExceptionBuilder::default();
                output = crate::protocol_serde::shape_service_exception::de_service_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_function_configuration::GetFunctionConfigurationError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidParameterValueException" => {
            crate::operation::get_function_configuration::GetFunctionConfigurationError::InvalidParameterValueException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidParameterValueExceptionBuilder::default();
                    output = crate::protocol_serde::shape_invalid_parameter_value_exception::de_invalid_parameter_value_exception_json_err(
                        _response_body,
                        output,
                    )
                    .map_err(crate::operation::get_function_configuration::GetFunctionConfigurationError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        _ => crate::operation::get_function_configuration::GetFunctionConfigurationError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_function_configuration_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::get_function_configuration::GetFunctionConfigurationOutput,
    crate::operation::get_function_configuration::GetFunctionConfigurationError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::get_function_configuration::builders::GetFunctionConfigurationOutputBuilder::default();
        output = crate::protocol_serde::shape_get_function_configuration::de_get_function_configuration(_response_body, output)
            .map_err(crate::operation::get_function_configuration::GetFunctionConfigurationError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_get_function_configuration(
    value: &[u8],
    mut builder: crate::operation::get_function_configuration::builders::GetFunctionConfigurationOutputBuilder,
) -> Result<
    crate::operation::get_function_configuration::builders::GetFunctionConfigurationOutputBuilder,
    ::aws_smithy_json::deserialize::error::DeserializeError,
> {
    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "Architectures" => {
                    builder = builder.set_architectures(crate::protocol_serde::shape_architectures_list::de_architectures_list(tokens)?);
                }
                "CodeSha256" => {
                    builder = builder.set_code_sha256(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "CodeSize" => {
                    builder = builder.set_code_size(
                        ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                            .map(i64::try_from)
                            .transpose()?,
                    );
                }
                "DeadLetterConfig" => {
                    builder = builder.set_dead_letter_config(crate::protocol_serde::shape_dead_letter_config::de_dead_letter_config(tokens)?);
                }
                "Description" => {
                    builder = builder.set_description(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "Environment" => {
                    builder = builder.set_environment(crate::protocol_serde::shape_environment_response::de_environment_response(tokens)?);
                }
                "EphemeralStorage" => {
                    builder = builder.set_ephemeral_storage(crate::protocol_serde::shape_ephemeral_storage::de_ephemeral_storage(tokens)?);
                }
                "FileSystemConfigs" => {
                    builder =
                        builder.set_file_system_configs(crate::protocol_serde::shape_file_system_config_list::de_file_system_config_list(tokens)?);
                }
                "FunctionArn" => {
                    builder = builder.set_function_arn(
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
                "Handler" => {
                    builder = builder.set_handler(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "ImageConfigResponse" => {
                    builder =
                        builder.set_image_config_response(crate::protocol_serde::shape_image_config_response::de_image_config_response(tokens)?);
                }
                "KMSKeyArn" => {
                    builder = builder.set_kms_key_arn(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "LastModified" => {
                    builder = builder.set_last_modified(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "LastUpdateStatus" => {
                    builder = builder.set_last_update_status(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| crate::types::LastUpdateStatus::from(u.as_ref())))
                            .transpose()?,
                    );
                }
                "LastUpdateStatusReason" => {
                    builder = builder.set_last_update_status_reason(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "LastUpdateStatusReasonCode" => {
                    builder = builder.set_last_update_status_reason_code(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| crate::types::LastUpdateStatusReasonCode::from(u.as_ref())))
                            .transpose()?,
                    );
                }
                "Layers" => {
                    builder = builder.set_layers(crate::protocol_serde::shape_layers_reference_list::de_layers_reference_list(tokens)?);
                }
                "LoggingConfig" => {
                    builder = builder.set_logging_config(crate::protocol_serde::shape_logging_config::de_logging_config(tokens)?);
                }
                "MasterArn" => {
                    builder = builder.set_master_arn(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "MemorySize" => {
                    builder = builder.set_memory_size(
                        ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                            .map(i32::try_from)
                            .transpose()?,
                    );
                }
                "PackageType" => {
                    builder = builder.set_package_type(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| crate::types::PackageType::from(u.as_ref())))
                            .transpose()?,
                    );
                }
                "RevisionId" => {
                    builder = builder.set_revision_id(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "Role" => {
                    builder = builder.set_role(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "Runtime" => {
                    builder = builder.set_runtime(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| crate::types::Runtime::from(u.as_ref())))
                            .transpose()?,
                    );
                }
                "RuntimeVersionConfig" => {
                    builder =
                        builder.set_runtime_version_config(crate::protocol_serde::shape_runtime_version_config::de_runtime_version_config(tokens)?);
                }
                "SigningJobArn" => {
                    builder = builder.set_signing_job_arn(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "SigningProfileVersionArn" => {
                    builder = builder.set_signing_profile_version_arn(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "SnapStart" => {
                    builder = builder.set_snap_start(crate::protocol_serde::shape_snap_start_response::de_snap_start_response(tokens)?);
                }
                "State" => {
                    builder = builder.set_state(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| crate::types::State::from(u.as_ref())))
                            .transpose()?,
                    );
                }
                "StateReason" => {
                    builder = builder.set_state_reason(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "StateReasonCode" => {
                    builder = builder.set_state_reason_code(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| crate::types::StateReasonCode::from(u.as_ref())))
                            .transpose()?,
                    );
                }
                "Timeout" => {
                    builder = builder.set_timeout(
                        ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                            .map(i32::try_from)
                            .transpose()?,
                    );
                }
                "TracingConfig" => {
                    builder = builder.set_tracing_config(crate::protocol_serde::shape_tracing_config_response::de_tracing_config_response(tokens)?);
                }
                "Version" => {
                    builder = builder.set_version(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "VpcConfig" => {
                    builder = builder.set_vpc_config(crate::protocol_serde::shape_vpc_config_response::de_vpc_config_response(tokens)?);
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