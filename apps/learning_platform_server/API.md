# Learning Platform Server API Documentation

## Overview

The Learning Platform Server provides gRPC services for managing courses, enrollments, credentials, and tipping functionality.

## Services

### CourseService

#### ListCourses
- **Description**: List all available courses
- **Request**: `ListCoursesRequest`
- **Response**: `ListCoursesResponse`
- **Errors**: Internal server error

#### CreateCourse
- **Description**: Create a new course
- **Request**: `CreateCourseRequest`
- **Response**: `CreateCourseResponse`
- **Errors**: 
  - InvalidArgument: Missing or invalid course data
  - Internal: Database error

#### UpdateCourse
- **Description**: Update an existing course
- **Request**: `UpdateCourseRequest`
- **Response**: `UpdateCourseResponse`
- **Errors**: 
  - InvalidArgument: Missing or invalid course data
  - Internal: Database error

### EnrollmentService

#### Enroll
- **Description**: Enroll a user in a course
- **Request**: `EnrollRequest`
- **Response**: `EnrollResponse`
- **Errors**: 
  - InvalidArgument: Invalid user or course ID
  - NotFound: Course not found
  - AlreadyExists: User already enrolled
  - Internal: Database error

#### UpdateProgress
- **Description**: Update user's progress in a course
- **Request**: `UpdateProgressRequest`
- **Response**: `UpdateProgressResponse`
- **Errors**: 
  - InvalidArgument: Invalid enrollment ID or progress value
  - NotFound: Enrollment not found
  - Internal: Database error

### CredentialService

#### IssueCredential
- **Description**: Issue a credential for course completion
- **Request**: `IssueCredentialRequest`
- **Response**: `IssueCredentialResponse`
- **Errors**: 
  - InvalidArgument: Invalid user or course ID, or invalid credential type
  - NotFound: Course or enrollment not found
  - FailedPrecondition: User has not completed the course
  - Internal: Database error

### TipService

#### SendTip
- **Description**: Send tip to content creator
- **Request**: `SendTipRequest`
- **Response**: `SendTipResponse`
- **Errors**: 
  - InvalidArgument: Invalid sender or recipient ID, or invalid amount
  - NotFound: Course not found (if course ID provided)
  - Internal: Database error

### AuthService

#### Authenticate
- **Description**: Authenticate user and get access token
- **Request**: `AuthRequest`
- **Response**: `AuthResponse`
- **Errors**: 
  - InvalidArgument: Missing username or password
  - Unauthenticated: Invalid username or password
  - Internal: Database or token generation error

### UserService

#### RegisterUser
- **Description**: Register a new user
- **Request**: `RegisterUserRequest`
- **Response**: `RegisterUserResponse`
- **Errors**: 
  - InvalidArgument: Missing or invalid user data
  - AlreadyExists: Username already exists
  - Internal: Database or password hashing error

### HealthService

#### Check
- **Description**: Check service health
- **Request**: `HealthCheckRequest`
- **Response**: `HealthCheckResponse`
- **Errors**: None

#### Watch
- **Description**: Watch service health (not implemented)
- **Request**: `HealthCheckRequest`
- **Response**: Streaming `HealthCheckResponse`
- **Errors**: Unimplemented

## Data Models

### Course
- `id` (string): Unique identifier
- `title` (string): Course title
- `description` (string): Course description
- `creator_id` (string): ID of the course creator
- `modules` (repeated Module): Course modules

### Module
- `id` (string): Unique identifier
- `title` (string): Module title
- `lessons` (repeated Lesson): Module lessons

### Lesson
- `id` (string): Unique identifier
- `title` (string): Lesson title
- `content` (string): Lesson content

### Enrollment
- `id` (string): Unique identifier
- `user_id` (string): ID of the enrolled user
- `course_id` (string): ID of the enrolled course
- `status` (EnrollmentStatus): Enrollment status
- `progress` (float): Completion progress (0.0 to 100.0)

### Credential
- `id` (string): Unique identifier
- `user_id` (string): ID of the user
- `course_id` (string): ID of the course
- `credential_type` (CredentialType): Type of credential
- `verification_code` (string): Verification code

### Tip
- `id` (string): Unique identifier
- `sender_id` (string): ID of the sender
- `recipient_id` (string): ID of the recipient
- `amount` (double): Tip amount
- `currency` (string): Currency code
- `course_id` (string, optional): Associated course ID

### User
- `user_id` (string): Unique identifier
- `username` (string): Username
- `email` (string): Email address

## Enums

### CredentialType
- `CERTIFICATE` = 0
- `BADGE` = 1
- `MICRO_DEGREE` = 2
- `DEGREE` = 3

### EnrollmentStatus
- `ENROLLED` = 0
- `IN_PROGRESS` = 1
- `COMPLETED` = 2
- `DROPPED` = 3

## Error Handling

The API uses gRPC status codes for error reporting:

- `OK` (0): Success
- `CANCELLED` (1): The operation was cancelled
- `UNKNOWN` (2): Unknown error
- `INVALID_ARGUMENT` (3): Client specified an invalid argument
- `DEADLINE_EXCEEDED` (4): Deadline expired
- `NOT_FOUND` (5): Some requested entity was not found
- `ALREADY_EXISTS` (6): Some entity that we attempted to create already exists
- `PERMISSION_DENIED` (7): The caller does not have permission to execute the specified operation
- `RESOURCE_EXHAUSTED` (8): Some resource has been exhausted
- `FAILED_PRECONDITION` (9): Operation was rejected because the system is not in a state required for the operation's execution
- `ABORTED` (10): The operation was aborted
- `OUT_OF_RANGE` (11): Operation was attempted past the valid range
- `UNIMPLEMENTED` (12): Operation is not implemented or not supported
- `INTERNAL` (13): Internal errors
- `UNAVAILABLE` (14): The service is currently unavailable
- `DATA_LOSS` (15): Unrecoverable data loss or corruption
- `UNAUTHENTICATED` (16): The request does not have valid authentication credentials

## Authentication

Most API endpoints require authentication using JWT tokens. The token should be provided in the `Authorization` header with the `Bearer` prefix.

Example:
```
Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
```

To obtain a token, use the `Authenticate` method of the `AuthService`.