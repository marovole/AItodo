# Change: Implement AI Todo System MVP

## Why
Transform the existing ChatGPT desktop application into an intelligent Todo system that automatically researches tasks using AI services, turning the app from a simple ChatGPT wrapper into a productivity powerhouse.

## What Changes
- **BREAKING**: Transform core app functionality from ChatGPT wrapper to AI Todo System
- Add SQLite database for Todo and research result persistence  
- Implement three-panel layout (sidebar, todo list, detail panel) replacing current ChatGPT view
- Add Todo CRUD operations with status management
- Implement ChatGPT WebView automation for deep research capabilities
- Add research progress monitoring and result display
- Integrate system notifications for research completion
- Replace ask.js with research automation scripts

## Impact
- **Affected specs**: All functionality will be replaced with Todo system capabilities
- **Affected code**: Core application architecture, all React components, Rust backend modules
- **Database**: New SQLite layer with Todo, research_results, settings, webview_sessions tables
- **UI**: Complete redesign from ChatGPT wrapper to Todo management interface