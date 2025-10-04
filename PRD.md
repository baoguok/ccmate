CC config is a configuration file manager for Claude Code.


## Claude Code configuration files definition

- **User settings** are defined in `~/.claude/settings.json` and apply to all projects.
- **Project settings** are saved in your project directory:
	- `.claude/settings.json` for settings that are checked into source control and shared with your team
	- `.claude/settings.local.json` for settings that are not checked in, useful for personal preferences and experimentation. Claude Code will configure git to ignore `.claude/settings.local.json` when it is created.
- For enterprise deployments of Claude Code, we also support **enterprise managed policy settings**. These take precedence over user and project settings. System administrators can deploy policies to:
	- macOS: `/Library/Application Support/ClaudeCode/managed-settings.json`
	- Linux and WSL: `/etc/claude-code/managed-settings.json`
	- Windows: `C:\ProgramData\ClaudeCode\managed-settings.json`
- Enterprise deployments can also configure **managed MCP servers** that override user-configured servers. See [Enterprise MCP configuration](https://docs.claude.com/en/docs/claude-code/mcp#enterprise-mcp-configuration):
	- macOS: `/Library/Application Support/ClaudeCode/managed-mcp.json`
	- Linux and WSL: `/etc/claude-code/managed-mcp.json`
	- Windows: `C:\ProgramData\ClaudeCode\managed-mcp.json`


In this phase, only do the follwing tasks:

- Implement the Tauri commands and react-query/mutation that read and write the configuration files.
- A minimal UI that allows you to view and edit the configuration files.