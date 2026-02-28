using System.Net.Http.Json;
using System.Text;
using System.Text.Json;
using Microsoft.Extensions.Caching.Memory;
using Microsoft.Extensions.Options;
using ProgressiveWorkout.Application.Ports;

namespace ProgressiveWorkout.Infrastructure.Auth;

public sealed class WgerAuthTokenService : IAuthTokenService
{
    private readonly string _cacheKey = "WgerAuthTokenService:Response";
    private readonly WgerAuthTokenOptions _options;
    private readonly HttpClient _client;
    private readonly IMemoryCache _cache;

    public WgerAuthTokenService(
        IOptions<WgerAuthTokenOptions> options,
        HttpClient client,
        IMemoryCache cache
    )
    {
        _options = options.Value;
        _client = client;
        _cache = cache;

        const char UrlDelimiter = '/';
        _client.BaseAddress = new Uri(_options.BaseAddress.TrimEnd(UrlDelimiter) + UrlDelimiter);
    }

    public async Task<string> GetTokenAsync()
    {
        Token? cachedToken = await GetCachedToken();
        if (cachedToken is not null)
        {
            return cachedToken.Access;
        }

        Token token = await ObtainTokenAsync();
        CacheToken(token);
        return token.Access;
    }

    private async Task<Token?> GetCachedToken()
    {
        if (_cache.TryGetValue(_cacheKey, out Token? token) && token is not null)
        {
            if (await VerifyTokenAsync(token.Access))
            {
                return token;
            }
            return await RefreshTokenAsync(token.Refresh);
        }
        return token;
    }

    private async Task<Token> ObtainTokenAsync()
    {
        var body = new TokenObtainRequest
        {
            Username = _options.Username,
            Password = _options.Password,
        };
        var request = new HttpRequestMessage(HttpMethod.Post, "token")
        {
            Content = new StringContent(
                JsonSerializer.Serialize(body),
                Encoding.UTF8,
                "application/json"
            ),
        };
        using HttpResponseMessage response = await _client.SendAsync(request);
        response.EnsureSuccessStatusCode();

        TokenObtainResponse tokenResponse =
            await response.Content.ReadFromJsonAsync<TokenObtainResponse>()
            ?? throw new JsonException("Failed to parse token response");
        return WgerAuthTokenMapper.Map(tokenResponse);
    }

    private async Task<Token> RefreshTokenAsync(string refreshToken)
    {
        var body = new TokenRefreshRequest { Refresh = refreshToken };
        var request = new HttpRequestMessage(HttpMethod.Post, "token/refresh")
        {
            Content = new StringContent(
                JsonSerializer.Serialize(body),
                Encoding.UTF8,
                "application/json"
            ),
        };
        using HttpResponseMessage response = await _client.SendAsync(request);
        response.EnsureSuccessStatusCode();

        TokenRefreshResponse tokenResponse =
            await response.Content.ReadFromJsonAsync<TokenRefreshResponse>()
            ?? throw new JsonException("Failed to parse token response");

        Token token = WgerAuthTokenMapper.Map(tokenResponse, refreshToken);
        CacheToken(token);
        return token;
    }

    private async Task<bool> VerifyTokenAsync(string token)
    {
        var body = new TokenVerifyRequest { Token = token };
        var request = new HttpRequestMessage(HttpMethod.Post, "token/verify")
        {
            Content = new StringContent(
                JsonSerializer.Serialize(body),
                Encoding.UTF8,
                "application/json"
            ),
        };
        using HttpResponseMessage response = await _client.SendAsync(request);
        return response.IsSuccessStatusCode;
    }

    private void CacheToken(Token token)
    {
        _cache.Set(_cacheKey, token);
    }
}
