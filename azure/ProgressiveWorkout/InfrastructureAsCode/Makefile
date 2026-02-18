TERRAFORM ?= terraform

include .env

AZUREPLATFORM_CLIENTID := $(AZUREPLATFORM_CLIENTID)
AZUREPLATFORM_TENANTID := $(AZUREPLATFORM_TENANTID)
TF_BACKEND_CONTAINERNAME := $(APPLICATION_SHORTNAME)-$(AZUREPLATFORM_APPLICATIONINSTANCE)-terraform-state
TF_BACKEND_KEY := $(APPLICATION_SHORTNAME)-$(AZUREPLATFORM_APPLICATIONINSTANCE)
TF_BACKEND_STORAGEACCOUNTNAME := $(AZUREPLATFORM_STORAGEACCOUNTNAME)
TF_BACKEND_USEAZUREAUTH := true
TF_PLAN_PATH := ./tfplan
export TF_VAR_application_name := $(APPLICATION_SHORTNAME)
export TF_VAR_application_instance_name := $(AZUREPLATFORM_APPLICATIONINSTANCE)
export TF_VAR_auth_client_id := $(AZUREPLATFORM_AUTHCLIENTID)
export TF_VAR_auth_secret_name := $(AZUREPLATFORM_AUTHSECRETNAME)
export TF_VAR_key_vault_name := $(AZUREPLATFORM_KEYVAULTNAME)
export TF_VAR_resource_group_name := $(AZUREPLATFORM_RESOURCEGROUPNAME)
export TF_VAR_subscription_id := $(AZUREPLATFORM_SUBSCRIPTIONID)

.DEFAULT_GOAL := up

.PHONY: terraform-apply terraform-init terraform-plan terraform-validate down up


terraform-init:
	az storage container create \
		--name "$(TF_BACKEND_CONTAINERNAME)" \
		--account-name "$(TF_BACKEND_STORAGEACCOUNTNAME)" \
		--auth-mode login
	terraform init \
		-backend-config="container_name=$(TF_BACKEND_CONTAINERNAME)" \
		-backend-config="key=$(TF_BACKEND_KEY)" \
		-backend-config="resource_group_name=$(TF_VAR_resource_group_name)" \
		-backend-config="storage_account_name=$(TF_BACKEND_STORAGEACCOUNTNAME)" \
		-backend-config="use_azuread_auth=$(TF_BACKEND_USEAZUREAUTH)"

terraform-validate: terraform-init
	terraform validate

terraform-plan: terraform-validate
	terraform plan \
		-input=false \
		-out=$(TF_PLAN_PATH)

terraform-apply: terraform-plan
	terraform apply \
		-auto-approve \
		-input=false \
		$(TF_PLAN_PATH)

down:
	terraform plan \
		-destroy \
		-input=false \
		-out=$(TF_PLAN_PATH)
	terraform apply \
		-auto-approve \
		-destroy \
		-input=false \
		$(TF_PLAN_PATH)
	az storage container delete \
		--name "$(TF_BACKEND_CONTAINERNAME)" \
		--account-name "$(TF_BACKEND_STORAGEACCOUNTNAME)" \
		--auth-mode login

up: terraform-apply
