# Learning Platform Architecture

## Overview
Educational platform extending our skill development system with course management, enrollment tracking, credential issuance, and tipping functionality.

## Domain Model
```mermaid
classDiagram
    class Course {
        +id: Uuid
        +title: String
        +description: String
        +creator_id: Uuid
        +modules: Vec~Module~
    }

    class Module {
        +id: Uuid
        +title: String
        +lessons: Vec~Lesson~
        +quiz: Quiz
    }

    class Lesson {
        +id: Uuid
        +title: String
        +content: String
        +media_url: String
    }

    class Enrollment {
        +id: Uuid
        +user_id: Uuid
        +course_id: Uuid
        +progress: f32
        +status: EnrollmentStatus
    }

    class AcademicCredential {
        +id: Uuid
        +user_id: Uuid
        +course_id: Uuid
        +credential_type: CredentialType
        +issued_at: DateTime
    }

    class Tip {
        +id: Uuid
        +from_user_id: Uuid
        +to_user_id: Uuid
        +amount: f64
        +currency: String
        +course_id: Option~Uuid~
    }

    Course "1" *-- "*" Module
    Module "1" *-- "*" Lesson
    Module "1" *-- "1" Quiz
    User "1" *-- "*" Enrollment
    Course "1" *-- "*" Enrollment
    User "1" *-- "*" AcademicCredential
    Course "1" *-- "*" AcademicCredential
    User "1" *-- "*" Tip
```

## Service Interfaces
### gRPC Service Methods
- `CreateCourse(CourseRequest) → CourseResponse`
- `EnrollUser(EnrollmentRequest) → EnrollmentResponse`
- `UpdateProgress(ProgressUpdateRequest) → ProgressResponse`
- `IssueCredential(CredentialRequest) → CredentialResponse`
- `TipEducator(TipRequest) → TipResponse`

## Database Schema
```mermaid
erDiagram
    COURSE ||--o{ MODULE : contains
    MODULE ||--o{ LESSON : contains
    MODULE ||--o{ QUIZ : contains
    USER ||--o{ ENROLLMENT : has
    COURSE ||--o{ ENROLLMENT : has
    USER ||--o{ ACADEMIC_CREDENTIAL : holds
    USER ||--o{ TIP_RECEIVED : receives
    USER ||--o{ TIP_SENT : gives

    COURSE {
        uuid id PK
        string title
        text description
        uuid creator_id FK
    }
    
    MODULE {
        uuid id PK
        uuid course_id FK
        string title
        int order
    }
    
    ENROLLMENT {
        uuid id PK
        uuid user_id FK
        uuid course_id FK
        float progress
        string status
    }
    
    ACADEMIC_CREDENTIAL {
        uuid id PK
        uuid user_id FK
        uuid course_id FK
        string credential_type
        timestamp issued_at
    }
    
    TIP {
        uuid id PK
        uuid from_user_id FK
        uuid to_user_id FK
        uuid course_id FK "Nullable"
        decimal amount
        string currency
    }
```

## Component Interaction
```mermaid
sequenceDiagram
    Frontend->>+LearningService: CreateCourse(course_data)
    LearningService->>+DB: Save course
    DB-->>-LearningService: Course ID
    LearningService-->>-Frontend: Course created
    
    Frontend->>+LearningService: EnrollUser(user_id, course_id)
    LearningService->>+DB: Create enrollment
    DB-->>-LearningService: Enrollment record
    LearningService-->>-Frontend: Enrollment confirmed
    
    Frontend->>+LearningService: UpdateProgress(update)
    LearningService->>+DB: Update enrollment
    DB-->>-LearningService: Updated record
    LearningService->>+SkillService: UpdateSkillProgress()
    SkillService-->>-LearningService: Updated skills
    LearningService-->>-Frontend: Progress updated
    
    Frontend->>+LearningService: IssueCredential(request)
    LearningService->>+DB: Save credential
    DB-->>-LearningService: Credential ID
    LearningService-->>-Frontend: Credential issued
    
    Frontend->>+PaymentService: TipEducator(tip_data)
    PaymentService-->>-Frontend: Tip processed
```

## Reusable Components
1. **From skill_tracking**:
   - `LearningPathCreator` → Adapt for course creation
   - `SkillProgressTracker` → Reuse for course progress tracking
   - `CertificationDisplay` → Extend for academic credentials

2. **Shared Infrastructure**:
   - gRPC server setup from `grpc_server.rs`
   - Database connection pooling
   - Error handling patterns
   - Type conversion utilities

## Implementation Plan
1. **Core Domain Setup**:
   - Course/Module/Lesson entities
   - Enrollment tracking system
   - Credential issuance workflow

2. **Service Layer**:
   - gRPC service implementation
   - Database repositories
   - Integration with skill service

3. **UI Components**:
   - Course management interface
   - Enrollment dashboard
   - Progress tracking view
   - Credential display

4. **Tipping Integration**:
   - Educator tipping UI
   - Payment processing
   - Tip history tracking