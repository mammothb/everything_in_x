resource "azurerm_application_insights" "main" {
  name                = "${local.prefix}-appinsights"
  location            = data.azurerm_resource_group.main.location
  resource_group_name = data.azurerm_resource_group.main.name
  application_type    = "web"
}
