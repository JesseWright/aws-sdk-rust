// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// <p>Creates a discoverer.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateDiscoverer {
    _private: (),
}
impl CreateDiscoverer {
    /// Creates a new builder-style object to manufacture [`CreateDiscovererInput`](crate::input::CreateDiscovererInput)
    pub fn builder() -> crate::input::create_discoverer_input::Builder {
        crate::input::create_discoverer_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for CreateDiscoverer {
    type Output = std::result::Result<
        crate::output::CreateDiscovererOutput,
        crate::error::CreateDiscovererError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 201 {
            crate::operation_deser::parse_create_discoverer_error(response)
        } else {
            crate::operation_deser::parse_create_discoverer_response(response)
        }
    }
}

/// <p>Creates a registry.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateRegistry {
    _private: (),
}
impl CreateRegistry {
    /// Creates a new builder-style object to manufacture [`CreateRegistryInput`](crate::input::CreateRegistryInput)
    pub fn builder() -> crate::input::create_registry_input::Builder {
        crate::input::create_registry_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for CreateRegistry {
    type Output =
        std::result::Result<crate::output::CreateRegistryOutput, crate::error::CreateRegistryError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 201 {
            crate::operation_deser::parse_create_registry_error(response)
        } else {
            crate::operation_deser::parse_create_registry_response(response)
        }
    }
}

/// <p>Creates a schema definition.</p> <note><p>Inactive schemas will be deleted after two years.</p></note>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateSchema {
    _private: (),
}
impl CreateSchema {
    /// Creates a new builder-style object to manufacture [`CreateSchemaInput`](crate::input::CreateSchemaInput)
    pub fn builder() -> crate::input::create_schema_input::Builder {
        crate::input::create_schema_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for CreateSchema {
    type Output =
        std::result::Result<crate::output::CreateSchemaOutput, crate::error::CreateSchemaError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 201 {
            crate::operation_deser::parse_create_schema_error(response)
        } else {
            crate::operation_deser::parse_create_schema_response(response)
        }
    }
}

/// <p>Deletes a discoverer.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteDiscoverer {
    _private: (),
}
impl DeleteDiscoverer {
    /// Creates a new builder-style object to manufacture [`DeleteDiscovererInput`](crate::input::DeleteDiscovererInput)
    pub fn builder() -> crate::input::delete_discoverer_input::Builder {
        crate::input::delete_discoverer_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DeleteDiscoverer {
    type Output = std::result::Result<
        crate::output::DeleteDiscovererOutput,
        crate::error::DeleteDiscovererError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 204 {
            crate::operation_deser::parse_delete_discoverer_error(response)
        } else {
            crate::operation_deser::parse_delete_discoverer_response(response)
        }
    }
}

/// <p>Deletes a Registry.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteRegistry {
    _private: (),
}
impl DeleteRegistry {
    /// Creates a new builder-style object to manufacture [`DeleteRegistryInput`](crate::input::DeleteRegistryInput)
    pub fn builder() -> crate::input::delete_registry_input::Builder {
        crate::input::delete_registry_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DeleteRegistry {
    type Output =
        std::result::Result<crate::output::DeleteRegistryOutput, crate::error::DeleteRegistryError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 204 {
            crate::operation_deser::parse_delete_registry_error(response)
        } else {
            crate::operation_deser::parse_delete_registry_response(response)
        }
    }
}

/// <p>Delete the resource-based policy attached to the specified registry.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteResourcePolicy {
    _private: (),
}
impl DeleteResourcePolicy {
    /// Creates a new builder-style object to manufacture [`DeleteResourcePolicyInput`](crate::input::DeleteResourcePolicyInput)
    pub fn builder() -> crate::input::delete_resource_policy_input::Builder {
        crate::input::delete_resource_policy_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DeleteResourcePolicy {
    type Output = std::result::Result<
        crate::output::DeleteResourcePolicyOutput,
        crate::error::DeleteResourcePolicyError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 204 {
            crate::operation_deser::parse_delete_resource_policy_error(response)
        } else {
            crate::operation_deser::parse_delete_resource_policy_response(response)
        }
    }
}

/// <p>Delete a schema definition.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteSchema {
    _private: (),
}
impl DeleteSchema {
    /// Creates a new builder-style object to manufacture [`DeleteSchemaInput`](crate::input::DeleteSchemaInput)
    pub fn builder() -> crate::input::delete_schema_input::Builder {
        crate::input::delete_schema_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DeleteSchema {
    type Output =
        std::result::Result<crate::output::DeleteSchemaOutput, crate::error::DeleteSchemaError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 204 {
            crate::operation_deser::parse_delete_schema_error(response)
        } else {
            crate::operation_deser::parse_delete_schema_response(response)
        }
    }
}

/// <p>Delete the schema version definition</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteSchemaVersion {
    _private: (),
}
impl DeleteSchemaVersion {
    /// Creates a new builder-style object to manufacture [`DeleteSchemaVersionInput`](crate::input::DeleteSchemaVersionInput)
    pub fn builder() -> crate::input::delete_schema_version_input::Builder {
        crate::input::delete_schema_version_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DeleteSchemaVersion {
    type Output = std::result::Result<
        crate::output::DeleteSchemaVersionOutput,
        crate::error::DeleteSchemaVersionError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 204 {
            crate::operation_deser::parse_delete_schema_version_error(response)
        } else {
            crate::operation_deser::parse_delete_schema_version_response(response)
        }
    }
}

/// <p>Describe the code binding URI.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeCodeBinding {
    _private: (),
}
impl DescribeCodeBinding {
    /// Creates a new builder-style object to manufacture [`DescribeCodeBindingInput`](crate::input::DescribeCodeBindingInput)
    pub fn builder() -> crate::input::describe_code_binding_input::Builder {
        crate::input::describe_code_binding_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DescribeCodeBinding {
    type Output = std::result::Result<
        crate::output::DescribeCodeBindingOutput,
        crate::error::DescribeCodeBindingError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_code_binding_error(response)
        } else {
            crate::operation_deser::parse_describe_code_binding_response(response)
        }
    }
}

