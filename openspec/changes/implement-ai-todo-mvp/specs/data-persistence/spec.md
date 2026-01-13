## ADDED Requirements

### Requirement: SQLite Database Schema
The system SHALL implement persistent storage using SQLite with proper schema migrations.

#### Scenario: Initialize database on first launch
- **WHEN** application starts for first time
- **THEN** create SQLite database in user data directory
- **AND** execute schema migration scripts
- **AND** initialize default settings if needed
- **AND** verify database integrity

#### Scenario: Database schema migrations
- **WHEN** application version includes schema changes
- **THEN** execute migration scripts in version order
- **AND** update schema version table
- **AND** handle migration failures gracefully with rollback

### Requirement: Todo Data Persistence
The system SHALL provide reliable Todo data storage with proper relationships.

#### Scenario: Store Todo item
- **WHEN** user creates or updates Todo
- **THEN** insert/update record in todos table
- **AND** include all Todo fields (id, title, description, url, status, priority, service preference)
- **AND** set appropriate timestamps (created_at, updated_at, completed_at)
- **AND** enforce data constraints and validation

#### Scenario: Retrieve Todo collections
- **WHEN** UI requests Todo lists
- **THEN** query todos table with optional filters
- **AND** support filtering by status, priority, date ranges
- **AND** return results sorted by creation date and priority
- **AND** include associated research counts

### Requirement: Research Results Storage
The system SHALL persist research results with metadata for historical reference.

#### Scenario: Store research results
- **WHEN** research automation completes
- **THEN** insert record in research_results table
- **AND** store content as Markdown with metadata as JSON
- **AND** link to parent Todo item with foreign key constraint
- **AND** include service information and timing data

#### Scenario: Query research history
- **WHEN** user views Todo details
- **THEN** retrieve all research results for specific Todo
- **AND** order by completion date (newest first)
- **AND** include metadata for display formatting

### Requirement: Settings and Configuration
The system SHALL maintain application settings with user preferences.

#### Scenario: Persist user preferences
- **WHEN** user changes application settings
- **THEN** update settings table with new values
- **AND** validate setting values before storage
- **AND** emit change events to update running application

#### Scenario: Load application configuration
- **WHEN** application starts
- **THEN** load all settings from database
- **AND** apply settings to application state
- **AND** use fallback defaults for missing settings