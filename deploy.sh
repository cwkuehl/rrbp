#!/bin/bash
cargo build -r
# /opt/Haushalt/rrbp/Rocket.toml
cp -f ~/rust/rrbp/target/release/rrbp /opt/Haushalt/rrbp
cp -f ~/rust/rrbp/#rrbp.sh /opt/Haushalt/rrbp
cp -rf ~/rust/rrbp/templates /opt/Haushalt/rrbp
#cp -f /opt/Haushalt/CSBP/cert/cert.pem /opt/Haushalt/CSBP/cert/cert.key /opt/Haushalt/rrbp/cert
# ~/hsqldb/rsbp.db
