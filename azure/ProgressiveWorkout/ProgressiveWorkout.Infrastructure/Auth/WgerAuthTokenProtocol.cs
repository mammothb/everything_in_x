using System.Text.Json.Serialization;

namespace ProgressiveWorkout.Infrastructure.Auth;

public class TokenObtainRequest
{
    [JsonPropertyName("username")]
    public required string Username { get; init; }

    [JsonPropertyName("password")]
    public required string Password { get; init; }
}

public class TokenObtainResponse
{
    [JsonPropertyName("access")]
    public required string Access { get; init; }

    [JsonPropertyName("refresh")]
    public required string Refresh { get; init; }
}

public class TokenRefreshRequest
{
    [JsonPropertyName("refresh")]
    public required string Refresh { get; init; }
}

public class TokenRefreshResponse
{
    [JsonPropertyName("access")]
    public required string Access { get; init; }
}

public class TokenVerifyRequest
{
    [JsonPropertyName("token")]
    public required string Token { get; init; }
}
