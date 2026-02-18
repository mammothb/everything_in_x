resource "azurerm_service_plan" "linux" {
  name                = "${local.prefix}-linux-asp"
  location            = data.azurerm_resource_group.main.location
  os_type             = "Linux"
  resource_group_name = data.azurerm_resource_group.main.name
  sku_name            = "F1"
}

resource "azurerm_linux_web_app" "api" {
  name                = "${local.prefix}-api-web-app"
  location            = data.azurerm_resource_group.main.location
  resource_group_name = data.azurerm_resource_group.main.name
  service_plan_id     = azurerm_service_plan.linux.id

  site_config {
    always_on                         = false
    ftps_state                        = "Disabled"
    health_check_path                 = "/_health"
    health_check_eviction_time_in_min = 5
    http2_enabled                     = true
    minimum_tls_version               = "1.2"
    scm_minimum_tls_version           = "1.2"
    use_32_bit_worker                 = true

    application_stack {
      dotnet_version = "10.0"
    }
  }

  https_only = true

  app_settings = {
    AZUREAD_AUTH_CLIENT_SECRET       = "@Microsoft.KeyVault(VaultName=${data.azurerm_key_vault.main.name};SecretName=${var.auth_secret_name})"
    WEBSITE_AUTH_AAD_ALLOWED_TENANTS = data.azurerm_client_config.main.tenant_id
  }

  auth_settings_v2 {
    auth_enabled           = true
    require_authentication = true
    require_https          = true
    runtime_version        = "~1"
    unauthenticated_action = "RedirectToLoginPage"

    default_provider = "azureactivedirectory"
    excluded_paths   = ["/_health"]

    active_directory_v2 {
      client_id                  = var.auth_client_id
      tenant_auth_endpoint       = "https://login.microsoftonline.com/${data.azurerm_client_config.main.tenant_id}/v2.0"
      allowed_applications       = [var.auth_client_id]
      allowed_audiences          = ["api://${var.auth_client_id}"]
      client_secret_setting_name = "AZUREAD_AUTH_CLIENT_SECRET"
    }
    login {
      token_store_enabled = true
    }
  }

  identity {
    type = "SystemAssigned"
  }
}

resource "azurerm_role_assignment" "api_web_app_key_vault" {
  scope                = data.azurerm_key_vault.main.id
  principal_id         = azurerm_linux_web_app.api.identity[0].principal_id
  role_definition_name = "Key Vault Secrets User"
  description          = "Allow the web app to read Key Vault secrets"
}
