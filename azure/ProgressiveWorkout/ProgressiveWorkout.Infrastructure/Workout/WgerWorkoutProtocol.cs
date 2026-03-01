using System.Text.Json.Serialization;

namespace ProgressiveWorkout.Infrastructure.Workout;

public class GetTemplatesQuery
{
    [JsonPropertyName("created")]
    public DateTime? Created { get; init; }

    [JsonPropertyName("description")]
    public string? Description { get; init; }

    // Number of results per page
    [JsonPropertyName("limit")]
    public int? Limit { get; init; }

    [JsonPropertyName("name")]
    public string? Name { get; init; }

    // The initial index from which to return the results
    [JsonPropertyName("offset")]
    public int? Offset { get; init; }

    // Which field to use when ordering the results
    [JsonPropertyName("ordering")]
    public string? Ordering { get; init; }
}

public class GetTemplatesResponse
{
    [JsonPropertyName("count")]
    public required int Count { get; init; }

    [JsonPropertyName("next")]
    public string? Next { get; init; }

    [JsonPropertyName("previous")]
    public string? Previous { get; init; }

    [JsonPropertyName("results")]
    public required List<RoutineResponse> Name { get; init; }
}

public class RoutineResponse
{
    [JsonPropertyName("id")]
    public required int Id { get; init; }

    [JsonPropertyName("name")]
    public string? Name { get; init; }

    [JsonPropertyName("description")]
    public string? Description { get; init; }

    [JsonPropertyName("created")]
    public required DateTime Created { get; init; }

    [JsonPropertyName("start")]
    public required DateTime Start { get; init; }

    [JsonPropertyName("end")]
    public required DateTime End { get; init; }

    [JsonPropertyName("fit_in_week")]
    public bool? FitInWeek { get; init; }

    [JsonPropertyName("is_template")]
    public bool? IsTemplate { get; init; }

    [JsonPropertyName("is_public")]
    public bool? IsPublic { get; init; }
}
