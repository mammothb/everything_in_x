# Configure the Microsoft Azure Provider
provider "azurerm" {
  # This is only required when the User, Service Principal, or Identity running
  # Terraform lacks the permissions to register Azure Resource Providers.
  resource_provider_registrations = "none"
  features {}
}
