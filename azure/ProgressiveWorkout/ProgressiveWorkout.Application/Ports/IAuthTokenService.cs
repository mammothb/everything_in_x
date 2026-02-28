namespace ProgressiveWorkout.Application.Ports;

public interface IAuthTokenService
{
    public Task<string> GetTokenAsync();
}