/// <p>Describes the discoverer.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeDiscoverer {
    _private: (),
}
impl DescribeDiscoverer {
    /// Creates a new builder-style object to manufacture [`DescribeDiscovererInput`](crate::input::DescribeDiscovererInput)
    pub fn builder() -> crate::input::describe_discoverer_input::Builder {
        crate::input::describe_discoverer_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DescribeDiscoverer {
    type Output = std::result::Result<
        crate::output::DescribeDiscovererOutput,
        crate::error::DescribeDiscovererError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_discoverer_error(response)
        } else {
            crate::operation_deser::parse_describe_discoverer_response(response)
        }
    }
}

/// <p>Describes the registry.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeRegistry {
    _private: (),
}
impl DescribeRegistry {
    /// Creates a new builder-style object to manufacture [`DescribeRegistryInput`](crate::input::DescribeRegistryInput)
    pub fn builder() -> crate::input::describe_registry_input::Builder {
        crate::input::describe_registry_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DescribeRegistry {
    type Output = std::result::Result<
        crate::output::DescribeRegistryOutput,
        crate::error::DescribeRegistryError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_registry_error(response)
        } else {
            crate::operation_deser::parse_describe_registry_response(response)
        }
    }
}

/// <p>Retrieve the schema definition.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeSchema {
    _private: (),
}
impl DescribeSchema {
    /// Creates a new builder-style object to manufacture [`DescribeSchemaInput`](crate::input::DescribeSchemaInput)
    pub fn builder() -> crate::input::describe_schema_input::Builder {
        crate::input::describe_schema_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DescribeSchema {
    type Output =
        std::result::Result<crate::output::DescribeSchemaOutput, crate::error::DescribeSchemaError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_schema_error(response)
        } else {
            crate::operation_deser::parse_describe_schema_response(response)
        }
    }
}

