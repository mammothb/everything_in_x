namespace ProgressiveWorkout.Infrastructure.Auth;

public static class WgerAuthTokenMapper
{
    public static Token Map(ObtainTokenResponse response)
    {
        return new Token { Access = response.Access, Refresh = response.Refresh };
    }

    public static Token Map(RefreshTokenResponse response, string refreshToken)
    {
        return new Token { Access = response.Access, Refresh = refreshToken };
    }
}
