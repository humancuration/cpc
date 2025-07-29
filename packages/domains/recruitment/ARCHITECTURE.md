# Recruitment System Module Architecture

This document outlines the architecture for the recruitment module, implementing the LinkedIn competitor functionality as specified in `docs/planned_apps.md` line 170. The design follows our hexagonal architecture principles and screaming architecture pattern as documented in planned_apps.md (lines 154-182).

## 1. Module Overview and Responsibilities

The Recruitment System module provides comprehensive job market functionality with focus on:

- **Job Market Creation & Management**: Posting jobs, managing job listings, and tracking applicants
- **Career Development**: Resume posting, job searching, and career trajectory planning
- **Recruitment Workflow**: Applicant tracking, interview scheduling, and candidate evaluation
- **Network Building**: Professional networking with opt-in data sharing across the federation

This module adheres to our cooperative principles by:
- Prioritizing user control over profile data and sharing preferences
- Supporting privacy through granular consent controls while enabling professional networking
- Enabling opt-in data sharing to strengthen the job market ecosystem
- Focusing on human dignity and opportunity in all features

## 2. Directory Structure Following Vertical Slice Pattern

```
packages/cpc-core/recruitment/
├── Cargo.toml
├── migrations/
│   ├── 20250801000000_create_recruitment_tables.sql
│   └── 20250801000001_add_candidate_evaluation_tables.sql
└── src/
    ├── lib.rs                  # Main crate entry, exports the module
    ├── domain/                 # Core business models
    │   ├── models.rs           # Primary entities (Job, Candidate, Employer, etc.)
    │   ├── value_objects.rs    # Domain-specific types
    │   └── errors.rs           # Custom error types
    ├── application/            # Business logic services
    │   ├── job_service.rs      # Job posting and management operations
    │   ├── candidate_service.rs # Candidate profile management
    │   ├── application_service.rs # Job application workflow
    │   ├── interview_service.rs # Scheduling and evaluation workflows
    │   └── alert_service.rs    # Job alert and notification system
    ├── infrastructure/         # External implementations
    │   ├── repository.rs       # Database access layer
    │   ├── graphql.rs          # Public API implementation
    │   ├── grpc.rs             # Internal service communication
    │   ├── calendar_integration.rs # Calendar system integration
    │   ├── notification.rs     # Notification system integration
    │   ├── resume_parser.rs    # Resume parsing and processing
    │   └── matching_engine.rs  # Job/candidate matching algorithms
    └── presentation/           # UI components (Yew)
        ├── mod.rs
        ├── job_listings.rs     # Job board component
        ├── candidate_profile.rs # Candidate profile editor
        ├── employer_dashboard.rs # Employer management dashboard
        ├── application_tracker.rs # Application tracking interface
        └── interview_scheduler.rs # Interview scheduling component
```

## 3. Component Diagram Showing Domain Layers

