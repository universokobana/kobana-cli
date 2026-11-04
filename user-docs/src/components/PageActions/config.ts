// The only file of PageActions that differs between docs sites.

export const PAGE_ACTIONS = {
  // Name the MCP install links and snippets register in the client.
  mcpServerName: 'kobana-cli-docs',
  // MCP server of THIS documentation (api/mcp.ts, routed from /mcp), not the
  // product's MCP; `null` hides the MCP section.
  mcpUrl: 'https://cli.kobana.com.br/mcp' as string | null,
  // Docs assistant on this site (components/Assistant + api/assistant.ts):
  // adds "Ask AI" to the menu, which fires the `kobana:ask-ai` event.
  assistant: true,
  // Routes the llms-txt plugin skips (its `excludeRoutes`): they have no
  // sibling `.md`, so the Markdown items fall back or hide there.
  markdownExcluded: [] as RegExp[],
};
