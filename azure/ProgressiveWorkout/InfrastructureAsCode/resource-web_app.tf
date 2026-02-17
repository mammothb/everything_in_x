resource "azurerm_service_plan" "linux" {
  name = "${var.application_name}-${var.application_instance_name}-linux-asp"
  location = data.azurerm_resource_group.main.location
  os_type = "Linux"
  resource_group_name = data.azurerm_resource_group.main.name
  sku_name = "F1"
}

resource "azurerm_linux_web_app" "api" {
  name = "${var.application_name}-${var.application_instance_name}-api-web-app"
  location = data.azurerm_resource_group.main.location
  resource_group_name = data.azurerm_resource_group.main.name
  service_plan_id = azurerm_service_plan.linux.id
  site_config {
    always_on = false
    application_stack {
      dotnet_version = "10.0"
    }
  }
}
