#!/usr/bin/env bash

set -euo pipefail

. .env

STACK_SUFFIX="${STACK_SUFFIX:-}"

yq -r '.stacks[] | [.name, .template_path] | @tsv' ./definition.yml \
    | while IFS=$'\t' read name template_path; do
        stack_name="${name}${STACK_SUFFIX}"
        echo "Deploying '${stack_name}' using '${template_path}'"
        aws cloudformation deploy \
            --stack-name "${stack_name}" \
            --template-file "${template_path}" \
            --parameter-overrides StackSuffix="${STACK_SUFFIX}"
    done
