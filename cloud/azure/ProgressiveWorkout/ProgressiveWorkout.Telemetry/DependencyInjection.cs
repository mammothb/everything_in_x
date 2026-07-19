using Azure.Monitor.OpenTelemetry.AspNetCore;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Hosting;
using Microsoft.Extensions.Logging;
using OpenTelemetry;
using OpenTelemetry.Metrics;
using OpenTelemetry.Trace;

namespace ProgressiveWorkout.Telemetry;

public static class DependencyInjection
{
    public static IHostApplicationBuilder RegisterTelemetry(this IHostApplicationBuilder builder)
    {
        builder.Logging.AddOpenTelemetry();
        builder
            .Services.AddOpenTelemetry()
            .UseOtlpExporter()
            // .UseAzureMonitor()
            .WithTracing(tracing => tracing.AddAspNetCoreInstrumentation())
            .WithMetrics(metrics => metrics.AddAspNetCoreInstrumentation());

        return builder;
    }
}
