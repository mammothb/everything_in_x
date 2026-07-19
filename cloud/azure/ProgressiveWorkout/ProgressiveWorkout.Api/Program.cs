using ProgressiveWorkout.Application.Enums;
using ProgressiveWorkout.Application.Ports;
using ProgressiveWorkout.Infrastructure;
using ProgressiveWorkout.Telemetry;

WebApplicationBuilder builder = WebApplication.CreateBuilder(args);

builder.RegisterTelemetry();
builder.RegisterInfrastructure();
builder.Services.AddHealthChecks();

WebApplication app = builder.Build();

app.MapGet("/", () => "Hello World!");
app.MapGet(
    "/test",
    async ([FromKeyedServices(AuthTokenType.Wger)] IAuthTokenService service) =>
    {
        return await service.GetTokenAsync();
    }
);
app.MapGet(
    "/test-2",
    async (IWorkoutService service) =>
    {
        return await service.GetTemplatesAsync();
    }
);
app.MapHealthChecks("/_health");

app.Run();
