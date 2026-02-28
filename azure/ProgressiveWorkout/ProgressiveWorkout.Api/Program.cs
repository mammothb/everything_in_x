using ProgressiveWorkout.Telemetry;

WebApplicationBuilder builder = WebApplication.CreateBuilder(args);

builder.RegisterTelemetry();
builder.Services.AddHealthChecks();

WebApplication app = builder.Build();

app.MapGet("/", () => "Hello World!");
app.MapHealthChecks("/_health");

app.Run();
