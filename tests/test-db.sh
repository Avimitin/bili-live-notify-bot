#!/bin/bash

# test table rooms
# test should be run step by step to avoid conflict
cargo test --test live_room_test -- --test-threads 1
