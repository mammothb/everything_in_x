BUILD ?= Release
MAKE ?= make

IAC_DIR := ./InfrastructureAsCode
PROJECT_NAME := ProgressiveWorkout.Api
PROJECT_PATH := ./$(PROJECT_NAME)/$(PROJECT_NAME).csproj
PUBLISH_DIR := ./publish

.DEFAULT_GOAL := infra-up

.PHONY: app-build app-deploy app-publish app-run clean infra-down infra-up monitor

app-build:
	dotnet restore
	dotnet build $(PROJECT_PATH) --no-restore -c $(BUILD)

app-run: app-build
	dotnet run --project $(PROJECT_PATH) --no-build -c $(BUILD)

app-publish: app-build
	dotnet publish $(PROJECT_PATH) --no-build -c $(BUILD) -o $(PUBLISH_DIR)

app-deploy: app-publish
	cd $(PUBLISH_DIR) && zip -r ../$(PROJECT_NAME).zip .
	resource_group_name=$$(cd $(IAC_DIR) && terraform output -raw resource_group_name); \
		api_web_app_name=$$(cd $(IAC_DIR) && terraform output -raw api_web_app_name); \
		az webapp deploy --resource-group $${resource_group_name} --name $${api_web_app_name} --src-path ./$(PROJECT_NAME).zip --type zip

monitor:
	docker run --rm --name jaeger \
		-e COLLECTOR_ZIPKIN_HOST_PORT=:9411 \
		-p 16686:16686 \
		-p 4317:4317 \
		-p 4318:4318 \
		-p 14250:14250 \
		-p 14268:14268 \
		-p 14269:14269 \
		-p 9411:9411 \
		jaegertracing/all-in-one:1.76.0


clean:
	dotnet clean
	rm -rf $(PUBLISH_DIR) ./$(PROJECT_NAME)/bin ./$(PROJECT_NAME)/obj ./$(PROJECT_NAME).zip

infra-down:
	$(MAKE) down -C $(IAC_DIR)

infra-up:
	$(MAKE) up -C $(IAC_DIR)
