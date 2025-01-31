// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// <p>Creates a Data Store that can ingest and export FHIR formatted data.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateFHIRDatastore {
    _private: (),
}
impl CreateFHIRDatastore {
    /// Creates a new builder-style object to manufacture [`CreateFhirDatastoreInput`](crate::input::CreateFhirDatastoreInput)
    pub fn builder() -> crate::input::create_fhir_datastore_input::Builder {
        crate::input::create_fhir_datastore_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for CreateFHIRDatastore {
    type Output = std::result::Result<
        crate::output::CreateFhirDatastoreOutput,
        crate::error::CreateFHIRDatastoreError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_fhir_datastore_error(response)
        } else {
            crate::operation_deser::parse_create_fhir_datastore_response(response)
        }
    }
}

/// <p>Deletes a Data Store. </p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteFHIRDatastore {
    _private: (),
}
impl DeleteFHIRDatastore {
    /// Creates a new builder-style object to manufacture [`DeleteFhirDatastoreInput`](crate::input::DeleteFhirDatastoreInput)
    pub fn builder() -> crate::input::delete_fhir_datastore_input::Builder {
        crate::input::delete_fhir_datastore_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DeleteFHIRDatastore {
    type Output = std::result::Result<
        crate::output::DeleteFhirDatastoreOutput,
        crate::error::DeleteFHIRDatastoreError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_fhir_datastore_error(response)
        } else {
            crate::operation_deser::parse_delete_fhir_datastore_response(response)
        }
    }
}

/// <p>Gets the properties associated with the FHIR Data Store, including the Data Store ID,
/// Data Store ARN, Data Store name, Data Store status, created at, Data Store type version, and
/// Data Store endpoint.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeFHIRDatastore {
    _private: (),
}
impl DescribeFHIRDatastore {
    /// Creates a new builder-style object to manufacture [`DescribeFhirDatastoreInput`](crate::input::DescribeFhirDatastoreInput)
    pub fn builder() -> crate::input::describe_fhir_datastore_input::Builder {
        crate::input::describe_fhir_datastore_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DescribeFHIRDatastore {
    type Output = std::result::Result<
        crate::output::DescribeFhirDatastoreOutput,
        crate::error::DescribeFHIRDatastoreError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_fhir_datastore_error(response)
        } else {
            crate::operation_deser::parse_describe_fhir_datastore_response(response)
        }
    }
}

/// <p>Displays the properties of a FHIR export job, including the ID, ARN, name, and the status of the job.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeFHIRExportJob {
    _private: (),
}
impl DescribeFHIRExportJob {
    /// Creates a new builder-style object to manufacture [`DescribeFhirExportJobInput`](crate::input::DescribeFhirExportJobInput)
    pub fn builder() -> crate::input::describe_fhir_export_job_input::Builder {
        crate::input::describe_fhir_export_job_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DescribeFHIRExportJob {
    type Output = std::result::Result<
        crate::output::DescribeFhirExportJobOutput,
        crate::error::DescribeFHIRExportJobError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_fhir_export_job_error(response)
        } else {
            crate::operation_deser::parse_describe_fhir_export_job_response(response)
        }
    }
}

/// <p>Displays the properties of a FHIR import job, including the ID, ARN, name, and the status of the job. </p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeFHIRImportJob {
    _private: (),
}
impl DescribeFHIRImportJob {
    /// Creates a new builder-style object to manufacture [`DescribeFhirImportJobInput`](crate::input::DescribeFhirImportJobInput)
    pub fn builder() -> crate::input::describe_fhir_import_job_input::Builder {
        crate::input::describe_fhir_import_job_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DescribeFHIRImportJob {
    type Output = std::result::Result<
        crate::output::DescribeFhirImportJobOutput,
        crate::error::DescribeFHIRImportJobError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_fhir_import_job_error(response)
        } else {
            crate::operation_deser::parse_describe_fhir_import_job_response(response)
        }
    }
}

