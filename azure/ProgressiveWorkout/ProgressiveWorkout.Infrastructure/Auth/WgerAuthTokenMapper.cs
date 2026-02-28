namespace ProgressiveWorkout.Infrastructure.Auth;

public static class WgerAuthTokenMapper
{
    public static Token Map(TokenObtainResponse response)
    {
        return new Token { Access = response.Access, Refresh = response.Refresh };
    }

    public static Token Map(TokenRefreshResponse response, string refreshToken)
    {
        return new Token { Access = response.Access, Refresh = refreshToken };
    }
}