```
┌───────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┐
│                                                       Presentation                                                  │
│ ┌───────────────────┐       ┌─────────────────────┐       ┌───────────────────┐       ┌─────────────────────────────┐ │
│ │    Yew Components │──────▶│   GraphQL Queries   │──────▶│     Job Service   │──────▶│        Domain Models        │ │
│ └───────────────────┘       └─────────────────────┘       └───────────────────┘       └─────────────────────────────┘ │
│                                                                                                                       │
│ ┌───────────────────┐       ┌─────────────────────┐       ┌───────────────────┐                                       │
│ │ Candidate Profile │──────▶│ Candidate Service   │──────▶│ Application Service├───────────────────────────────────────┤ │
│ └───────────────────┘       └─────────────────────┘       └───────────────────┘                                       │
│                                                                                                                       │
│ ┌───────────────────┐       ┌─────────────────────┐       ┌───────────────────┐                                       │
│ │  Interview UI     │──────▶│ Interview Service   │──────▶│     Alert Service │                                       │
│ └───────────────────┘       └─────────────────────┘       └───────────────────┘                                       │
│                                                                                                                       │
└───────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┘
                                      ▲                               ▲                               ▲
                                      │                               │                               │
                                      ▼                               ▼                               ▼
┌───────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┐
│                                                     Infrastructure                                                    │
│ ┌───────────────────┐       ┌─────────────────────┐       ┌───────────────────┐       ┌─────────────────────────────┐ │
│ │     SQL Database  │◀─────▶│   Repositories     ├──────▶│ Calendar Integration│◀────▶│  Google/Outlook Calendars   │ │
│ └───────────────────┘       └─────────────────────┘       └───────────────────┘       └─────────────────────────────┘ │
│                                                                                                                       │
│ ┌───────────────────┐       ┌─────────────────────┐       ┌───────────────────┐       ┌─────────────────────────────┐ │
│ │  Notification     │◀─────▶│ Notification System ├──────▶│  Email/SMS Gateway│◀─────▶│ Twilio/SendGrid/ETC         │ │
│ │   Services        │       │                     │       │                   │       │                             │ │
│ └───────────────────┘       └─────────────────────┘       └───────────────────┘       └─────────────────────────────┘ │
│                                                                                                                       │
│ ┌───────────────────┐       ┌─────────────────────┐       ┌───────────────────┐       ┌─────────────────────────────┐ │
│ │    Resume Store   │◀─────▶│ Resume Parser       ├──────▶│   PDF Processing  │◀─────▶│    pdf-rs library           │ │
│ └───────────────────┘       └─────────────────────┘       └───────────────────┘       └─────────────────────────────┘ │
│                                                                                                                       │
│ ┌───────────────────┐       ┌─────────────────────┐                                                                 │ │
│ │   Matching Engine │◀─────▶│   Data Sharing      │                                                                 │ │
│ └───────────────────┘       │ Federation Layer    │                                                                 │ │
│                             └─────────────────────┘                                                                 │ │
│                                                                                                                       │
└───────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┘
```

## 4. Data Flow for Key Operations

### Job Posting Workflow
1. Employer creates job posting via GraphQL mutation
2. JobService validates job details against employer profile permissions
3. Domain models create job entity with required fields (title, description, location, etc.)
4. Repository stores the job in SQL database
5. MatchingEngine indexes the job for search and candidate matching
6. AlertService checks for candidates with active job alerts matching this job
7. System returns Job ID to presentation layer for employer dashboard

### Application Submission Workflow
1. Candidate submits application via UI with resume and cover letter
2. ApplicationService validates candidate profile completeness
3. ResumeParser processes the uploaded resume (PDF/DOCX) into structured data
4. Domain models create application entity linking candidate and job
5. Repository stores the application and parsed resume data
6. NotificationService sends confirmation to candidate
7. AlertService notifies employer of new application
8. Application appears in employer's dashboard for review

### Interview Scheduling Workflow
1. Employer selects candidate for interview via UI
2. InterviewService creates interview request with available time slots
3. CalendarIntegration checks employer's calendar availability
4. NotificationService sends interview request to candidate
5. Candidate selects preferred time slot via UI
6. CalendarIntegration creates calendar events for both parties
7. System sends calendar invites through integrated calendar services
8. Interview appears in both parties' calendars and dashboard

### Candidate Evaluation Workflow
1. After interview, employer accesses evaluation form via UI
2. InterviewService creates evaluation session
3. Domain models validate evaluator permissions
4. Employer submits ratings and feedback through UI
5. ApplicationService updates application status and stores evaluation
6. Repository persists evaluation data with timestamp and evaluator ID
7. AlertService may trigger next steps based on evaluation outcome
8. Candidate receives appropriate notification based on configured preferences

## 5. Integration Points with Other Modules

| Integration Point | Module | Purpose |
|-------------------|--------|---------|
| Authentication | `cpc-core/auth` | Verify user identities and permissions |
| User Profile | `cpc-core/user` | Retrieve and link to user profiles |
| Calendar | `cpc-core/calendar` | Schedule and manage interviews |
| Notification | `cpc-core/notification` | Send job alerts and application notifications |
| Document Editor | `cpc-core/document_editor` | Process resumes in PDF/DOCX formats |
| Payment | `cpc-core/payment` | Handle premium job posting features |
| Cooperative | `cpc-core/cooperative` | Handle cooperative-specific job opportunities |

