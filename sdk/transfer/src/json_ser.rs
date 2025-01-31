// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_create_access_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateAccessInput,
) {
    if let Some(var_1) = &input.home_directory {
        object.key("HomeDirectory").string(var_1);
    }
    if let Some(var_2) = &input.home_directory_type {
        object.key("HomeDirectoryType").string(var_2.as_str());
    }
    if let Some(var_3) = &input.home_directory_mappings {
        let mut array_4 = object.key("HomeDirectoryMappings").start_array();
        for item_5 in var_3 {
            {
                let mut object_6 = array_4.value().start_object();
                crate::json_ser::serialize_structure_home_directory_map_entry(
                    &mut object_6,
                    item_5,
                );
                object_6.finish();
            }
        }
        array_4.finish();
    }
    if let Some(var_7) = &input.policy {
        object.key("Policy").string(var_7);
    }
    if let Some(var_8) = &input.posix_profile {
        let mut object_9 = object.key("PosixProfile").start_object();
        crate::json_ser::serialize_structure_posix_profile(&mut object_9, var_8);
        object_9.finish();
    }
    if let Some(var_10) = &input.role {
        object.key("Role").string(var_10);
    }
    if let Some(var_11) = &input.server_id {
        object.key("ServerId").string(var_11);
    }
    if let Some(var_12) = &input.external_id {
        object.key("ExternalId").string(var_12);
    }
}

pub fn serialize_structure_create_server_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateServerInput,
) {
    if let Some(var_13) = &input.certificate {
        object.key("Certificate").string(var_13);
    }
    if let Some(var_14) = &input.domain {
        object.key("Domain").string(var_14.as_str());
    }
    if let Some(var_15) = &input.endpoint_details {
        let mut object_16 = object.key("EndpointDetails").start_object();
        crate::json_ser::serialize_structure_endpoint_details(&mut object_16, var_15);
        object_16.finish();
    }
    if let Some(var_17) = &input.endpoint_type {
        object.key("EndpointType").string(var_17.as_str());
    }
    if let Some(var_18) = &input.host_key {
        object.key("HostKey").string(var_18);
    }
    if let Some(var_19) = &input.identity_provider_details {
        let mut object_20 = object.key("IdentityProviderDetails").start_object();
        crate::json_ser::serialize_structure_identity_provider_details(&mut object_20, var_19);
        object_20.finish();
    }
    if let Some(var_21) = &input.identity_provider_type {
        object.key("IdentityProviderType").string(var_21.as_str());
    }
    if let Some(var_22) = &input.logging_role {
        object.key("LoggingRole").string(var_22);
    }
    if let Some(var_23) = &input.protocols {
        let mut array_24 = object.key("Protocols").start_array();
        for item_25 in var_23 {
            {
                array_24.value().string(item_25.as_str());
            }
        }
        array_24.finish();
    }
    if let Some(var_26) = &input.security_policy_name {
        object.key("SecurityPolicyName").string(var_26);
    }
    if let Some(var_27) = &input.tags {
        let mut array_28 = object.key("Tags").start_array();
        for item_29 in var_27 {
            {
                let mut object_30 = array_28.value().start_object();
                crate::json_ser::serialize_structure_tag(&mut object_30, item_29);
                object_30.finish();
            }
        }
        array_28.finish();
    }
}

pub fn serialize_structure_create_user_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateUserInput,
) {
    if let Some(var_31) = &input.home_directory {
        object.key("HomeDirectory").string(var_31);
    }
    if let Some(var_32) = &input.home_directory_type {
        object.key("HomeDirectoryType").string(var_32.as_str());
    }
    if let Some(var_33) = &input.home_directory_mappings {
        let mut array_34 = object.key("HomeDirectoryMappings").start_array();
        for item_35 in var_33 {
            {
                let mut object_36 = array_34.value().start_object();
                crate::json_ser::serialize_structure_home_directory_map_entry(
                    &mut object_36,
                    item_35,
                );
                object_36.finish();
            }
        }
        array_34.finish();
    }
    if let Some(var_37) = &input.policy {
        object.key("Policy").string(var_37);
    }
    if let Some(var_38) = &input.posix_profile {
        let mut object_39 = object.key("PosixProfile").start_object();
        crate::json_ser::serialize_structure_posix_profile(&mut object_39, var_38);
        object_39.finish();
    }
    if let Some(var_40) = &input.role {
        object.key("Role").string(var_40);
    }
    if let Some(var_41) = &input.server_id {
        object.key("ServerId").string(var_41);
    }
    if let Some(var_42) = &input.ssh_public_key_body {
        object.key("SshPublicKeyBody").string(var_42);
    }
    if let Some(var_43) = &input.tags {
        let mut array_44 = object.key("Tags").start_array();
        for item_45 in var_43 {
            {
                let mut object_46 = array_44.value().start_object();
                crate::json_ser::serialize_structure_tag(&mut object_46, item_45);
                object_46.finish();
            }
        }
        array_44.finish();
    }
    if let Some(var_47) = &input.user_name {
        object.key("UserName").string(var_47);
    }
}

