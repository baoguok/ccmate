## Tech Stack

- This is a Tauri v2 app. 
- Always use pnpm to install dependencies.
- Use TypeScript for frontend code. Do not use any type.
- Must use @tanstack/react-query for data fetching, mutations, and asynchronous tasks. All query and mutation should be defined in the `lib/query.ts` file by default.
- Use React Router for navigation.
- Use React Hook Form for form handling.
- Use shadcn/ui and tailwindcss(v4) for styling

## React code principles

- Use functional components and hooks.
- Do not use export default to export components.
- Carefully design whether to put the react query/mutation logic in the component file or in the `lib/query.ts` file.
- Carefully design whether to put the form handling logic in the component file or in the `lib/form.ts` file.
- Do not separate components into smaller files unless I told you to do so.

## Code architecture guide

- You should write Tauri commands in the `src-tauri/src/commands.rs` file with a well designed command name.

## Use exa by Default
Always use exa when I need code generation, library installation, setup or configuration steps, or library/API documentation. This means you should automatically use the exa MCP tools get library docs without me having to explicitly ask.