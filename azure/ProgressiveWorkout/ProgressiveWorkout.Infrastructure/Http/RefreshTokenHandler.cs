using System.Net.Http.Headers;
using ProgressiveWorkout.Application.Ports;

namespace ProgressiveWorkout.Infrastructure.Http;

public class RefreshTokenHandler(IAuthTokenService authTokenService) : DelegatingHandler
{
    private readonly IAuthTokenService _authTokenService = authTokenService;

    protected override async Task<HttpResponseMessage> SendAsync(
        HttpRequestMessage request,
        CancellationToken cancellationToken
    )
    {
        string token = await _authTokenService.GetTokenAsync();
        request.Headers.Authorization = new AuthenticationHeaderValue("Bearer", token);

        return await base.SendAsync(request, cancellationToken);
    }
}
