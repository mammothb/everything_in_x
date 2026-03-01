using System.ComponentModel.DataAnnotations;

namespace ProgressiveWorkout.Infrastructure.Workout;

public sealed class WgerWorkoutOptions
{
    public const string SectionName = "Wger:Workout";

    [Required]
    [Url]
    public required string BaseAddress { get; init; }
}