#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ExportSchema {
    _private: (),
}
impl ExportSchema {
    /// Creates a new builder-style object to manufacture [`ExportSchemaInput`](crate::input::ExportSchemaInput)
    pub fn builder() -> crate::input::export_schema_input::Builder {
        crate::input::export_schema_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ExportSchema {
    type Output =
        std::result::Result<crate::output::ExportSchemaOutput, crate::error::ExportSchemaError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_export_schema_error(response)
        } else {
            crate::operation_deser::parse_export_schema_response(response)
        }
    }
}

/// <p>Get the code binding source URI.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetCodeBindingSource {
    _private: (),
}
impl GetCodeBindingSource {
    /// Creates a new builder-style object to manufacture [`GetCodeBindingSourceInput`](crate::input::GetCodeBindingSourceInput)
    pub fn builder() -> crate::input::get_code_binding_source_input::Builder {
        crate::input::get_code_binding_source_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for GetCodeBindingSource {
    type Output = std::result::Result<
        crate::output::GetCodeBindingSourceOutput,
        crate::error::GetCodeBindingSourceError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_code_binding_source_error(response)
        } else {
            crate::operation_deser::parse_get_code_binding_source_response(response)
        }
    }
}

/// <p>Get the discovered schema that was generated based on sampled events.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetDiscoveredSchema {
    _private: (),
}
impl GetDiscoveredSchema {
    /// Creates a new builder-style object to manufacture [`GetDiscoveredSchemaInput`](crate::input::GetDiscoveredSchemaInput)
    pub fn builder() -> crate::input::get_discovered_schema_input::Builder {
        crate::input::get_discovered_schema_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for GetDiscoveredSchema {
    type Output = std::result::Result<
        crate::output::GetDiscoveredSchemaOutput,
        crate::error::GetDiscoveredSchemaError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_discovered_schema_error(response)
        } else {
            crate::operation_deser::parse_get_discovered_schema_response(response)
        }
    }
}

/// <p>Retrieves the resource-based policy attached to a given registry.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetResourcePolicy {
    _private: (),
}
impl GetResourcePolicy {
    /// Creates a new builder-style object to manufacture [`GetResourcePolicyInput`](crate::input::GetResourcePolicyInput)
    pub fn builder() -> crate::input::get_resource_policy_input::Builder {
        crate::input::get_resource_policy_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for GetResourcePolicy {
    type Output = std::result::Result<
        crate::output::GetResourcePolicyOutput,
        crate::error::GetResourcePolicyError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_resource_policy_error(response)
        } else {
            crate::operation_deser::parse_get_resource_policy_response(response)
        }
    }
}

/// <p>List the discoverers.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListDiscoverers {
    _private: (),
}
impl ListDiscoverers {
    /// Creates a new builder-style object to manufacture [`ListDiscoverersInput`](crate::input::ListDiscoverersInput)
    pub fn builder() -> crate::input::list_discoverers_input::Builder {
        crate::input::list_discoverers_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListDiscoverers {
    type Output = std::result::Result<
        crate::output::ListDiscoverersOutput,
        crate::error::ListDiscoverersError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_discoverers_error(response)
        } else {
            crate::operation_deser::parse_list_discoverers_response(response)
        }
    }
}

/// <p>List the registries.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListRegistries {
    _private: (),
}
impl ListRegistries {
    /// Creates a new builder-style object to manufacture [`ListRegistriesInput`](crate::input::ListRegistriesInput)
    pub fn builder() -> crate::input::list_registries_input::Builder {
        crate::input::list_registries_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListRegistries {
    type Output =
        std::result::Result<crate::output::ListRegistriesOutput, crate::error::ListRegistriesError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_registries_error(response)
        } else {
            crate::operation_deser::parse_list_registries_response(response)
        }
    }
}

