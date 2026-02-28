using System.ComponentModel.DataAnnotations;

namespace ProgressiveWorkout.Infrastructure.Auth;

public sealed class WgerAuthTokenOptions
{
    public const string SectionName = "Wger:AuthToken";

    [Required]
    [Url]
    public required string BaseAddress { get; init; }

    [Required]
    [MinLength(1)]
    public required string Username { get; init; }

    [Required]
    [MinLength(1)]
    public required string Password { get; init; }
}
