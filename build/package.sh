#!/bin/bash

OS=$1
RLS=$2
VERSION=`cat Cargo.toml | grep -m 1 "version =" | cut -d'"' -f 2`

OUTPUT_NAME=nri-universal-discovery-${OS}-${VERSION}

TEMPDIR=`mktemp -d`
BUILD_DIR=${TEMPDIR}/${OUTPUT_NAME}

[ -f "${OUTPUT_NAME}.tar.gz" ] && rm -rf ${OUTPUT_NAME}.tar.gz
mkdir -p ${BUILD_DIR}

cp ./target/${RLS}/release/nri-universal-discovery ${BUILD_DIR}/
cp ./README.md ${BUILD_DIR}/
cp -a ./docs ${BUILD_DIR}/

[ -f "./scripts/install_${OS}.sh" ] && cp ./scripts/install_${OS}.sh ${BUILD_DIR}/

# Create the gzip
tar -C ${TEMPDIR} -czf ${OUTPUT_NAME}.tar.gz ${OUTPUT_NAME}/

# Cleanup
rm -rf ${BUILD_DIR}