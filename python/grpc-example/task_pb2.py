# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# NO CHECKED-IN PROTOBUF GENCODE
# source: task.proto
# Protobuf Python Version: 5.27.2
"""Generated protocol buffer code."""
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import runtime_version as _runtime_version
from google.protobuf import symbol_database as _symbol_database
from google.protobuf.internal import builder as _builder
_runtime_version.ValidateProtobufRuntimeVersion(
    _runtime_version.Domain.PUBLIC,
    5,
    27,
    2,
    '',
    'task.proto'
)
# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()




DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\ntask.proto\x12\x04task\"F\n\x04Task\x12\n\n\x02id\x18\x01 \x01(\t\x12\r\n\x05title\x18\x02 \x01(\t\x12\x13\n\x0b\x64\x65scription\x18\x03 \x01(\t\x12\x0e\n\x06status\x18\x04 \x01(\t\"7\n\x11\x43reateTaskRequest\x12\r\n\x05title\x18\x01 \x01(\t\x12\x13\n\x0b\x64\x65scription\x18\x02 \x01(\t\"S\n\x11UpdateTaskRequest\x12\n\n\x02id\x18\x01 \x01(\t\x12\r\n\x05title\x18\x02 \x01(\t\x12\x13\n\x0b\x64\x65scription\x18\x03 \x01(\t\x12\x0e\n\x06status\x18\x04 \x01(\t\"\x1c\n\x0eGetTaskRequest\x12\n\n\x02id\x18\x01 \x01(\t\"(\n\x0cTaskResponse\x12\x18\n\x04task\x18\x01 \x01(\x0b\x32\n.task.Task\"\x1f\n\x11\x44\x65leteTaskRequest\x12\n\n\x02id\x18\x01 \x01(\t\".\n\x11ListTasksResponse\x12\x19\n\x05tasks\x18\x01 \x03(\x0b\x32\n.task.Task\"\x07\n\x05\x45mpty2\x9f\x02\n\x0bTaskService\x12\x39\n\nCreateTask\x12\x17.task.CreateTaskRequest\x1a\x12.task.TaskResponse\x12\x33\n\x07GetTask\x12\x14.task.GetTaskRequest\x1a\x12.task.TaskResponse\x12\x39\n\nUpdateTask\x12\x17.task.UpdateTaskRequest\x1a\x12.task.TaskResponse\x12\x32\n\nDeleteTask\x12\x17.task.DeleteTaskRequest\x1a\x0b.task.Empty\x12\x31\n\tListTasks\x12\x0b.task.Empty\x1a\x17.task.ListTasksResponseb\x06proto3')

_globals = globals()
_builder.BuildMessageAndEnumDescriptors(DESCRIPTOR, _globals)
_builder.BuildTopDescriptorsAndMessages(DESCRIPTOR, 'task_pb2', _globals)
if not _descriptor._USE_C_DESCRIPTORS:
  DESCRIPTOR._loaded_options = None
  _globals['_TASK']._serialized_start=20
  _globals['_TASK']._serialized_end=90
  _globals['_CREATETASKREQUEST']._serialized_start=92
  _globals['_CREATETASKREQUEST']._serialized_end=147
  _globals['_UPDATETASKREQUEST']._serialized_start=149
  _globals['_UPDATETASKREQUEST']._serialized_end=232
  _globals['_GETTASKREQUEST']._serialized_start=234
  _globals['_GETTASKREQUEST']._serialized_end=262
  _globals['_TASKRESPONSE']._serialized_start=264
  _globals['_TASKRESPONSE']._serialized_end=304
  _globals['_DELETETASKREQUEST']._serialized_start=306
  _globals['_DELETETASKREQUEST']._serialized_end=337
  _globals['_LISTTASKSRESPONSE']._serialized_start=339
  _globals['_LISTTASKSRESPONSE']._serialized_end=385
  _globals['_EMPTY']._serialized_start=387
  _globals['_EMPTY']._serialized_end=394
  _globals['_TASKSERVICE']._serialized_start=397
  _globals['_TASKSERVICE']._serialized_end=684
# @@protoc_insertion_point(module_scope)
