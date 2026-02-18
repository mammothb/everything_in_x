WebApplicationBuilder builder = WebApplication.CreateBuilder(args);

builder.Services.AddHealthChecks();

WebApplication app = builder.Build();

app.MapGet("/", () => "Hello World!");
app.MapHealthChecks("/_health");

app.Run();