pub fn serialize_structure_delete_access_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteAccessInput,
) {
    if let Some(var_48) = &input.server_id {
        object.key("ServerId").string(var_48);
    }
    if let Some(var_49) = &input.external_id {
        object.key("ExternalId").string(var_49);
    }
}

pub fn serialize_structure_delete_server_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteServerInput,
) {
    if let Some(var_50) = &input.server_id {
        object.key("ServerId").string(var_50);
    }
}

pub fn serialize_structure_delete_ssh_public_key_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteSshPublicKeyInput,
) {
    if let Some(var_51) = &input.server_id {
        object.key("ServerId").string(var_51);
    }
    if let Some(var_52) = &input.ssh_public_key_id {
        object.key("SshPublicKeyId").string(var_52);
    }
    if let Some(var_53) = &input.user_name {
        object.key("UserName").string(var_53);
    }
}

pub fn serialize_structure_delete_user_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteUserInput,
) {
    if let Some(var_54) = &input.server_id {
        object.key("ServerId").string(var_54);
    }
    if let Some(var_55) = &input.user_name {
        object.key("UserName").string(var_55);
    }
}

pub fn serialize_structure_describe_access_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeAccessInput,
) {
    if let Some(var_56) = &input.server_id {
        object.key("ServerId").string(var_56);
    }
    if let Some(var_57) = &input.external_id {
        object.key("ExternalId").string(var_57);
    }
}

pub fn serialize_structure_describe_security_policy_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeSecurityPolicyInput,
) {
    if let Some(var_58) = &input.security_policy_name {
        object.key("SecurityPolicyName").string(var_58);
    }
}

pub fn serialize_structure_describe_server_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeServerInput,
) {
    if let Some(var_59) = &input.server_id {
        object.key("ServerId").string(var_59);
    }
}

pub fn serialize_structure_describe_user_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeUserInput,
) {
    if let Some(var_60) = &input.server_id {
        object.key("ServerId").string(var_60);
    }
    if let Some(var_61) = &input.user_name {
        object.key("UserName").string(var_61);
    }
}

pub fn serialize_structure_import_ssh_public_key_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ImportSshPublicKeyInput,
) {
    if let Some(var_62) = &input.server_id {
        object.key("ServerId").string(var_62);
    }
    if let Some(var_63) = &input.ssh_public_key_body {
        object.key("SshPublicKeyBody").string(var_63);
    }
    if let Some(var_64) = &input.user_name {
        object.key("UserName").string(var_64);
    }
}

pub fn serialize_structure_list_accesses_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListAccessesInput,
) {
    if let Some(var_65) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_65).into()),
        );
    }
    if let Some(var_66) = &input.next_token {
        object.key("NextToken").string(var_66);
    }
    if let Some(var_67) = &input.server_id {
        object.key("ServerId").string(var_67);
    }
}

pub fn serialize_structure_list_security_policies_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListSecurityPoliciesInput,
) {
    if let Some(var_68) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_68).into()),
        );
    }
    if let Some(var_69) = &input.next_token {
        object.key("NextToken").string(var_69);
    }
}

pub fn serialize_structure_list_servers_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListServersInput,
) {
    if let Some(var_70) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_70).into()),
        );
    }
    if let Some(var_71) = &input.next_token {
        object.key("NextToken").string(var_71);
    }
}

pub fn serialize_structure_list_tags_for_resource_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListTagsForResourceInput,
) {
    if let Some(var_72) = &input.arn {
        object.key("Arn").string(var_72);
    }
    if let Some(var_73) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_73).into()),
        );
    }
    if let Some(var_74) = &input.next_token {
        object.key("NextToken").string(var_74);
    }
}

pub fn serialize_structure_list_users_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListUsersInput,
) {
    if let Some(var_75) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_75).into()),
        );
    }
    if let Some(var_76) = &input.next_token {
        object.key("NextToken").string(var_76);
    }
    if let Some(var_77) = &input.server_id {
        object.key("ServerId").string(var_77);
    }
}

