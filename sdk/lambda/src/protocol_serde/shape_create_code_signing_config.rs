// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_create_code_signing_config_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::create_code_signing_config::CreateCodeSigningConfigOutput,
    crate::operation::create_code_signing_config::CreateCodeSigningConfigError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::create_code_signing_config::CreateCodeSigningConfigError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(crate::operation::create_code_signing_config::CreateCodeSigningConfigError::unhandled(
                generic,
            ))
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "ServiceException" => crate::operation::create_code_signing_config::CreateCodeSigningConfigError::ServiceException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ServiceExceptionBuilder::default();
                output = crate::protocol_serde::shape_service_exception::de_service_exception_json_err(_response_body, output)
                    .map_err(crate::operation::create_code_signing_config::CreateCodeSigningConfigError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidParameterValueException" => {
            crate::operation::create_code_signing_config::CreateCodeSigningConfigError::InvalidParameterValueException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidParameterValueExceptionBuilder::default();
                    output = crate::protocol_serde::shape_invalid_parameter_value_exception::de_invalid_parameter_value_exception_json_err(
                        _response_body,
                        output,
                    )
                    .map_err(crate::operation::create_code_signing_config::CreateCodeSigningConfigError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        _ => crate::operation::create_code_signing_config::CreateCodeSigningConfigError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_create_code_signing_config_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::create_code_signing_config::CreateCodeSigningConfigOutput,
    crate::operation::create_code_signing_config::CreateCodeSigningConfigError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::create_code_signing_config::builders::CreateCodeSigningConfigOutputBuilder::default();
        output = crate::protocol_serde::shape_create_code_signing_config::de_create_code_signing_config(_response_body, output)
            .map_err(crate::operation::create_code_signing_config::CreateCodeSigningConfigError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        crate::serde_util::create_code_signing_config_output_output_correct_errors(output).build()
    })
}

pub fn ser_create_code_signing_config_input(
    input: &crate::operation::create_code_signing_config::CreateCodeSigningConfigInput,
) -> Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_create_code_signing_config_input::ser_create_code_signing_config_input_input(&mut object, input)?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}

pub(crate) fn de_create_code_signing_config(
    value: &[u8],
    mut builder: crate::operation::create_code_signing_config::builders::CreateCodeSigningConfigOutputBuilder,
) -> Result<
    crate::operation::create_code_signing_config::builders::CreateCodeSigningConfigOutputBuilder,
    ::aws_smithy_json::deserialize::error::DeserializeError,
> {
    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "CodeSigningConfig" => {
                    builder = builder.set_code_signing_config(crate::protocol_serde::shape_code_signing_config::de_code_signing_config(tokens)?);
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