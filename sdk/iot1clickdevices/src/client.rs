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

/// An ergonomic service client for `IoT1ClickDevicesService`.
///
/// This client allows ergonomic access to a `IoT1ClickDevicesService`-shaped service.
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
    pub fn claim_devices_by_claim_code(&self) -> fluent_builders::ClaimDevicesByClaimCode<C, M, R> {
        fluent_builders::ClaimDevicesByClaimCode::new(self.handle.clone())
    }
    pub fn describe_device(&self) -> fluent_builders::DescribeDevice<C, M, R> {
        fluent_builders::DescribeDevice::new(self.handle.clone())
    }
    pub fn finalize_device_claim(&self) -> fluent_builders::FinalizeDeviceClaim<C, M, R> {
        fluent_builders::FinalizeDeviceClaim::new(self.handle.clone())
    }
    pub fn get_device_methods(&self) -> fluent_builders::GetDeviceMethods<C, M, R> {
        fluent_builders::GetDeviceMethods::new(self.handle.clone())
    }
    pub fn initiate_device_claim(&self) -> fluent_builders::InitiateDeviceClaim<C, M, R> {
        fluent_builders::InitiateDeviceClaim::new(self.handle.clone())
    }
    pub fn invoke_device_method(&self) -> fluent_builders::InvokeDeviceMethod<C, M, R> {
        fluent_builders::InvokeDeviceMethod::new(self.handle.clone())
    }
    pub fn list_device_events(&self) -> fluent_builders::ListDeviceEvents<C, M, R> {
        fluent_builders::ListDeviceEvents::new(self.handle.clone())
    }
    pub fn list_devices(&self) -> fluent_builders::ListDevices<C, M, R> {
        fluent_builders::ListDevices::new(self.handle.clone())
    }
    pub fn list_tags_for_resource(&self) -> fluent_builders::ListTagsForResource<C, M, R> {
        fluent_builders::ListTagsForResource::new(self.handle.clone())
    }
    pub fn tag_resource(&self) -> fluent_builders::TagResource<C, M, R> {
        fluent_builders::TagResource::new(self.handle.clone())
    }
    pub fn unclaim_device(&self) -> fluent_builders::UnclaimDevice<C, M, R> {
        fluent_builders::UnclaimDevice::new(self.handle.clone())
    }
    pub fn untag_resource(&self) -> fluent_builders::UntagResource<C, M, R> {
        fluent_builders::UntagResource::new(self.handle.clone())
    }
    pub fn update_device_state(&self) -> fluent_builders::UpdateDeviceState<C, M, R> {
        fluent_builders::UpdateDeviceState::new(self.handle.clone())
    }
}
pub mod fluent_builders {
    #[derive(std::fmt::Debug)]
    pub struct ClaimDevicesByClaimCode<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::claim_devices_by_claim_code_input::Builder,
    }
    impl<C, M, R> ClaimDevicesByClaimCode<C, M, R>
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
            crate::output::ClaimDevicesByClaimCodeOutput,
            smithy_http::result::SdkError<crate::error::ClaimDevicesByClaimCodeError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::ClaimDevicesByClaimCodeInputOperationOutputAlias,
                crate::output::ClaimDevicesByClaimCodeOutput,
                crate::error::ClaimDevicesByClaimCodeError,
                crate::input::ClaimDevicesByClaimCodeInputOperationRetryAlias,
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
        /// <p>The claim code, starting with "C-", as provided by the device manufacturer.</p>
        pub fn claim_code(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.claim_code(inp);
            self
        }
        pub fn set_claim_code(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_claim_code(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct DescribeDevice<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::describe_device_input::Builder,
    }
    impl<C, M, R> DescribeDevice<C, M, R>
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
            crate::output::DescribeDeviceOutput,
            smithy_http::result::SdkError<crate::error::DescribeDeviceError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::DescribeDeviceInputOperationOutputAlias,
                crate::output::DescribeDeviceOutput,
                crate::error::DescribeDeviceError,
                crate::input::DescribeDeviceInputOperationRetryAlias,
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
        /// <p>The unique identifier of the device.</p>
        pub fn device_id(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.device_id(inp);
            self
        }
        pub fn set_device_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_device_id(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct FinalizeDeviceClaim<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::finalize_device_claim_input::Builder,
    }
    impl<C, M, R> FinalizeDeviceClaim<C, M, R>
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
            crate::output::FinalizeDeviceClaimOutput,
            smithy_http::result::SdkError<crate::error::FinalizeDeviceClaimError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::FinalizeDeviceClaimInputOperationOutputAlias,
                crate::output::FinalizeDeviceClaimOutput,
                crate::error::FinalizeDeviceClaimError,
                crate::input::FinalizeDeviceClaimInputOperationRetryAlias,
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
        /// <p>The unique identifier of the device.</p>
        pub fn device_id(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.device_id(inp);
            self
        }
        pub fn set_device_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_device_id(input);
            self
        }
        /// Adds a key-value pair to `Tags`.
        ///
        /// To override the contents of this collection use [`set_tags`](Self::set_tags).
        /// <p>A collection of key/value pairs defining the resource tags. For example, {
        /// "tags": {"key1": "value1", "key2": "value2"} }. For more information, see <a href="https://aws.amazon.com/answers/account-management/aws-tagging-strategies/">AWS
        /// Tagging Strategies</a>.</p><p>
        /// </p>
        pub fn tags(
            mut self,
            k: impl Into<std::string::String>,
            v: impl Into<std::string::String>,
        ) -> Self {
            self.inner = self.inner.tags(k, v);
            self
        }
        pub fn set_tags(
            mut self,
            input: std::option::Option<
                std::collections::HashMap<std::string::String, std::string::String>,
            >,
        ) -> Self {
            self.inner = self.inner.set_tags(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct GetDeviceMethods<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::get_device_methods_input::Builder,
    }
    impl<C, M, R> GetDeviceMethods<C, M, R>
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
            crate::output::GetDeviceMethodsOutput,
            smithy_http::result::SdkError<crate::error::GetDeviceMethodsError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::GetDeviceMethodsInputOperationOutputAlias,
                crate::output::GetDeviceMethodsOutput,
                crate::error::GetDeviceMethodsError,
                crate::input::GetDeviceMethodsInputOperationRetryAlias,
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
        /// <p>The unique identifier of the device.</p>
        pub fn device_id(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.device_id(inp);
            self
        }
        pub fn set_device_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_device_id(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct InitiateDeviceClaim<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::initiate_device_claim_input::Builder,
    }
    impl<C, M, R> InitiateDeviceClaim<C, M, R>
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
            crate::output::InitiateDeviceClaimOutput,
            smithy_http::result::SdkError<crate::error::InitiateDeviceClaimError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::InitiateDeviceClaimInputOperationOutputAlias,
                crate::output::InitiateDeviceClaimOutput,
                crate::error::InitiateDeviceClaimError,
                crate::input::InitiateDeviceClaimInputOperationRetryAlias,
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
        /// <p>The unique identifier of the device.</p>
        pub fn device_id(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.device_id(inp);
            self
        }
        pub fn set_device_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_device_id(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct InvokeDeviceMethod<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::invoke_device_method_input::Builder,
    }
    impl<C, M, R> InvokeDeviceMethod<C, M, R>
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
            crate::output::InvokeDeviceMethodOutput,
            smithy_http::result::SdkError<crate::error::InvokeDeviceMethodError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::InvokeDeviceMethodInputOperationOutputAlias,
                crate::output::InvokeDeviceMethodOutput,
                crate::error::InvokeDeviceMethodError,
                crate::input::InvokeDeviceMethodInputOperationRetryAlias,
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
        /// <p>The unique identifier of the device.</p>
        pub fn device_id(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.device_id(inp);
            self
        }
        pub fn set_device_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_device_id(input);
            self
        }
        /// <p>The device method to invoke.</p>
        pub fn device_method(mut self, inp: crate::model::DeviceMethod) -> Self {
            self.inner = self.inner.device_method(inp);
            self
        }
        pub fn set_device_method(
            mut self,
            input: std::option::Option<crate::model::DeviceMethod>,
        ) -> Self {
            self.inner = self.inner.set_device_method(input);
            self
        }
        /// <p>A JSON encoded string containing the device method request parameters.</p>
        pub fn device_method_parameters(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.device_method_parameters(inp);
            self
        }
        pub fn set_device_method_parameters(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_device_method_parameters(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct ListDeviceEvents<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::list_device_events_input::Builder,
    }
    impl<C, M, R> ListDeviceEvents<C, M, R>
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
            crate::output::ListDeviceEventsOutput,
            smithy_http::result::SdkError<crate::error::ListDeviceEventsError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::ListDeviceEventsInputOperationOutputAlias,
                crate::output::ListDeviceEventsOutput,
                crate::error::ListDeviceEventsError,
                crate::input::ListDeviceEventsInputOperationRetryAlias,
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
        /// <p>The unique identifier of the device.</p>
        pub fn device_id(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.device_id(inp);
            self
        }
        pub fn set_device_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_device_id(input);
            self
        }
        /// <p>The start date for the device event query, in ISO8061 format. For example,
        /// 2018-03-28T15:45:12.880Z
        /// </p>
        pub fn from_time_stamp(mut self, inp: smithy_types::Instant) -> Self {
            self.inner = self.inner.from_time_stamp(inp);
            self
        }
        pub fn set_from_time_stamp(
            mut self,
            input: std::option::Option<smithy_types::Instant>,
        ) -> Self {
            self.inner = self.inner.set_from_time_stamp(input);
            self
        }
        /// <p>The maximum number of results to return per request. If not set, a default value of
        /// 100 is used.</p>
        pub fn max_results(mut self, inp: i32) -> Self {
            self.inner = self.inner.max_results(inp);
            self
        }
        pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
            self.inner = self.inner.set_max_results(input);
            self
        }
        /// <p>The token to retrieve the next set of results.</p>
        pub fn next_token(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.next_token(inp);
            self
        }
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_next_token(input);
            self
        }
        /// <p>The end date for the device event query, in ISO8061 format. For example,
        /// 2018-03-28T15:45:12.880Z
        /// </p>
        pub fn to_time_stamp(mut self, inp: smithy_types::Instant) -> Self {
            self.inner = self.inner.to_time_stamp(inp);
            self
        }
        pub fn set_to_time_stamp(
            mut self,
            input: std::option::Option<smithy_types::Instant>,
        ) -> Self {
            self.inner = self.inner.set_to_time_stamp(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct ListDevices<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::list_devices_input::Builder,
    }
    impl<C, M, R> ListDevices<C, M, R>
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
            crate::output::ListDevicesOutput,
            smithy_http::result::SdkError<crate::error::ListDevicesError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::ListDevicesInputOperationOutputAlias,
                crate::output::ListDevicesOutput,
                crate::error::ListDevicesError,
                crate::input::ListDevicesInputOperationRetryAlias,
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
        /// <p>The type of the device, such as "button".</p>
        pub fn device_type(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.device_type(inp);
            self
        }
        pub fn set_device_type(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_device_type(input);
            self
        }
        /// <p>The maximum number of results to return per request. If not set, a default value of
        /// 100 is used.</p>
        pub fn max_results(mut self, inp: i32) -> Self {
            self.inner = self.inner.max_results(inp);
            self
        }
        pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
            self.inner = self.inner.set_max_results(input);
            self
        }
        /// <p>The token to retrieve the next set of results.</p>
        pub fn next_token(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.next_token(inp);
            self
        }
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_next_token(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct ListTagsForResource<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::list_tags_for_resource_input::Builder,
    }
    impl<C, M, R> ListTagsForResource<C, M, R>
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
            crate::output::ListTagsForResourceOutput,
            smithy_http::result::SdkError<crate::error::ListTagsForResourceError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::ListTagsForResourceInputOperationOutputAlias,
                crate::output::ListTagsForResourceOutput,
                crate::error::ListTagsForResourceError,
                crate::input::ListTagsForResourceInputOperationRetryAlias,
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
        /// <p>The ARN of the resource.</p>
        pub fn resource_arn(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.resource_arn(inp);
            self
        }
        pub fn set_resource_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_resource_arn(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct TagResource<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::tag_resource_input::Builder,
    }
    impl<C, M, R> TagResource<C, M, R>
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
            crate::output::TagResourceOutput,
            smithy_http::result::SdkError<crate::error::TagResourceError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::TagResourceInputOperationOutputAlias,
                crate::output::TagResourceOutput,
                crate::error::TagResourceError,
                crate::input::TagResourceInputOperationRetryAlias,
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
        /// <p>The ARN of the resource.</p>
        pub fn resource_arn(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.resource_arn(inp);
            self
        }
        pub fn set_resource_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_resource_arn(input);
            self
        }
        /// Adds a key-value pair to `Tags`.
        ///
        /// To override the contents of this collection use [`set_tags`](Self::set_tags).
        /// <p>A collection of key/value pairs defining the resource tags. For example, {
        /// "tags": {"key1": "value1", "key2": "value2"} }. For more information, see <a href="https://aws.amazon.com/answers/account-management/aws-tagging-strategies/">AWS
        /// Tagging Strategies</a>.</p><p>
        /// </p>
        pub fn tags(
            mut self,
            k: impl Into<std::string::String>,
            v: impl Into<std::string::String>,
        ) -> Self {
            self.inner = self.inner.tags(k, v);
            self
        }
        pub fn set_tags(
            mut self,
            input: std::option::Option<
                std::collections::HashMap<std::string::String, std::string::String>,
            >,
        ) -> Self {
            self.inner = self.inner.set_tags(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct UnclaimDevice<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::unclaim_device_input::Builder,
    }
    impl<C, M, R> UnclaimDevice<C, M, R>
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
            crate::output::UnclaimDeviceOutput,
            smithy_http::result::SdkError<crate::error::UnclaimDeviceError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::UnclaimDeviceInputOperationOutputAlias,
                crate::output::UnclaimDeviceOutput,
                crate::error::UnclaimDeviceError,
                crate::input::UnclaimDeviceInputOperationRetryAlias,
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
        /// <p>The unique identifier of the device.</p>
        pub fn device_id(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.device_id(inp);
            self
        }
        pub fn set_device_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_device_id(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct UntagResource<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::untag_resource_input::Builder,
    }
    impl<C, M, R> UntagResource<C, M, R>
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
            crate::output::UntagResourceOutput,
            smithy_http::result::SdkError<crate::error::UntagResourceError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::UntagResourceInputOperationOutputAlias,
                crate::output::UntagResourceOutput,
                crate::error::UntagResourceError,
                crate::input::UntagResourceInputOperationRetryAlias,
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
        /// <p>The ARN of the resource.</p>
        pub fn resource_arn(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.resource_arn(inp);
            self
        }
        pub fn set_resource_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_resource_arn(input);
            self
        }
        /// Appends an item to `TagKeys`.
        ///
        /// To override the contents of this collection use [`set_tag_keys`](Self::set_tag_keys).
        /// <p>A collections of tag keys. For example, {"key1","key2"}</p>
        pub fn tag_keys(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.tag_keys(inp);
            self
        }
        pub fn set_tag_keys(
            mut self,
            input: std::option::Option<std::vec::Vec<std::string::String>>,
        ) -> Self {
            self.inner = self.inner.set_tag_keys(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct UpdateDeviceState<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::update_device_state_input::Builder,
    }
    impl<C, M, R> UpdateDeviceState<C, M, R>
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
            crate::output::UpdateDeviceStateOutput,
            smithy_http::result::SdkError<crate::error::UpdateDeviceStateError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::UpdateDeviceStateInputOperationOutputAlias,
                crate::output::UpdateDeviceStateOutput,
                crate::error::UpdateDeviceStateError,
                crate::input::UpdateDeviceStateInputOperationRetryAlias,
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
        /// <p>The unique identifier of the device.</p>
        pub fn device_id(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.device_id(inp);
            self
        }
        pub fn set_device_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_device_id(input);
            self
        }
        /// <p>If true, the device is enabled. If false, the device is
        /// disabled.</p>
        pub fn enabled(mut self, inp: bool) -> Self {
            self.inner = self.inner.enabled(inp);
            self
        }
        pub fn set_enabled(mut self, input: std::option::Option<bool>) -> Self {
            self.inner = self.inner.set_enabled(input);
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
