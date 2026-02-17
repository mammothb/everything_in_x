MAKE ?= make

.DEFAULT_GOAL := infra-up

.PHONY: infra-down infra-up

infra-down:
	$(MAKE) down -C ./InfrastructureAsCode

infra-up:
	$(MAKE) up -C ./InfrastructureAsCode
