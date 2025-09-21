#!/usr/bin/env bash

set -euo pipefail

. .env

suffix="${1:+-${1}}"

yq -r '.stacks[] | [.name, .template_path] | @tsv' ./definition.yml \
    | while IFS=$'\t' read name template_path; do
        stack_name="${name}${suffix}"
        echo "Deploying '${stack_name}' using '${template_path}'"
        aws cloudformation deploy \
            --stack-name "${stack_name}" \
            --template-file "${template_path}"
    done
