using System.Net.Http.Json;
using System.Text.Json;
using Microsoft.AspNetCore.WebUtilities;
using Microsoft.Extensions.Options;
using ProgressiveWorkout.Application.Ports;

namespace ProgressiveWorkout.Infrastructure.Workout;

public sealed class WgerWorkoutService : IWorkoutService
{
    private readonly WgerWorkoutOptions _options;
    private readonly HttpClient _client;

    public WgerWorkoutService(IOptions<WgerWorkoutOptions> options, HttpClient client)
    {
        _options = options.Value;
        _client = client;

        const char UrlDelimiter = '/';
        _client.BaseAddress = new Uri(_options.BaseAddress.TrimEnd(UrlDelimiter) + UrlDelimiter);
    }

    public async Task<string> GetTemplatesAsync()
    {
        var queryObj = new GetTemplatesQuery { };
        string queryString = JsonSerializer.Serialize(queryObj);
        Dictionary<string, string?> queryDict =
            JsonSerializer.Deserialize<Dictionary<string, string?>>(queryString)
            ?? throw new JsonException("Failed to convert templates query");
        string requestUri = QueryHelpers.AddQueryString("templates/", queryDict);

        var request = new HttpRequestMessage(HttpMethod.Get, requestUri);
        using HttpResponseMessage response = await _client.SendAsync(request);
        response.EnsureSuccessStatusCode();

        GetTemplatesResponse responseObj =
            await response.Content.ReadFromJsonAsync<GetTemplatesResponse>()
            ?? throw new JsonException("Failed to parse templates response");

        return JsonSerializer.Serialize(responseObj);
    }
}