The module will provide the following integration points for other modules:

- **Professional Network API**: Allow other modules to access opt-in professional profiles
- **Job Opportunity Feed**: Provide filtered job listings to relevant contexts
- **Candidate Pipeline API**: Enable tracking of recruitment progress from external systems

## 6. Technology Choices

### Core Components
- **Domain Logic**: Pure Rust with strong type system
- **Database**: PostgreSQL via SQLx (already in workspace dependencies)
- **Tracing**: tracing crate for observability
- **Resume Processing**: pdf-rs (already in tech stack)

### API Implementation
- **GraphQL API**: Juniper (already in tech stack)
  - Public API for job search, profile viewing (with appropriate privacy controls)
  - Standardized query structure for federation compatibility
  - Pagination and filtering built into core schema

- **gRPC Interface**: tonic (already in tech stack)
  - Internal service communication for matching engine
  - Integration with calendar and notification systems
  - Federation-wide data sharing capabilities

### Matching Algorithms
- **Basic Matching**: Keyword-based matching using PostgreSQL full-text search
- **Advanced Matching**: Vector embeddings using Rust-based cosine similarity
  - For future implementation when AI capabilities are added
  - MIT-licensed vector library to be added

### UI Components
- **Frontend Framework**: Yew (already in tech stack)
  - Consistent experience across desktop and mobile
  - Responsive design patterns for various job search scenarios
  - Progressive enhancement for accessibility

## 7. Security Considerations for Recruitment Data

### Data Protection
- **End-to-End Encryption**: Sensitive application data encrypted at rest using AES-256
- **Granular Permissions**: Different access levels for candidates, employers, and admins
- **Audit Logs**: Comprehensive logging of all profile and application accesses
- **Data Minimization**: Only necessary personal data collected with explicit consent

### Privacy by Design
- **Opt-in Data Sharing**: Users control what information is shared across the federation
- **Anonymized Analytics**: Usage data aggregated and anonymized for system improvement
- **Right to be Forgotten**: Complete data deletion process for accounts
- **Consent Management**: Clear controls for sharing professional information

### Special Considerations
- **Resume Protection**: Resumes only accessible to employers who received applications
- **Anti-Discrimination Features**: System flags potentially biased language in job descriptions
- **Salary Transparency**: Encourages but doesn't require salary ranges in job postings
- **Accessibility**: Full WCAG 2.1 AA compliance for all interfaces

## 8. Future Expansion Points

### Immediate Roadmap
- **Skill Endorsements**: Peer validation of professional skills
- **Salary Benchmarking**: Anonymous salary data sharing within the federation
- **Video Interviewing**: Integrated video capabilities with privacy controls
- **Application Analytics**: Insights for employers on application sources and conversion

### Medium-Term
- **AI-Powered Matching**: Enhanced matching algorithms with user feedback
- **Career Path Mapping**: Visualizing potential career trajectories
- **Diversity Analytics**: Anonymous metrics to help improve hiring practices
- **Automated Interview Prep**: Resources for candidates based on job requirements

### Long-Term
- **Skills-Based Hiring**: Moving beyond traditional resume screening
- **Professional Development**: Integrated learning resources tied to job requirements
- **Cooperative Job Market**: Special features for worker-owned businesses
- **Decentralized Reputation**: Portable professional reputation across the federation

## 9. Compliance Verification

✅ Hexagonal Architecture with clear domain/application/infrastructure separation  
✅ Screaming Architecture reflecting recruitment and job market capabilities  
✅ Vertical slice implementation within `packages/cpc-core/`  
✅ Permissively licensed dependencies (MIT/Apache 2.0)  
✅ Integration with existing calendar and notification infrastructure  
✅ Support for opt-in data sharing across federation  
✅ Mobile-first responsive UI design  
✅ Comprehensive privacy and security considerations  
✅ GraphQL API for public access as required  
✅ gRPC implementation for internal service communication  
✅ PostgreSQL as primary data storage system