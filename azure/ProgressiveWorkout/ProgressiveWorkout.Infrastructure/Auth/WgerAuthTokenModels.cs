namespace ProgressiveWorkout.Infrastructure.Auth;

public class Token
{
    public required string Access { get; init; }

    public required string Refresh { get; init; }
}
