#!/bin/bash
diesel --database-url /home/wolfgang/hsqldb/rsbp.db print-schema >rep/src/schema.rs
