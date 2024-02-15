// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_layer_version_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::delete_layer_version::DeleteLayerVersionOutput,
    crate::operation::delete_layer_version::DeleteLayerVersionError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::delete_layer_version::DeleteLayerVersionError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::delete_layer_version::DeleteLayerVersionError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "TooManyRequestsException" => crate::operation::delete_layer_version::DeleteLayerVersionError::TooManyRequestsException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::TooManyRequestsExceptionBuilder::default();
                output = crate::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                    .map_err(crate::operation::delete_layer_version::DeleteLayerVersionError::unhandled)?;
                output = output.set_retry_after_seconds(
                    crate::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
                        crate::operation::delete_layer_version::DeleteLayerVersionError::unhandled(
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
        "ServiceException" => crate::operation::delete_layer_version::DeleteLayerVersionError::ServiceException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ServiceExceptionBuilder::default();
                output = crate::protocol_serde::shape_service_exception::de_service_exception_json_err(_response_body, output)
                    .map_err(crate::operation::delete_layer_version::DeleteLayerVersionError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::delete_layer_version::DeleteLayerVersionError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_layer_version_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::delete_layer_version::DeleteLayerVersionOutput,
    crate::operation::delete_layer_version::DeleteLayerVersionError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::delete_layer_version::builders::DeleteLayerVersionOutputBuilder::default();
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}