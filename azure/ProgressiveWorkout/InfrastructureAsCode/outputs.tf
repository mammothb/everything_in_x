output "api_web_app_name" {
  value       = azurerm_linux_web_app.api.name
  description = "API web app name"
}

output "resource_group_name" {
  value       = data.azurerm_resource_group.main.name
  description = "Resource group name"
}

