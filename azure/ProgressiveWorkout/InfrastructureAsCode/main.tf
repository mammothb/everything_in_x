# Configure the Microsoft Azure Provider
provider "azurerm" {
  # This is only required when the User, Service Principal, or Identity running
  # Terraform lacks the permissions to register Azure Resource Providers.
  resource_provider_registrations = "none"
  subscription_id                 = var.subscription_id
  features {}
}

data "azurerm_client_config" "main" {}

data "azurerm_key_vault" "main" {
  name                = var.key_vault_name
  resource_group_name = data.azurerm_resource_group.main.name
}

data "azurerm_resource_group" "main" {
  name = var.resource_group_name
}

locals {
  prefix = "${var.application_name}-${var.application_instance_name}"
}
