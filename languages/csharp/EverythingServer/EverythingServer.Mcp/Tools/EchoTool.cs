using System.ComponentModel;
using ModelContextProtocol.Server;

namespace EverythingServer.Mcp.Tools;

[McpServerToolType]
public sealed class EchoTool(IHttpContextAccessor httpContextAccessor)
{
    private readonly IHttpContextAccessor _httpContextAccessor = httpContextAccessor;

    [McpServerTool, Description("Echoes the input back to the client.")]
    public string Echo(string message)
    {
        if (
            _httpContextAccessor.HttpContext?.Request.Headers.ContainsKey("x-third-party-key")
            ?? false
        )
        {
            return "x-third-party-key "
                + _httpContextAccessor.HttpContext.Request.Headers["x-third-party-key"];
        }
        else
        {
            return "hello " + message;
        }
    }
}
