## ADDED Requirements

### Requirement: Todo Creation and Management
The system SHALL provide Todo creation, editing, deletion, and organization capabilities with status tracking.

#### Scenario: Create new Todo item
- **WHEN** user clicks "＋ New Research Task" button
- **THEN** display modal with title, description, and optional URL fields
- **AND** validate required fields before submission
- **AND** create Todo with "pending" status in database

#### Scenario: Edit existing Todo
- **WHEN** user double-clicks on Todo item or clicks edit button
- **THEN** open edit modal with current Todo data
- **AND** allow modification of all Todo properties
- **AND** update database record on save

#### Scenario: Delete Todo with confirmation
- **WHEN** user clicks delete button on Todo item
- **THEN** show confirmation dialog
- **AND** permanently delete Todo and related research results on confirmation

#### Scenario: Todo status progression
- **WHEN** user changes Todo status
- **THEN** update Todo status in database
- **AND** move Todo between sidebar categories (pending → researching → review → done → archived)
- **AND** update timestamp for status changes

### Requirement: Todo Organization and Filtering
The system SHALL provide Todo organization by status and content-based search capabilities.

#### Scenario: Status-based organization
- **WHEN** user views sidebar navigation
- **THEN** display categorized Todo counts (Pending, Researching, Review, Done, Archived)
- **AND** show Todos in respective categories when selected

#### Scenario: Search and filter functionality
- **WHEN** user types in search input
- **THEN** filter Todos by title, description, and URL content
- **AND** update Todo list in real-time as user types
- **AND** maintain filter state across navigation

### Requirement: Todo Detail Display
The system SHALL display comprehensive Todo information and associated research results.

#### Scenario: View Todo details
- **WHEN** user selects Todo item
- **THEN** show detail panel with full Todo information
- **AND** display associated research results in Markdown format
- **AND** show research metadata (service, duration, timestamp)
- **AND** provide action buttons for research and status changes