pub fn serialize_structure_start_server_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StartServerInput,
) {
    if let Some(var_78) = &input.server_id {
        object.key("ServerId").string(var_78);
    }
}

pub fn serialize_structure_stop_server_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StopServerInput,
) {
    if let Some(var_79) = &input.server_id {
        object.key("ServerId").string(var_79);
    }
}

pub fn serialize_structure_tag_resource_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) {
    if let Some(var_80) = &input.arn {
        object.key("Arn").string(var_80);
    }
    if let Some(var_81) = &input.tags {
        let mut array_82 = object.key("Tags").start_array();
        for item_83 in var_81 {
            {
                let mut object_84 = array_82.value().start_object();
                crate::json_ser::serialize_structure_tag(&mut object_84, item_83);
                object_84.finish();
            }
        }
        array_82.finish();
    }
}

pub fn serialize_structure_test_identity_provider_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TestIdentityProviderInput,
) {
    if let Some(var_85) = &input.server_id {
        object.key("ServerId").string(var_85);
    }
    if let Some(var_86) = &input.server_protocol {
        object.key("ServerProtocol").string(var_86.as_str());
    }
    if let Some(var_87) = &input.source_ip {
        object.key("SourceIp").string(var_87);
    }
    if let Some(var_88) = &input.user_name {
        object.key("UserName").string(var_88);
    }
    if let Some(var_89) = &input.user_password {
        object.key("UserPassword").string(var_89);
    }
}

pub fn serialize_structure_untag_resource_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UntagResourceInput,
) {
    if let Some(var_90) = &input.arn {
        object.key("Arn").string(var_90);
    }
    if let Some(var_91) = &input.tag_keys {
        let mut array_92 = object.key("TagKeys").start_array();
        for item_93 in var_91 {
            {
                array_92.value().string(item_93);
            }
        }
        array_92.finish();
    }
}

pub fn serialize_structure_update_access_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateAccessInput,
) {
    if let Some(var_94) = &input.home_directory {
        object.key("HomeDirectory").string(var_94);
    }
    if let Some(var_95) = &input.home_directory_type {
        object.key("HomeDirectoryType").string(var_95.as_str());
    }
    if let Some(var_96) = &input.home_directory_mappings {
        let mut array_97 = object.key("HomeDirectoryMappings").start_array();
        for item_98 in var_96 {
            {
                let mut object_99 = array_97.value().start_object();
                crate::json_ser::serialize_structure_home_directory_map_entry(
                    &mut object_99,
                    item_98,
                );
                object_99.finish();
            }
        }
        array_97.finish();
    }
    if let Some(var_100) = &input.policy {
        object.key("Policy").string(var_100);
    }
    if let Some(var_101) = &input.posix_profile {
        let mut object_102 = object.key("PosixProfile").start_object();
        crate::json_ser::serialize_structure_posix_profile(&mut object_102, var_101);
        object_102.finish();
    }
    if let Some(var_103) = &input.role {
        object.key("Role").string(var_103);
    }
    if let Some(var_104) = &input.server_id {
        object.key("ServerId").string(var_104);
    }
    if let Some(var_105) = &input.external_id {
        object.key("ExternalId").string(var_105);
    }
}

pub fn serialize_structure_update_server_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateServerInput,
) {
    if let Some(var_106) = &input.certificate {
        object.key("Certificate").string(var_106);
    }
    if let Some(var_107) = &input.protocol_details {
        let mut object_108 = object.key("ProtocolDetails").start_object();
        crate::json_ser::serialize_structure_protocol_details(&mut object_108, var_107);
        object_108.finish();
    }
    if let Some(var_109) = &input.endpoint_details {
        let mut object_110 = object.key("EndpointDetails").start_object();
        crate::json_ser::serialize_structure_endpoint_details(&mut object_110, var_109);
        object_110.finish();
    }
    if let Some(var_111) = &input.endpoint_type {
        object.key("EndpointType").string(var_111.as_str());
    }
    if let Some(var_112) = &input.host_key {
        object.key("HostKey").string(var_112);
    }
    if let Some(var_113) = &input.identity_provider_details {
        let mut object_114 = object.key("IdentityProviderDetails").start_object();
        crate::json_ser::serialize_structure_identity_provider_details(&mut object_114, var_113);
        object_114.finish();
    }
    if let Some(var_115) = &input.logging_role {
        object.key("LoggingRole").string(var_115);
    }
    if let Some(var_116) = &input.protocols {
        let mut array_117 = object.key("Protocols").start_array();
        for item_118 in var_116 {
            {
                array_117.value().string(item_118.as_str());
            }
        }
        array_117.finish();
    }
    if let Some(var_119) = &input.security_policy_name {
        object.key("SecurityPolicyName").string(var_119);
    }
    if let Some(var_120) = &input.server_id {
        object.key("ServerId").string(var_120);
    }
}

