Each module is to be a self-contained Rust crate that the main backend application depends on, and users can install/uninstall them from the options menu (in an app options submenu).

# Apps Checklist

Each app module is to be a self-contained Rust crate that the main backend application depends on. Feel free to add to this list of apps with others that you think would be a good idea! (Always check to see the progress of the app before making further plans for it)

We should implement oauth2 with support for TikTok, Facebook, YouTube, WhatsApp, Instagram, Threads, WeChat, Messenger, Snapchat, Discord, X, Twitch, Gmail, etc.

## Entertainment

For information about our privacy policies and consent management, see our [Privacy Policy](privacy_policy.md).

- [x] **Music player**: COMPLETE implementation with all features including timestamped comments, visualizer presets, offline downloads, and social interactions. Fully validated as architectural blueprint. See [music_player_integration.md](docs/music_player_integration.md) for implementation details.

- [/] **DAW**: Fully featured, allows for creating new tracks, mixing, mastering, and remixing. [Architecture Plan](apps/daw/ARCHITECTURE.md)

## Productivity & Work

These tools can help users manage their work, studies, and personal projects.

- [ ] **Docs**: A streamlined word processor for writing and editing documents. Key features would include basic formatting (bold, italics, lists), the ability to insert images, and options to export to PDF or .docx formats.

- [ ] **Sheets**: A simple spreadsheet application for creating budgets, tracking lists, or managing simple data. It should handle basic formulas, charts, and be compatible with formats like .xlsx.

- [ ] **Notes & Memos**: A flexible space for capturing thoughts. This could range from simple text notes to more complex notes that include checklists, images, and attachments. Features like tagging and notebooks for organization would be very useful.

- [ ] **Task Manager**: A dedicated to-do list function where users can create tasks, set deadlines and reminders, and categorize them into different projects or lists.

- [/] **Calendar**: An integrated calendar that can sync with external services like Google Calendar and Outlook. It would manage events, set reminders, and offer different views (daily, weekly, monthly).

- [/] **Website Builder**: This is for everyone from individuals to large businesses, fully featured, easy to use, feature rich. Also have a link-in-bio site builder. Both have great UI to let users build for both web and mobile, and both integrate well with things like payment processors (including our own) and sales platforms (or from their own shop, with a storebuilder wizard).

## Personal Finance

This is a powerful category that can provide immense value to users by helping them understand and manage their financial lives.

- [ ] **Budget Planner**: Tools for users to set monthly or weekly budgets for different spending categories (e.g., groceries, entertainment, transport). Visual progress bars can make this engaging.

