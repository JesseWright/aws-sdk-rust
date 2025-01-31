// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[derive(Debug)]
pub(crate) struct Handle<
    C = smithy_client::erase::DynConnector,
    M = aws_hyper::AwsMiddleware,
    R = smithy_client::retry::Standard,
> {
    client: smithy_client::Client<C, M, R>,
    conf: crate::Config,
}

/// An ergonomic service client for `AmazonSageMakerRuntime`.
///
/// This client allows ergonomic access to a `AmazonSageMakerRuntime`-shaped service.
/// Each method corresponds to an endpoint defined in the service's Smithy model,
/// and the request and response shapes are auto-generated from that same model.
///
/// # Using a Client
///
/// Once you have a client set up, you can access the service's endpoints
/// by calling the appropriate method on [`Client`]. Each such method
/// returns a request builder for that endpoint, with methods for setting
/// the various fields of the request. Once your request is complete, use
/// the `send` method to send the request. `send` returns a future, which
/// you then have to `.await` to get the service's response.
///
/// [builder pattern]: https://rust-lang.github.io/api-guidelines/type-safety.html#c-builder
/// [SigV4-signed requests]: https://docs.aws.amazon.com/general/latest/gr/signature-version-4.html
#[derive(std::fmt::Debug)]
pub struct Client<
    C = smithy_client::erase::DynConnector,
    M = aws_hyper::AwsMiddleware,
    R = smithy_client::retry::Standard,
> {
    handle: std::sync::Arc<Handle<C, M, R>>,
}

impl<C, M, R> std::clone::Clone for Client<C, M, R> {
    fn clone(&self) -> Self {
        Self {
            handle: self.handle.clone(),
        }
    }
}

#[doc(inline)]
pub use smithy_client::Builder;

impl<C, M, R> From<smithy_client::Client<C, M, R>> for Client<C, M, R> {
    fn from(client: smithy_client::Client<C, M, R>) -> Self {
        Self::with_config(client, crate::Config::builder().build())
    }
}

impl<C, M, R> Client<C, M, R> {
    pub fn with_config(client: smithy_client::Client<C, M, R>, conf: crate::Config) -> Self {
        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }

