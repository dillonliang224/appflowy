///
//  Generated code. Do not modify.
//  source: trash_create.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,unnecessary_const,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type,unnecessary_this,prefer_final_fields,deprecated_member_use_from_same_package

import 'dart:core' as $core;
import 'dart:convert' as $convert;
import 'dart:typed_data' as $typed_data;
@$core.Deprecated('Use createTrashParamsDescriptor instead')
const CreateTrashParams$json = const {
  '1': 'CreateTrashParams',
  '2': const [
    const {'1': 'id', '3': 1, '4': 1, '5': 9, '10': 'id'},
    const {'1': 'name', '3': 2, '4': 1, '5': 9, '10': 'name'},
    const {'1': 'modified_time', '3': 3, '4': 1, '5': 3, '10': 'modifiedTime'},
    const {'1': 'create_time', '3': 4, '4': 1, '5': 3, '10': 'createTime'},
  ],
};

/// Descriptor for `CreateTrashParams`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List createTrashParamsDescriptor = $convert.base64Decode('ChFDcmVhdGVUcmFzaFBhcmFtcxIOCgJpZBgBIAEoCVICaWQSEgoEbmFtZRgCIAEoCVIEbmFtZRIjCg1tb2RpZmllZF90aW1lGAMgASgDUgxtb2RpZmllZFRpbWUSHwoLY3JlYXRlX3RpbWUYBCABKANSCmNyZWF0ZVRpbWU=');
@$core.Deprecated('Use trashDescriptor instead')
const Trash$json = const {
  '1': 'Trash',
  '2': const [
    const {'1': 'id', '3': 1, '4': 1, '5': 9, '10': 'id'},
    const {'1': 'name', '3': 2, '4': 1, '5': 9, '10': 'name'},
    const {'1': 'modified_time', '3': 3, '4': 1, '5': 3, '10': 'modifiedTime'},
    const {'1': 'create_time', '3': 4, '4': 1, '5': 3, '10': 'createTime'},
  ],
};

/// Descriptor for `Trash`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List trashDescriptor = $convert.base64Decode('CgVUcmFzaBIOCgJpZBgBIAEoCVICaWQSEgoEbmFtZRgCIAEoCVIEbmFtZRIjCg1tb2RpZmllZF90aW1lGAMgASgDUgxtb2RpZmllZFRpbWUSHwoLY3JlYXRlX3RpbWUYBCABKANSCmNyZWF0ZVRpbWU=');
@$core.Deprecated('Use repeatedTrashDescriptor instead')
const RepeatedTrash$json = const {
  '1': 'RepeatedTrash',
  '2': const [
    const {'1': 'items', '3': 1, '4': 3, '5': 11, '6': '.Trash', '10': 'items'},
  ],
};

/// Descriptor for `RepeatedTrash`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List repeatedTrashDescriptor = $convert.base64Decode('Cg1SZXBlYXRlZFRyYXNoEhwKBWl0ZW1zGAEgAygLMgYuVHJhc2hSBWl0ZW1z');