- [ ] **Expense Tracker**: A simple way to log daily expenses. To make this even easier, you could include features like receipt scanning (using the phone's camera) to automatically pull details.

- [ ] **Subscription Manager**: A tool specifically for tracking recurring payments like streaming services, gym memberships, and software subscriptions. It can notify users before a payment is due.

- [ ] **Savings Goals**: A feature that allows users to set specific savings goals (e.g., "Vacation Fund," "New Laptop") and track their progress.

- [ ] **Personal BI Dashboard**: This is where your BI idea comes to life for the average person. It wouldn't be complex business intelligence, but rather a personal dashboard that provides simple, clear insights into their finances. This could include charts showing spending trends over time, a breakdown of income versus expenses, and a net worth tracker.

## Health & Wellness

Integrating wellness features can make the app a daily companion for users looking to improve their physical and mental well-being.

- [ ] **Habit Tracker**: Part of the health domain module (`packages/cpc-core/health/`) for tracking positive habits like drinking enough water, exercising, or meditating. See [Health Module Documentation](docs/architecture/health.md) for implementation details.

- [ ] **Mood Journal**: Part of the health domain module (`packages/cpc-core/health/`) for logging mood with privacy controls. See [Health Module Documentation](docs/architecture/health.md) for implementation details.

- [ ] **Meditation & Mindfulness**: Part of the health domain module (`packages/cpc-core/health/`) with guided meditations and breathing exercises. See [Health Module Documentation](docs/architecture/health.md) for implementation details.

- [ ] **Meal & Hydration Planner**: Part of the health domain module (`packages/cpc-core/health/`) for planning meals and tracking water intake. See [Health Module Documentation](docs/architecture/health.md) for implementation details.

- [x] **HIPAA-compliant audit trails**: COMPLETE implementation with comprehensive logging of all Protected Health Information (PHI) access events. Technical highlights include:
  - Research access patterns with NULL user_id handling for anonymization
  - Purpose code tracking (UserView, ProviderAccess, Research, DataSync, Admin)
  - Wearable sync logging with automatic data minimization
  - Fail-safe pattern implementation ensuring system continuity if logging fails
  - AES-256 encryption at rest for all audit logs
  - 1-year active storage with 5-year archival (6-year total retention)
  Compliance roadmap: Q3: Automated audit log certification process
  See [Health Module Documentation](docs/architecture/health.md) for implementation details.

## Personal Organization & Utilities

These are the digital equivalents of a Swiss Army knife—small tools that are incredibly useful to have in one place.

- [ ] **File Manager**: A central hub to organize all the documents, spreadsheets, and notes created within the app. Offering integration with cloud services like Google Drive or Dropbox could be a major plus.

- [ ] **Password Manager**: A secure, encrypted vault for storing and managing passwords for different websites and services. This is a significant feature that requires a strong focus on security.

- [ ] **Unit Converter**: A handy utility for converting currencies, weights, measurements, and temperatures.

- [ ] **QR Code Scanner**: A built-in tool to quickly scan QR codes, which have become increasingly common.

## Small Businesses & Startups

The key here is simplicity, affordability, and integration. Small business owners often wear many hats, so tools that save time and consolidate tasks are a huge win.

- [/] **Invoicing & Quoting**: A simple tool to create, send, and track professional invoices and quotes. Features could include payment integration (with Stripe, PayPal, etc.), automatic payment reminders, and status tracking (sent, viewed, paid).

- [x] **Simple CRM (Customer Relationship Management)**: A lightweight contact manager to track customer interactions, notes, and sales pipelines. It wouldn't need the complexity of a Salesforce, but just enough to manage leads and nurture client relationships effectively. Features include:
  - Platform-native contact management with consent-based data sharing
  - Sales pipeline visualization with Bevy
  - Consent settings visualization with Yew
  - Interaction tracking and deal management

- [ ] **Project Management Lite**: A visual project management tool, perhaps using Kanban boards (like Trello), to track tasks, assign them to team members, set deadlines, and attach relevant files.

- [ ] **Time Tracking**: A simple utility for owners and employees to log hours against specific projects or clients, which can then be used for billing or payroll.

- [ ] **Business Health Dashboard**: A simplified BI tool that connects to other modules (Invoicing, Expense Tracking) to provide at-a-glance metrics like revenue, outstanding invoices, profit and loss, and cash flow.

## Medium-Sized Businesses (SMBs)

As businesses grow, their needs shift towards more robust collaboration, process management, and data analysis.

- [/] **Advanced CRM & Sales Pipeline**: An expanded CRM that can handle a larger sales team, with features for lead scoring, email marketing integration, detailed sales reporting, and customizable pipelines.

- [ ] **HR & Team Management Suite**: This module would be a central place for human resources management.
  - [ ] **Employee Directory**: A central database of employee information.
  - [ ] **Leave/Time-Off Management**: A system for employees to request time off and for managers to approve it.
  - [ ] **Simple Payroll**: Integration or a built-in tool to run payroll, although this can be complex due to regulations. Partnering with a payroll service might be a better option.

- [ ] **Inventory Management**: For businesses that sell physical products, a tool to track stock levels, manage suppliers, process purchase orders, and get alerts when inventory is low.

- [ ] **Comprehensive Financial Suite**: Moving beyond simple invoicing to a more complete accounting module that includes a general ledger, accounts payable/receivable, bank reconciliation, and financial reporting (Balance Sheet, Income Statement).

- [ ] **Internal Knowledge Base**: A dedicated space for creating and organizing internal documentation, company policies, training materials, and how-to guides for employees.

## Large Businesses & Enterprise

Large enterprises require tools that focus on scalability, security, compliance, and deep data integration across departments.

- [ ] **Enterprise Resource Planning (ERP) Modules**: This is the core of enterprise software. Your multi-use app could offer modular versions of ERP components.
  - [/] **Supply Chain Management (SCM)**: Tools to manage the entire flow of goods, from procurement of raw materials to delivery of the final product.
  - [ ] **Advanced Financials & Compliance**: Sophisticated accounting tools that can handle multiple currencies, complex tax regulations, and generate audit-ready reports.

- [ ] **Business Intelligence (BI) & Analytics**: Powerful, customizable dashboards that can pull data from all other modules to provide deep insights into business operations, market trends, and performance metrics.

- [ ] **Advanced HR Suite (HRIS)**: A full-featured Human Resources Information System that includes performance management, employee onboarding and offboarding workflows, benefits administration, and compliance management.

- [ ] **Compliance & Governance Tools**: Features to help manage regulatory compliance (like GDPR or HIPAA, depending on the industry), track policy acknowledgments, and manage internal audits.

- [ ] **API & Integration Hub**: For large businesses, the ability to connect your app to their existing software ecosystem (like Oracle, SAP, or custom internal tools) is critical. An easy-to-use API and pre-built connectors would be a major selling point.

## Cooperatives

Cooperatives have unique needs centered around member engagement, democratic governance, and surplus distribution.

- [ ] **Member Management & Directory**: More than just a customer list, this would be a central registry of members with details on their equity stake, voting rights, and engagement history.

- [ ] **Governance & Voting Module**: A secure tool to facilitate democratic processes. This could include features for proposing motions, holding discussions, and conducting secure, auditable votes.

- [ ] **Patronage & Surplus Distribution Calculator**: A specialized financial tool to track member purchases or contributions (patronage) and calculate how to distribute the cooperative's surplus or profits back to the members based on pre-defined rules.

- [ ] **Community & Communications Hub**: A private social network or forum for members to communicate with each other and with the cooperative's management, post updates, and organize community events.

- [ ] **Resource & Document Library**: A central place to store and share important documents with members, such as the cooperative's bylaws, meeting minutes, annual reports, and educational materials.

## Scientific

- [ ] ** Scientific Journal like Jupyter Notebook**:





## Architecture Notes

### Critical Correction: Domain Module Placement

⚠️ **Domain modules (Personal Finance, Health, etc.) MUST be implemented as vertical slices within `packages/cpc-core/`, NOT in `apps/` directory.** Standalone application folders for domain modules violate our screaming architecture principles.

- Top-level runnable applications (backend, desktop client, node workers) belong in `apps/`
- Domain-specific functionality (finance, health, productivity tools) belongs in `packages/cpc-core/`
- Migration files must reside in `packages/cpc-core/migrations/`, never in app-specific migration directories

### Modular Approach
- Don't force every feature on every user
- Core set of essential apps (like Notes and Tasks) with optional modules
- Users can "install" or enable other modules based on their needs
- Keeps the interface clean for those who only want basic functionality

### Seamless Integration
- Different parts work together (e.g., link "Vacation" savings goal to "Plan Vacation" task list)
- Unified design language across all modules
- Focus on providing the most essential 80% of functionality in a clean and accessible way

## Revised Architecture Principles

### 1. Core Design Philosophy

* **True Modularity**: Each app module must function as a standalone, self-contained unit that can be developed, tested, and deployed independently
* **User Empowerment**: Users should be able to enable/disable modules at runtime without restarting the application
* **Cooperative Values**: Architecture must support transparency, user control, and community participation in feature development
* **Domain Module Placement**: Domain-specific modules (such as finance, health, etc.) must be implemented as vertical slices within `packages/cpc-core/`, NOT in `apps/`. Standalone application folders for domain modules are strictly forbidden. Only top-level runnable applications (backend, desktop client, node workers) belong in `apps/`.
* **Critical Distinction**: `apps/` contains executable applications, while `packages/cpc-core/` contains business domain capabilities implemented as vertical slices. This separation ensures screaming architecture where the structure reflects business capabilities, not technical concerns.

### 2. Directory Structure & Crate Organization

The current structure largely aligns with our vision, but needs some refinements:

#### For Executable Applications (apps/ directory)
```
apps/
├── [module-name]/
│   ├── Cargo.toml
│   ├── migrations/             # Database migrations (for apps with DB)
│   └── src/
│       ├── lib.rs
│       ├── domain/             # Pure business logic, no external dependencies
│       ├── application/        # Use cases and service orchestrations
│       ├── infrastructure/     # Concrete implementations (DB, network, etc.)
│       └── web/                # API adapters (GraphQL, REST)
│           ├── routes.rs
│           ├── graphql.rs      # Query, Mutation, Subscription types
│           └── module.rs       # Module initialization & wiring
```


#### Non-executable Modules go in

packages
├── cpc-core/
│   └── [module-name]]
│       ├──