/// <p>List the schemas.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListSchemas {
    _private: (),
}
impl ListSchemas {
    /// Creates a new builder-style object to manufacture [`ListSchemasInput`](crate::input::ListSchemasInput)
    pub fn builder() -> crate::input::list_schemas_input::Builder {
        crate::input::list_schemas_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListSchemas {
    type Output =
        std::result::Result<crate::output::ListSchemasOutput, crate::error::ListSchemasError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_schemas_error(response)
        } else {
            crate::operation_deser::parse_list_schemas_response(response)
        }
    }
}

/// <p>Provides a list of the schema versions and related information.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListSchemaVersions {
    _private: (),
}
impl ListSchemaVersions {
    /// Creates a new builder-style object to manufacture [`ListSchemaVersionsInput`](crate::input::ListSchemaVersionsInput)
    pub fn builder() -> crate::input::list_schema_versions_input::Builder {
        crate::input::list_schema_versions_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListSchemaVersions {
    type Output = std::result::Result<
        crate::output::ListSchemaVersionsOutput,
        crate::error::ListSchemaVersionsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_schema_versions_error(response)
        } else {
            crate::operation_deser::parse_list_schema_versions_response(response)
        }
    }
}

/// <p>Get tags for resource.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListTagsForResource {
    _private: (),
}
impl ListTagsForResource {
    /// Creates a new builder-style object to manufacture [`ListTagsForResourceInput`](crate::input::ListTagsForResourceInput)
    pub fn builder() -> crate::input::list_tags_for_resource_input::Builder {
        crate::input::list_tags_for_resource_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListTagsForResource {
    type Output = std::result::Result<
        crate::output::ListTagsForResourceOutput,
        crate::error::ListTagsForResourceError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_tags_for_resource_error(response)
        } else {
            crate::operation_deser::parse_list_tags_for_resource_response(response)
        }
    }
}

/// <p>Put code binding URI</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct PutCodeBinding {
    _private: (),
}
impl PutCodeBinding {
    /// Creates a new builder-style object to manufacture [`PutCodeBindingInput`](crate::input::PutCodeBindingInput)
    pub fn builder() -> crate::input::put_code_binding_input::Builder {
        crate::input::put_code_binding_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for PutCodeBinding {
    type Output =
        std::result::Result<crate::output::PutCodeBindingOutput, crate::error::PutCodeBindingError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 202 {
            crate::operation_deser::parse_put_code_binding_error(response)
        } else {
            crate::operation_deser::parse_put_code_binding_response(response)
        }
    }
}

/// <p>The name of the policy.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct PutResourcePolicy {
    _private: (),
}
impl PutResourcePolicy {
    /// Creates a new builder-style object to manufacture [`PutResourcePolicyInput`](crate::input::PutResourcePolicyInput)
    pub fn builder() -> crate::input::put_resource_policy_input::Builder {
        crate::input::put_resource_policy_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for PutResourcePolicy {
    type Output = std::result::Result<
        crate::output::PutResourcePolicyOutput,
        crate::error::PutResourcePolicyError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_put_resource_policy_error(response)
        } else {
            crate::operation_deser::parse_put_resource_policy_response(response)
        }
    }
}

/// <p>Search the schemas</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct SearchSchemas {
    _private: (),
}
impl SearchSchemas {
    /// Creates a new builder-style object to manufacture [`SearchSchemasInput`](crate::input::SearchSchemasInput)
    pub fn builder() -> crate::input::search_schemas_input::Builder {
        crate::input::search_schemas_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for SearchSchemas {
    type Output =
        std::result::Result<crate::output::SearchSchemasOutput, crate::error::SearchSchemasError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_search_schemas_error(response)
        } else {
            crate::operation_deser::parse_search_schemas_response(response)
        }
    }
}

/// <p>Starts the discoverer</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct StartDiscoverer {
    _private: (),
}
impl StartDiscoverer {
    /// Creates a new builder-style object to manufacture [`StartDiscovererInput`](crate::input::StartDiscovererInput)
    pub fn builder() -> crate::input::start_discoverer_input::Builder {
        crate::input::start_discoverer_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for StartDiscoverer {
    type Output = std::result::Result<
        crate::output::StartDiscovererOutput,
        crate::error::StartDiscovererError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_start_discoverer_error(response)
        } else {
            crate::operation_deser::parse_start_discoverer_response(response)
        }
    }
}

