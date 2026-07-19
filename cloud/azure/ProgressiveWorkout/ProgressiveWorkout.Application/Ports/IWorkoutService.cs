namespace ProgressiveWorkout.Application.Ports;

public interface IWorkoutService
{
    public Task<string> GetTemplatesAsync();
}
