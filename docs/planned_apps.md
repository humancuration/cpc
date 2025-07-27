Each module is to be a self-contained Rust crate that the main backend application depends on, and users can install/uninstall them from the options menu (in an app options submenu).

# Apps Checklist

Each app module is to be a self-contained Rust crate that the main backend application depends on. Feel free to add to this list of apps with others that you think would be a good idea! (Always check to see the progress of the app before making further plans for it)

## Entertainment

For information about our privacy policies and consent management, see our [Privacy Policy](privacy_policy.md).

- [x] **Music player**: COMPLETE implementation with all features including timestamped comments, visualizer presets, offline downloads, and social interactions. Fully validated as architectural blueprint. See [music_player_integration.md](docs/music_player_integration.md) for implementation details.

- [ ] **DAW**: Fully featured, allows for creating new tracks, mixing, mastering, and remixing. [Architecture Plan](apps/daw/ARCHITECTURE.md)

## Productivity & Work

These tools can help users manage their work, studies, and personal projects.

- [ ] **Docs**: A streamlined word processor for writing and editing documents. Key features would include basic formatting (bold, italics, lists), the ability to insert images, and options to export to PDF or .docx formats.

- [ ] **Sheets**: A simple spreadsheet application for creating budgets, tracking lists, or managing simple data. It should handle basic formulas, charts, and be compatible with formats like .xlsx.

- [ ] **Notes & Memos**: A flexible space for capturing thoughts. This could range from simple text notes to more complex notes that include checklists, images, and attachments. Features like tagging and notebooks for organization would be very useful.

- [ ] **Task Manager**: A dedicated to-do list function where users can create tasks, set deadlines and reminders, and categorize them into different projects or lists.

- [ ] **Calendar**: An integrated calendar that can sync with external services like Google Calendar and Outlook. It would manage events, set reminders, and offer different views (daily, weekly, monthly).

- [ ] **Website Builder**: This is for everyone from individuals to large businesses, fully featured, easy to use, feature rich. Also have a link-in-bio site builder. Both have great UI to let users build for both web and mobile, and both integrate well with things like payment processors (including our own) and sales platforms (or from their own shop, with a storebuilder wizard).

## Personal Finance

This is a powerful category that can provide immense value to users by helping them understand and manage their financial lives.

- [ ] **Budget Planner**: Tools for users to set monthly or weekly budgets for different spending categories (e.g., groceries, entertainment, transport). Visual progress bars can make this engaging.