/// <p>Stops the discoverer</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct StopDiscoverer {
    _private: (),
}
impl StopDiscoverer {
    /// Creates a new builder-style object to manufacture [`StopDiscovererInput`](crate::input::StopDiscovererInput)
    pub fn builder() -> crate::input::stop_discoverer_input::Builder {
        crate::input::stop_discoverer_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for StopDiscoverer {
    type Output =
        std::result::Result<crate::output::StopDiscovererOutput, crate::error::StopDiscovererError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_stop_discoverer_error(response)
        } else {
            crate::operation_deser::parse_stop_discoverer_response(response)
        }
    }
}

/// <p>Add tags to a resource.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct TagResource {
    _private: (),
}
impl TagResource {
    /// Creates a new builder-style object to manufacture [`TagResourceInput`](crate::input::TagResourceInput)
    pub fn builder() -> crate::input::tag_resource_input::Builder {
        crate::input::tag_resource_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for TagResource {
    type Output =
        std::result::Result<crate::output::TagResourceOutput, crate::error::TagResourceError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 204 {
            crate::operation_deser::parse_tag_resource_error(response)
        } else {
            crate::operation_deser::parse_tag_resource_response(response)
        }
    }
}

/// <p>Removes tags from a resource.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UntagResource {
    _private: (),
}
impl UntagResource {
    /// Creates a new builder-style object to manufacture [`UntagResourceInput`](crate::input::UntagResourceInput)
    pub fn builder() -> crate::input::untag_resource_input::Builder {
        crate::input::untag_resource_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for UntagResource {
    type Output =
        std::result::Result<crate::output::UntagResourceOutput, crate::error::UntagResourceError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 204 {
            crate::operation_deser::parse_untag_resource_error(response)
        } else {
            crate::operation_deser::parse_untag_resource_response(response)
        }
    }
}

/// <p>Updates the discoverer</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateDiscoverer {
    _private: (),
}
impl UpdateDiscoverer {
    /// Creates a new builder-style object to manufacture [`UpdateDiscovererInput`](crate::input::UpdateDiscovererInput)
    pub fn builder() -> crate::input::update_discoverer_input::Builder {
        crate::input::update_discoverer_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for UpdateDiscoverer {
    type Output = std::result::Result<
        crate::output::UpdateDiscovererOutput,
        crate::error::UpdateDiscovererError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_discoverer_error(response)
        } else {
            crate::operation_deser::parse_update_discoverer_response(response)
        }
    }
}

/// <p>Updates a registry.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateRegistry {
    _private: (),
}
impl UpdateRegistry {
    /// Creates a new builder-style object to manufacture [`UpdateRegistryInput`](crate::input::UpdateRegistryInput)
    pub fn builder() -> crate::input::update_registry_input::Builder {
        crate::input::update_registry_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for UpdateRegistry {
    type Output =
        std::result::Result<crate::output::UpdateRegistryOutput, crate::error::UpdateRegistryError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_registry_error(response)
        } else {
            crate::operation_deser::parse_update_registry_response(response)
        }
    }
}

/// <p>Updates the schema definition</p> <note><p>Inactive schemas will be deleted after two years.</p></note>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateSchema {
    _private: (),
}
impl UpdateSchema {
    /// Creates a new builder-style object to manufacture [`UpdateSchemaInput`](crate::input::UpdateSchemaInput)
    pub fn builder() -> crate::input::update_schema_input::Builder {
        crate::input::update_schema_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for UpdateSchema {
    type Output =
        std::result::Result<crate::output::UpdateSchemaOutput, crate::error::UpdateSchemaError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_schema_error(response)
        } else {
            crate::operation_deser::parse_update_schema_response(response)
        }
    }
}
