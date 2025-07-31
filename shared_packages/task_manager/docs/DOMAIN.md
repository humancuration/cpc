# Task Manager Domain Model

## Core Entities

### Task
- **id**: Unique identifier (UUID)
- **title**: Task name
- **description**: Detailed task description
- **due_date**: Optional deadline
- **priority**: Enum (Low, Medium, High, Critical)
- **status**: Enum (Not Started, In Progress, Blocked, Completed)
- **progress**: Percentage complete (0-100)
- **assignee**: User ID reference
- **dependencies**: List of task IDs that must be completed first
- **recurrence**: Pattern for recurring tasks (daily, weekly, monthly)
- **created_at**: Timestamp
- **updated_at**: Timestamp

### Project
- **id**: Unique identifier
- **name**: Project title
- **description**: Project overview
- **tasks**: Collection of associated tasks
- **team**: Assigned team members
- **visibility**: Public/Private/Team-only

### Team
- **id**: Unique identifier
- **name**: Team name
- **members**: List of user IDs
- **permissions**: Role-based access control

## Value Objects
- **DueDate**: Validated date wrapper
- **ProgressPercentage**: 0-100 range validator
- **RecurrenceRule**: Pattern parser for recurring tasks
- **DependencyGraph**: Ensures no circular dependencies

## Domain Services
1. **TaskAssignmentService**: Handles task assignment logic
2. **DependencyResolver**: Manages task dependencies
3. **RecurrenceGenerator**: Creates recurring task instances
4. **ProgressCalculator**: Updates completion percentages

## Rules
- Tasks can't depend on themselves
- Completed tasks can't be modified
- High-priority tasks must be assigned
- Recurring tasks generate new instances upon completion