/// <p>Lists all FHIR Data Stores that are in the user’s account, regardless of Data Store
/// status.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListFHIRDatastores {
    _private: (),
}
impl ListFHIRDatastores {
    /// Creates a new builder-style object to manufacture [`ListFhirDatastoresInput`](crate::input::ListFhirDatastoresInput)
    pub fn builder() -> crate::input::list_fhir_datastores_input::Builder {
        crate::input::list_fhir_datastores_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListFHIRDatastores {
    type Output = std::result::Result<
        crate::output::ListFhirDatastoresOutput,
        crate::error::ListFHIRDatastoresError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_fhir_datastores_error(response)
        } else {
            crate::operation_deser::parse_list_fhir_datastores_response(response)
        }
    }
}

/// <p>
/// Lists all FHIR export jobs associated with an account and their statuses.
/// </p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListFHIRExportJobs {
    _private: (),
}
impl ListFHIRExportJobs {
    /// Creates a new builder-style object to manufacture [`ListFhirExportJobsInput`](crate::input::ListFhirExportJobsInput)
    pub fn builder() -> crate::input::list_fhir_export_jobs_input::Builder {
        crate::input::list_fhir_export_jobs_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListFHIRExportJobs {
    type Output = std::result::Result<
        crate::output::ListFhirExportJobsOutput,
        crate::error::ListFHIRExportJobsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_fhir_export_jobs_error(response)
        } else {
            crate::operation_deser::parse_list_fhir_export_jobs_response(response)
        }
    }
}

/// <p>
/// Lists all FHIR import jobs associated with an account and their statuses.
/// </p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListFHIRImportJobs {
    _private: (),
}
impl ListFHIRImportJobs {
    /// Creates a new builder-style object to manufacture [`ListFhirImportJobsInput`](crate::input::ListFhirImportJobsInput)
    pub fn builder() -> crate::input::list_fhir_import_jobs_input::Builder {
        crate::input::list_fhir_import_jobs_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListFHIRImportJobs {
    type Output = std::result::Result<
        crate::output::ListFhirImportJobsOutput,
        crate::error::ListFHIRImportJobsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_fhir_import_jobs_error(response)
        } else {
            crate::operation_deser::parse_list_fhir_import_jobs_response(response)
        }
    }
}

/// <p>
/// Returns a list of all existing tags associated with a Data Store.
/// </p>
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

/// <p>Begins a FHIR export job.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct StartFHIRExportJob {
    _private: (),
}
impl StartFHIRExportJob {
    /// Creates a new builder-style object to manufacture [`StartFhirExportJobInput`](crate::input::StartFhirExportJobInput)
    pub fn builder() -> crate::input::start_fhir_export_job_input::Builder {
        crate::input::start_fhir_export_job_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for StartFHIRExportJob {
    type Output = std::result::Result<
        crate::output::StartFhirExportJobOutput,
        crate::error::StartFHIRExportJobError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_start_fhir_export_job_error(response)
        } else {
            crate::operation_deser::parse_start_fhir_export_job_response(response)
        }
    }
}

/// <p>Begins a FHIR Import job.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct StartFHIRImportJob {
    _private: (),
}
impl StartFHIRImportJob {
    /// Creates a new builder-style object to manufacture [`StartFhirImportJobInput`](crate::input::StartFhirImportJobInput)
    pub fn builder() -> crate::input::start_fhir_import_job_input::Builder {
        crate::input::start_fhir_import_job_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for StartFHIRImportJob {
    type Output = std::result::Result<
        crate::output::StartFhirImportJobOutput,
        crate::error::StartFHIRImportJobError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_start_fhir_import_job_error(response)
        } else {
            crate::operation_deser::parse_start_fhir_import_job_response(response)
        }
    }
}

/// <p>
/// Adds a user specifed key and value tag to a Data Store.
/// </p>
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
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_tag_resource_error(response)
        } else {
            crate::operation_deser::parse_tag_resource_response(response)
        }
    }
}

/// <p>
/// Removes tags from a Data Store.
/// </p>
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
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_untag_resource_error(response)
        } else {
            crate::operation_deser::parse_untag_resource_response(response)
        }
    }
}
