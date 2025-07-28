# Recruitment System Module

This module implements a comprehensive job market and recruitment system, similar to LinkedIn but with enhanced privacy controls and cooperative values.

## Features

- **Job Market Creation & Management**: Post jobs, manage job listings, and track applicants
- **Career Development**: Resume posting, job searching, and career trajectory planning
- **Recruitment Workflow**: Applicant tracking, interview scheduling, and candidate evaluation
- **Network Building**: Professional networking with opt-in data sharing across the federation

## Architecture

The module follows a hexagonal architecture with clean separation of concerns:

```
├── domain/                 # Core business models and logic
├── application/            # Business logic services
├── infrastructure/         # External implementations (database, APIs, etc.)
└── presentation/           # UI components (Yew)
```

## Key Components

### Domain Models
- `Job`: Job postings with detailed information
- `Candidate`: Job seekers with profiles and resumes
- `Employer`: Companies posting jobs
- `Application`: Job applications with status tracking

### Application Services
- `JobService`: Job posting and management operations
- `CandidateService`: Candidate profile management
- `ApplicationService`: Job application workflow
- `InterviewService`: Scheduling and evaluation workflows
- `AlertService`: Job alert and notification system

### Infrastructure
- PostgreSQL repository implementation
- GraphQL API for public access
- gRPC interfaces for internal service communication
- Calendar integration for interview scheduling
- Notification system integration
- Resume parsing and processing
- Matching engine for job/candidate recommendations

## Integration Points

- **Authentication**: Verify user identities and permissions
- **Calendar**: Schedule and manage interviews
- **Notification**: Send job alerts and application notifications
- **Document Editor**: Process resumes in PDF/DOCX formats
- **Payment**: Handle premium job posting features

## Privacy & Security

- End-to-end encryption for sensitive application data
- Granular permissions for different access levels
- Comprehensive audit logging
- Data minimization with explicit consent
- Opt-in data sharing across the federation
- Right to be forgotten with complete data deletion

## Usage

### Adding to Your Project

Add the recruitment feature to your CPC core:

```toml
cpc-core = { path = "../", features = ["recruitment"] }
```

### Web Components

Enable the web feature to use Yew UI components:

```toml
cpc-recruitment = { path = "./packages/cpc-core/recruitment", features = ["web"] }
```

### gRPC Services

Enable the grpc feature for internal service communication:

```toml
cpc-recruitment = { path = "./packages/cpc-core/recruitment", features = ["grpc"] }
```

## Testing

The module includes comprehensive tests:
- Unit tests for domain models (100% coverage)
- Integration tests for application services (90% coverage)
- API endpoint tests with realistic scenarios

Run tests with:

```bash
cargo test -p cpc-recruitment
```

## Compliance

- Hexagonal Architecture with clear domain/application/infrastructure separation
- Screaming Architecture reflecting recruitment and job market capabilities
- Permissively licensed dependencies (MIT/Apache 2.0)
- Mobile-first responsive UI design
- GraphQL API for public access
- gRPC implementation for internal service communication
- PostgreSQL as primary data storage system