pub fn serialize_structure_update_user_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateUserInput,
) {
    if let Some(var_121) = &input.home_directory {
        object.key("HomeDirectory").string(var_121);
    }
    if let Some(var_122) = &input.home_directory_type {
        object.key("HomeDirectoryType").string(var_122.as_str());
    }
    if let Some(var_123) = &input.home_directory_mappings {
        let mut array_124 = object.key("HomeDirectoryMappings").start_array();
        for item_125 in var_123 {
            {
                let mut object_126 = array_124.value().start_object();
                crate::json_ser::serialize_structure_home_directory_map_entry(
                    &mut object_126,
                    item_125,
                );
                object_126.finish();
            }
        }
        array_124.finish();
    }
    if let Some(var_127) = &input.policy {
        object.key("Policy").string(var_127);
    }
    if let Some(var_128) = &input.posix_profile {
        let mut object_129 = object.key("PosixProfile").start_object();
        crate::json_ser::serialize_structure_posix_profile(&mut object_129, var_128);
        object_129.finish();
    }
    if let Some(var_130) = &input.role {
        object.key("Role").string(var_130);
    }
    if let Some(var_131) = &input.server_id {
        object.key("ServerId").string(var_131);
    }
    if let Some(var_132) = &input.user_name {
        object.key("UserName").string(var_132);
    }
}

pub fn serialize_structure_home_directory_map_entry(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::HomeDirectoryMapEntry,
) {
    if let Some(var_133) = &input.entry {
        object.key("Entry").string(var_133);
    }
    if let Some(var_134) = &input.target {
        object.key("Target").string(var_134);
    }
}

pub fn serialize_structure_posix_profile(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::PosixProfile,
) {
    if let Some(var_135) = &input.uid {
        object.key("Uid").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_135).into()),
        );
    }
    if let Some(var_136) = &input.gid {
        object.key("Gid").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_136).into()),
        );
    }
    if let Some(var_137) = &input.secondary_gids {
        let mut array_138 = object.key("SecondaryGids").start_array();
        for item_139 in var_137 {
            {
                array_138.value().number(
                    #[allow(clippy::useless_conversion)]
                    smithy_types::Number::NegInt((*item_139).into()),
                );
            }
        }
        array_138.finish();
    }
}

pub fn serialize_structure_endpoint_details(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::EndpointDetails,
) {
    if let Some(var_140) = &input.address_allocation_ids {
        let mut array_141 = object.key("AddressAllocationIds").start_array();
        for item_142 in var_140 {
            {
                array_141.value().string(item_142);
            }
        }
        array_141.finish();
    }
    if let Some(var_143) = &input.subnet_ids {
        let mut array_144 = object.key("SubnetIds").start_array();
        for item_145 in var_143 {
            {
                array_144.value().string(item_145);
            }
        }
        array_144.finish();
    }
    if let Some(var_146) = &input.vpc_endpoint_id {
        object.key("VpcEndpointId").string(var_146);
    }
    if let Some(var_147) = &input.vpc_id {
        object.key("VpcId").string(var_147);
    }
    if let Some(var_148) = &input.security_group_ids {
        let mut array_149 = object.key("SecurityGroupIds").start_array();
        for item_150 in var_148 {
            {
                array_149.value().string(item_150);
            }
        }
        array_149.finish();
    }
}

pub fn serialize_structure_identity_provider_details(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::IdentityProviderDetails,
) {
    if let Some(var_151) = &input.url {
        object.key("Url").string(var_151);
    }
    if let Some(var_152) = &input.invocation_role {
        object.key("InvocationRole").string(var_152);
    }
    if let Some(var_153) = &input.directory_id {
        object.key("DirectoryId").string(var_153);
    }
}

pub fn serialize_structure_tag(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Tag,
) {
    if let Some(var_154) = &input.key {
        object.key("Key").string(var_154);
    }
    if let Some(var_155) = &input.value {
        object.key("Value").string(var_155);
    }
}

pub fn serialize_structure_protocol_details(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ProtocolDetails,
) {
    if let Some(var_156) = &input.passive_ip {
        object.key("PassiveIp").string(var_156);
    }
}
