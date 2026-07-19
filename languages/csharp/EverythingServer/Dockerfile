# =============================================================================
# Build
# =============================================================================
FROM mcr.microsoft.com/dotnet/sdk:10.0-alpine AS build

WORKDIR /src

COPY Directory.Packages.props \
    global.json \
    ./

COPY EverythingServer.Mcp/EverythingServer.Mcp.csproj \
    ./EverythingServer.Mcp/

RUN dotnet restore \
    EverythingServer.Mcp/EverythingServer.Mcp.csproj

COPY . .

RUN dotnet publish \
    EverythingServer.Mcp/EverythingServer.Mcp.csproj \
    --no-restore \
    --output /app

# =============================================================================
# Run
# =============================================================================
FROM mcr.microsoft.com/dotnet/aspnet:10.0-alpine

WORKDIR /app

COPY --from=build /app .

USER $APP_UID

EXPOSE 8080

ENTRYPOINT ["./EverythingServer.Mcp"]
