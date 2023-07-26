#!/bin/bash
#echo DATABASE_URL=/home/wolfgang/hsqldb/csbp.db > .env
diesel --database-url /home/wolfgang/hsqldb/csbp.db print-schema >rep/src/schema.rs
#diesel print-schema --custom-type-derives "diesel::sql_types::SqlType", "std::fmt::Debug" >rep/src/schema.rs
#diesel migration run