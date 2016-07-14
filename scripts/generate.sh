#!/bin/sh
protoc --rust_out ./src/protocol ./src/protocol/messages.proto
