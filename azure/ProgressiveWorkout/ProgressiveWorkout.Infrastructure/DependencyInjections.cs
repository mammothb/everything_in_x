using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Hosting;
using ProgressiveWorkout.Infrastructure.Auth;

namespace ProgressiveWorkout.Infrastructure;

public static class DependencyInjection
{
    public static IHostApplicationBuilder RegisterInfrastructure(
        this IHostApplicationBuilder builder
    )
    {
        builder.Services.AddMemoryCache();
        // Wger token
        builder
            .Services.AddOptions<WgerAuthTokenOptions>()
            .Bind(builder.Configuration.GetSection(WgerAuthTokenOptions.SectionName))
            .ValidateDataAnnotations()
            .ValidateOnStart();
        builder.Services.AddHttpClient<WgerAuthTokenService>();

        return builder;
    }
}