    pub fn conf(&self) -> &crate::Config {
        &self.handle.conf
    }
}
impl<C, M, R> Client<C, M, R>
where
    C: smithy_client::bounds::SmithyConnector,
    M: smithy_client::bounds::SmithyMiddleware<C>,
    R: smithy_client::retry::NewRequestPolicy,
{
    pub fn invoke_endpoint(&self) -> fluent_builders::InvokeEndpoint<C, M, R> {
        fluent_builders::InvokeEndpoint::new(self.handle.clone())
    }
    pub fn invoke_endpoint_async(&self) -> fluent_builders::InvokeEndpointAsync<C, M, R> {
        fluent_builders::InvokeEndpointAsync::new(self.handle.clone())
    }
}
pub mod fluent_builders {
    #[derive(std::fmt::Debug)]
    pub struct InvokeEndpoint<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::invoke_endpoint_input::Builder,
    }
    impl<C, M, R> InvokeEndpoint<C, M, R>
    where
        C: smithy_client::bounds::SmithyConnector,
        M: smithy_client::bounds::SmithyMiddleware<C>,
        R: smithy_client::retry::NewRequestPolicy,
    {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::InvokeEndpointOutput,
            smithy_http::result::SdkError<crate::error::InvokeEndpointError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::InvokeEndpointInputOperationOutputAlias,
                crate::output::InvokeEndpointOutput,
                crate::error::InvokeEndpointError,
                crate::input::InvokeEndpointInputOperationRetryAlias,
            >,
        {
            let input = self
                .inner
                .build()
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            let op = input
                .make_operation(&self.handle.conf)
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            self.handle.client.call(op).await
        }
        /// <p>The name of the endpoint that you specified when you created the endpoint using the
        /// <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/API_CreateEndpoint.html">CreateEndpoint</a> API. </p>
        pub fn endpoint_name(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.endpoint_name(inp);
            self
        }
        pub fn set_endpoint_name(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_endpoint_name(input);
            self
        }
        /// <p>Provides input data, in the format specified in the <code>ContentType</code>
        /// request header. Amazon SageMaker passes all of the data in the body to the model. </p>
        /// <p>For information about the format of the request body, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/cdf-inference.html">Common Data
        /// Formats-Inference</a>.</p>
        pub fn body(mut self, inp: smithy_types::Blob) -> Self {
            self.inner = self.inner.body(inp);
            self
        }
        pub fn set_body(mut self, input: std::option::Option<smithy_types::Blob>) -> Self {
            self.inner = self.inner.set_body(input);
            self
        }
        /// <p>The MIME type of the input data in the request body.</p>
        pub fn content_type(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.content_type(inp);
            self
        }
        pub fn set_content_type(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_content_type(input);
            self
        }
        /// <p>The desired MIME type of the inference in the response.</p>
        pub fn accept(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.accept(inp);
            self
        }
        pub fn set_accept(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_accept(input);
            self
        }
        /// <p>Provides additional information about a request for an inference submitted to a model
        /// hosted at an Amazon SageMaker endpoint. The information is an opaque value that is
        /// forwarded verbatim. You could use this value, for example, to provide an ID that you can
        /// use to track a request or to provide other metadata that a service endpoint was
        /// programmed to process. The value must consist of no more than 1024 visible US-ASCII
        /// characters as specified in <a href="https://tools.ietf.org/html/rfc7230#section-3.2.6">Section 3.3.6. Field Value
        /// Components</a> of the Hypertext Transfer Protocol (HTTP/1.1). </p>
        /// <p>The code in your model is responsible for setting or updating any custom attributes in
        /// the response. If your code does not set this value in the response, an empty value is
        /// returned. For example, if a custom attribute represents the trace ID, your model can
        /// prepend the custom attribute with <code>Trace ID:</code> in your post-processing
        /// function.</p>
        /// <p>This feature is currently supported in the AWS SDKs but not in the Amazon SageMaker Python
        /// SDK.</p>
        pub fn custom_attributes(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.custom_attributes(inp);
            self
        }
        pub fn set_custom_attributes(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_custom_attributes(input);
            self
        }
        /// <p>The model to request for inference when invoking a multi-model endpoint.</p>
        pub fn target_model(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.target_model(inp);
            self
        }
        pub fn set_target_model(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_target_model(input);
            self
        }
        /// <p>Specify the production variant to send the inference request to when invoking an
        /// endpoint that is running two or more variants. Note that this parameter overrides the
        /// default behavior for the endpoint, which is to distribute the invocation traffic based
        /// on the variant weights.</p>
        /// <p>For information about how to use variant targeting to perform a/b testing, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/model-ab-testing.html">Test models in
        /// production</a>
        /// </p>
        pub fn target_variant(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.target_variant(inp);
            self
        }
        pub fn set_target_variant(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_target_variant(input);
            self
        }
        /// <p>If the endpoint hosts multiple containers and is configured to use direct invocation,
        /// this parameter specifies the host name of the container to invoke.</p>
        pub fn target_container_hostname(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.target_container_hostname(inp);
            self
        }
        pub fn set_target_container_hostname(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_target_container_hostname(input);
            self
        }
        /// <p>If you provide a value, it is added to the captured data when you enable data capture
        /// on the endpoint. For information about data capture, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/model-monitor-data-capture.html">Capture
        /// Data</a>.</p>
        pub fn inference_id(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.inference_id(inp);
            self
        }
        pub fn set_inference_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_inference_id(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct InvokeEndpointAsync<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::invoke_endpoint_async_input::Builder,
    }
    impl<C, M, R> InvokeEndpointAsync<C, M, R>
    where
        C: smithy_client::bounds::SmithyConnector,
        M: smithy_client::bounds::SmithyMiddleware<C>,
        R: smithy_client::retry::NewRequestPolicy,
    {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::InvokeEndpointAsyncOutput,
            smithy_http::result::SdkError<crate::error::InvokeEndpointAsyncError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::InvokeEndpointAsyncInputOperationOutputAlias,
                crate::output::InvokeEndpointAsyncOutput,
                crate::error::InvokeEndpointAsyncError,
                crate::input::InvokeEndpointAsyncInputOperationRetryAlias,
            >,
        {
            let input = self
                .inner
                .build()
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            let op = input
                .make_operation(&self.handle.conf)
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            self.handle.client.call(op).await
        }
        /// <p>The name of the endpoint that you specified when you created the endpoint using
        /// the <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_CreateEndpoint.html">
        /// <code>CreateEndpoint</code>
        /// </a> API.</p>
        pub fn endpoint_name(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.endpoint_name(inp);
            self
        }
        pub fn set_endpoint_name(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_endpoint_name(input);
            self
        }
        /// <p>The MIME type of the input data in the request body.</p>
        pub fn content_type(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.content_type(inp);
            self
        }
        pub fn set_content_type(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_content_type(input);
            self
        }
        /// <p>The desired MIME type of the inference in the response.</p>
        pub fn accept(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.accept(inp);
            self
        }
        pub fn set_accept(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_accept(input);
            self
        }
        /// <p>Provides additional information about a request for an inference submitted to
        /// a model hosted at an Amazon SageMaker endpoint. The information is an opaque value that is
        /// forwarded verbatim. You could use this value, for example, to provide an ID that you
        /// can use to track a request or to provide other metadata that a service endpoint was
        /// programmed to process. The value must consist of no more than 1024
        /// visible US-ASCII characters as specified in
        /// <a href="https://datatracker.ietf.org/doc/html/rfc7230#section-3.2.6">Section 3.3.6.
        /// Field Value Components</a> of the Hypertext Transfer Protocol (HTTP/1.1). </p>
        /// <p>The code in your model is responsible for setting or updating any custom attributes
        /// in the response. If your code does not set this value in the response, an empty
        /// value is returned. For example, if a custom attribute represents the trace ID,
        /// your model can prepend the custom attribute with <code>Trace ID</code>: in your post-processing function. </p>
        /// <p>This feature is currently supported in the AWS SDKs but not in the Amazon SageMaker Python SDK. </p>
        pub fn custom_attributes(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.custom_attributes(inp);
            self
        }
        pub fn set_custom_attributes(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_custom_attributes(input);
            self
        }
        /// <p>The identifier for the inference request. Amazon SageMaker will generate an identifier for you
        /// if none is specified. </p>
        pub fn inference_id(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.inference_id(inp);
            self
        }
        pub fn set_inference_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_inference_id(input);
            self
        }
        /// <p>The Amazon S3 URI where the inference request payload is stored.</p>
        pub fn input_location(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.input_location(inp);
            self
        }
        pub fn set_input_location(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_input_location(input);
            self
        }
        /// <p>Maximum age in seconds a request can be in the queue before it is marked as expired.</p>
        pub fn request_ttl_seconds(mut self, inp: i32) -> Self {
            self.inner = self.inner.request_ttl_seconds(inp);
            self
        }
        pub fn set_request_ttl_seconds(mut self, input: std::option::Option<i32>) -> Self {
            self.inner = self.inner.set_request_ttl_seconds(input);
            self
        }
    }
}
impl<C> Client<C, aws_hyper::AwsMiddleware, smithy_client::retry::Standard> {
    pub fn from_conf_conn(conf: crate::Config, conn: C) -> Self {
        let client = aws_hyper::Client::new(conn);
        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }
}
impl
    Client<
        smithy_client::erase::DynConnector,
        aws_hyper::AwsMiddleware,
        smithy_client::retry::Standard,
    >
{
    #[cfg(any(feature = "rustls", feature = "native-tls"))]
    pub fn new(config: &aws_types::config::Config) -> Self {
        Self::from_conf(config.into())
    }

    #[cfg(any(feature = "rustls", feature = "native-tls"))]
    pub fn from_conf(conf: crate::Config) -> Self {
        let client = aws_hyper::Client::https();
        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }
}
