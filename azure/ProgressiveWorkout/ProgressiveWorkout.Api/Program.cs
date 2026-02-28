using ProgressiveWorkout.Application.Ports;
using ProgressiveWorkout.Infrastructure;
using ProgressiveWorkout.Infrastructure.Auth;
using ProgressiveWorkout.Telemetry;

WebApplicationBuilder builder = WebApplication.CreateBuilder(args);

builder.RegisterTelemetry();
builder.RegisterInfrastructure();
builder.Services.AddHealthChecks();

builder.Services.AddScoped<IAuthTokenService, WgerAuthTokenService>();

WebApplication app = builder.Build();

app.MapGet("/", () => "Hello World!");
app.MapGet(
    "/test",
    async (IAuthTokenService service) =>
    {
        return await service.GetTokenAsync();
    }
);
app.MapHealthChecks("/_health");

app.Run();
