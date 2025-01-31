// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn endpoint_resolver() -> impl aws_endpoint::ResolveAwsEndpoint {
    aws_endpoint::PartitionResolver::new(
        aws_endpoint::Partition::builder()
            .id("aws")
            .region_regex(r#"^(us|eu|ap|sa|ca|me|af)\-\w+\-\d+$"#)
            .default_endpoint(aws_endpoint::partition::endpoint::Metadata {
                uri_template: "iam.{region}.amazonaws.com",
                protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                credential_scope: aws_endpoint::CredentialScope::builder().build(),
            })
            .partition_endpoint("aws-global")
            .regionalized(aws_endpoint::partition::Regionalized::NotRegionalized)
            .endpoint(
                "aws-global",
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "iam.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder()
                        .region("us-east-1")
                        .build(),
                },
            )
            .endpoint(
                "iam-fips",
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "iam-fips.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder()
                        .region("us-east-1")
                        .build(),
                },
            )
            .build()
            .expect("invalid partition"),
        vec![
            aws_endpoint::Partition::builder()
                .id("aws-cn")
                .region_regex(r#"^cn\-\w+\-\d+$"#)
                .default_endpoint(aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "iam.{region}.amazonaws.com.cn",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder().build(),
                })
                .partition_endpoint("aws-cn-global")
                .regionalized(aws_endpoint::partition::Regionalized::NotRegionalized)
                .endpoint(
                    "aws-cn-global",
                    aws_endpoint::partition::endpoint::Metadata {
                        uri_template: "iam.cn-north-1.amazonaws.com.cn",
                        protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                        signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                        credential_scope: aws_endpoint::CredentialScope::builder()
                            .region("cn-north-1")
                            .build(),
                    },
                )
                .build()
                .expect("invalid partition"),
            aws_endpoint::Partition::builder()
                .id("aws-iso")
                .region_regex(r#"^us\-iso\-\w+\-\d+$"#)
                .default_endpoint(aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "iam.{region}.c2s.ic.gov",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder().build(),
                })
                .partition_endpoint("aws-iso-global")
                .regionalized(aws_endpoint::partition::Regionalized::NotRegionalized)
                .endpoint(
                    "aws-iso-global",
                    aws_endpoint::partition::endpoint::Metadata {
                        uri_template: "iam.us-iso-east-1.c2s.ic.gov",
                        protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                        signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                        credential_scope: aws_endpoint::CredentialScope::builder()
                            .region("us-iso-east-1")
                            .build(),
                    },
                )
                .build()
                .expect("invalid partition"),
            aws_endpoint::Partition::builder()
                .id("aws-iso-b")
                .region_regex(r#"^us\-isob\-\w+\-\d+$"#)
                .default_endpoint(aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "iam.{region}.sc2s.sgov.gov",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder().build(),
                })
                .partition_endpoint("aws-iso-b-global")
                .regionalized(aws_endpoint::partition::Regionalized::NotRegionalized)
                .endpoint(
                    "aws-iso-b-global",
                    aws_endpoint::partition::endpoint::Metadata {
                        uri_template: "iam.us-isob-east-1.sc2s.sgov.gov",
                        protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                        signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                        credential_scope: aws_endpoint::CredentialScope::builder()
                            .region("us-isob-east-1")
                            .build(),
                    },
                )
                .build()
                .expect("invalid partition"),
            aws_endpoint::Partition::builder()
                .id("aws-us-gov")
                .region_regex(r#"^us\-gov\-\w+\-\d+$"#)
                .default_endpoint(aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "iam.{region}.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder().build(),
                })
                .partition_endpoint("aws-us-gov-global")
                .regionalized(aws_endpoint::partition::Regionalized::NotRegionalized)
                .endpoint(
                    "aws-us-gov-global",
                    aws_endpoint::partition::endpoint::Metadata {
                        uri_template: "iam.us-gov.amazonaws.com",
                        protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                        signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                        credential_scope: aws_endpoint::CredentialScope::builder()
                            .region("us-gov-west-1")
                            .build(),
                    },
                )
                .endpoint(
                    "iam-govcloud-fips",
                    aws_endpoint::partition::endpoint::Metadata {
                        uri_template: "iam.us-gov.amazonaws.com",
                        protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                        signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                        credential_scope: aws_endpoint::CredentialScope::builder()
                            .region("us-gov-west-1")
                            .build(),
                    },
                )
                .build()
                .expect("invalid partition"),
        ],
    )
}
