using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Hosting;
using ProgressiveWorkout.Application.Enums;
using ProgressiveWorkout.Application.Ports;
using ProgressiveWorkout.Infrastructure.Auth;
using ProgressiveWorkout.Infrastructure.Http;
using ProgressiveWorkout.Infrastructure.Workout;

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
        builder.Services.AddKeyedTransient<IAuthTokenService, WgerAuthTokenService>(
            AuthTokenType.Wger
        );

        // Wger workout
        builder
            .Services.AddOptions<WgerWorkoutOptions>()
            .Bind(builder.Configuration.GetSection(WgerWorkoutOptions.SectionName))
            .ValidateDataAnnotations()
            .ValidateOnStart();
        builder
            .Services.AddHttpClient<IWorkoutService, WgerWorkoutService>()
            .AddHttpMessageHandler(provider =>
            {
                IAuthTokenService authTokenService =
                    provider.GetRequiredKeyedService<IAuthTokenService>(AuthTokenType.Wger);
                return new RefreshTokenHandler(authTokenService);
            });

        return builder;
    }
}
