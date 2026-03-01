using System.Text.Json.Serialization;

namespace ProgressiveWorkout.Infrastructure.Auth;

public class ObtainTokenRequest
{
    [JsonPropertyName("username")]
    public required string Username { get; init; }

    [JsonPropertyName("password")]
    public required string Password { get; init; }
}

public class ObtainTokenResponse
{
    [JsonPropertyName("access")]
    public required string Access { get; init; }

    [JsonPropertyName("refresh")]
    public required string Refresh { get; init; }
}

public class RefreshTokenRequest
{
    [JsonPropertyName("refresh")]
    public required string Refresh { get; init; }
}

public class RefreshTokenResponse
{
    [JsonPropertyName("access")]
    public required string Access { get; init; }
}

public class VerifyTokenRequest
{
    [JsonPropertyName("token")]
    public required string Token { get; init; }
}
