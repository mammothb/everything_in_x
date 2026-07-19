using EverythingServer.Mcp.Tools;
using OpenTelemetry;
using OpenTelemetry.Metrics;
using OpenTelemetry.Trace;

WebApplicationBuilder builder = WebApplication.CreateBuilder(args);
builder.Services.AddMcpServer().WithHttpTransport().WithTools<EchoTool>();
builder.Services.AddHttpContextAccessor();
builder.Services.AddHttpLogging(o =>
{
    o.LoggingFields = Microsoft
        .AspNetCore
        .HttpLogging
        .HttpLoggingFields
        .RequestPropertiesAndHeaders;
    o.RequestHeaders.Add("Authorization");
    o.RequestHeaders.Add("X-Third-Party-Key");
});
builder
    .Services.AddOpenTelemetry()
    .WithTracing(b =>
        b.AddSource("*").AddAspNetCoreInstrumentation().AddHttpClientInstrumentation()
    )
    .WithMetrics(b => b.AddMeter("*").AddAspNetCoreInstrumentation().AddHttpClientInstrumentation())
    .WithLogging()
    .UseOtlpExporter();

WebApplication app = builder.Build();

app.UseHttpLogging();

app.MapGet("/", () => "Hello World!");
app.MapMcp("/mcp");

app.Run();