- [ ] **Expense Tracker**: A simple way to log daily expenses. To make this even easier, you could include features like receipt scanning (using the phone's camera) to automatically pull details.

- [ ] **Subscription Manager**: A tool specifically for tracking recurring payments like streaming services, gym memberships, and software subscriptions. It can notify users before a payment is due.

- [ ] **Savings Goals**: A feature that allows users to set specific savings goals (e.g., "Vacation Fund," "New Laptop") and track their progress.

- [ ] **Personal BI Dashboard**: This is where your BI idea comes to life for the average person. It wouldn't be complex business intelligence, but rather a personal dashboard that provides simple, clear insights into their finances. This could include charts showing spending trends over time, a breakdown of income versus expenses, and a net worth tracker.

## Health & Wellness

Integrating wellness features can make the app a daily companion for users looking to improve their physical and mental well-being.

- [ ] **Habit Tracker**: A tool to help users build positive habits like drinking enough water, exercising, reading, or meditating.

- [ ] **Mood Journal**: A simple and private space for users to log their mood each day, perhaps with a short note. Over time, this can help identify patterns.

- [ ] **Meditation & Mindfulness**: A small library of guided meditations, breathing exercises, or calming ambient sounds to help users de-stress.

- [ ] **Meal & Hydration Planner**: A simple tool for planning weekly meals, creating shopping lists, and tracking daily water intake.

## Personal Organization & Utilities

These are the digital equivalents of a Swiss Army knife—small tools that are incredibly useful to have in one place.

- [ ] **File Manager**: A central hub to organize all the documents, spreadsheets, and notes created within the app. Offering integration with cloud services like Google Drive or Dropbox could be a major plus.

- [ ] **Password Manager**: A secure, encrypted vault for storing and managing passwords for different websites and services. This is a significant feature that requires a strong focus on security.

- [ ] **Unit Converter**: A handy utility for converting currencies, weights, measurements, and temperatures.

- [ ] **QR Code Scanner**: A built-in tool to quickly scan QR codes, which have become increasingly common.

## Small Businesses & Startups

The key here is simplicity, affordability, and integration. Small business owners often wear many hats, so tools that save time and consolidate tasks are a huge win.

- [ ] **Invoicing & Quoting**: A simple tool to create, send, and track professional invoices and quotes. Features could include payment integration (with Stripe, PayPal, etc.), automatic payment reminders, and status tracking (sent, viewed, paid).

- [ ] **Simple CRM (Customer Relationship Management)**: A lightweight contact manager to track customer interactions, notes, and sales pipelines. It wouldn't need the complexity of a Salesforce, but just enough to manage leads and nurture client relationships effectively.

- [ ] **Project Management Lite**: A visual project management tool, perhaps using Kanban boards (like Trello), to track tasks, assign them to team members, set deadlines, and attach relevant files.

- [ ] **Time Tracking**: A simple utility for owners and employees to log hours against specific projects or clients, which can then be used for billing or payroll.

- [ ] **Business Health Dashboard**: A simplified BI tool that connects to other modules (Invoicing, Expense Tracking) to provide at-a-glance metrics like revenue, outstanding invoices, profit and loss, and cash flow.

## Medium-Sized Businesses (SMBs)

As businesses grow, their needs shift towards more robust collaboration, process management, and data analysis.

- [ ] **Advanced CRM & Sales Pipeline**: An expanded CRM that can handle a larger sales team, with features for lead scoring, email marketing integration, detailed sales reporting, and customizable pipelines.

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
  - [ ] **Supply Chain Management (SCM)**: Tools to manage the entire flow of goods, from procurement of raw materials to delivery of the final product.
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

### Modular Approach
- Don't force every feature on every user
- Core set of essential apps (like Notes and Tasks) with optional modules
- Users can "install" or enable other modules based on their needs
- Keeps the interface clean for those who only want basic functionality

### Seamless Integration
- Different parts work together (e.g., link "Vacation" savings goal to "Plan Vacation" task list)
- Unified design language across all modules
- Focus on providing the most essential 80% of functionality in a clean and accessible way

### Example Architecture

Each module is to be a self-contained Rust crate that the main backend application depends on.

Anatomy of a Module Crate

Let's imagine you want to formalize your Invoicing feature into a proper module. You would create a new crate, for example, at apps/invoicing.

This crate would have a structure that mirrors your hexagonal architecture:
Generated code

      
apps/invoicing/
├── Cargo.toml
└── src/
    ├── lib.rs          # Main crate entry, exports the module
    ├── domain/         # Core business models (Invoice, Customer)
    │   └── models.rs
    ├── application/    # Services that orchestrate business logic
    │   └── service.rs
    ├── infrastructure/ # Implementations of ports (e.g., database)
    │   └── repository.rs
    └── web/            # Adapters for the web layer (Axum/GraphQL)
        ├── routes.rs
        ├── graphql.rs
        └── module.rs   # The "wiring" file for this module

        domain: Contains your pure business models (Invoice, InvoiceStatus, etc.). These should have no knowledge of databases or web frameworks. This is the "hexagon."

    application: Contains the InvoiceService, which uses traits (ports) to talk to the outside world (like an InvoiceRepository trait).

    infrastructure: Contains the PgInvoiceRepository that implements the InvoiceRepository trait using sqlx.

    web: This is the key adapter layer. It contains the Axum routes, GraphQL resolvers, and input/output types specific to this module.

2. The "Wiring" File (web/module.rs)

This is the most important part for making your system plug-and-play. Each module crate will expose a struct and a function to hand its components to the main backend.

// In apps/invoicing/src/web/module.rs

use axum::Router;
use crate::web::{graphql::{InvoicingQuery, InvoicingMutation, InvoicingSubscription}, routes::create_invoicing_router};
use crate::application::service::InvoiceService;
use crate::db::DbPool;

// This struct holds all the pieces the backend needs from this module
pub struct InvoicingModule {
    pub router: Router,
    pub query: InvoicingQuery,
    pub mutation: InvoicingMutation,
    pub subscription: InvoicingSubscription,
}

// This function initializes the module and its dependencies
pub fn initialize(db_pool: DbPool) -> InvoicingModule {
    let invoice_service = InvoiceService::new(db_pool.clone());
    
    InvoicingModule {
        router: create_invoicing_router(invoice_service.clone()),
        query: InvoicingQuery::new(invoice_service.clone()),
        mutation: InvoicingMutation::new(invoice_service.clone()),
        subscription: InvoicingSubscription,
    }
}

3. Wiring it All Together in backend/src/main.rs

Your main.rs will become much simpler. Instead of initializing every service itself, it will just call the initialize function from each module it wants to include.

Before (Simplified from your current code):
      
// In backend/src/main.rs (current)

async fn main() {
    // ... setup ...
    let db = init_db().await.unwrap();

    // Initialize individual services
    let social_service = Arc::new(SocialService::new(...));
    let forum_service = Arc::new(ForumService::new(...));
    // ... and so on for every service

    // Build GraphQL schema by adding each service
    let schema = Schema::build(...)
        .data(social_service.clone())
        .data(forum_service.clone())
        // ... and so on
        .finish();

    // Build Axum router by merging each route
    let app = Router::new()
        .nest("/api/social", routes::social::router().with_state(...))
        .nest("/api/forum", routes::forum::router().with_state(...))
        // ... and so on
        .with_state(schema);
    
    // ... run server ...
}

After (Using the modular approach):
      
// In backend/src/main.rs (proposed)
use cpc_invoicing::web::module as invoicing; // Import the module
use cpc_social::web::module as social;     // Example for another module

async fn main() {
    // ... setup ...
    let db = init_db().await.unwrap();

    // Initialize modules
    let invoicing_module = invoicing::initialize(db.clone());
    let social_module = social::initialize(db.clone());
    // ... initialize other modules as needed

    // Build GraphQL schema by merging module components
    let schema = Schema::build(
        RootQuery(invoicing_module.query, social_module.query),
        RootMutation(invoicing_module.mutation, social_module.mutation),
        RootSubscription(invoicing_module.subscription, social_module.subscription)
    )
    .data(db.clone()) // Add shared state like the DB pool
    .finish();

    // Build Axum router by merging module routers
    let app = Router::new()
        .nest("/api/invoicing", invoicing_module.router)
        .nest("/api/social", social_module.router)
        // ... and so on
        .with_state(schema);
        
    // ... run server ...
}

Adding a new "app" to your multi-use app would then be as simple as:

    Adding the new module crate to your workspace.

    Adding it as a dependency in backend/Cargo.toml.

    Calling its initialize() function in main.rs and adding its components to the Router and Schema.

Step-by-Step Refactoring Plan

    Choose One Feature to Start: Let's use Invoicing as the example.

    Create the New Crate: In your workspace, create the apps/invoicing crate.

    Update Cargo.toml:

        In apps/invoicing/Cargo.toml, add dependencies like axum, async-graphql, sqlx, cpc-core, etc.

        In apps/backend/Cargo.toml, add the new invoicing crate as a dependency: cpc-invoicing = { path = "../invoicing" }.

    Move the Code:

        Move the contents of apps/backend/src/invoicing/ to apps/invoicing/src/.

        Organize the files into the domain, application, infrastructure, and web directories as described above.

        Move the relevant routes from apps/backend/src/routes/ and GraphQL definitions from apps/backend/src/graphql/ into the new crate's web directory.

    Create the module.rs: Implement the initialize() function in apps/invoicing/src/web/module.rs to wire up and return the module's router and GraphQL components.

    Update backend/main.rs: Modify your main.rs to import and use the invoicing module as shown in the "After" example. You will remove the direct initialization of InvoiceService and its routes from main.rs.

    Repeat: Repeat this process for your other vertical slices (social, forum, governance, etc.).

    This approach provides very clear boundaries, allows modules to be developed and tested more independently, and makes your main backend crate a lightweight "aggregator" of modules, which is exactly what you want for a plug-and-play system.