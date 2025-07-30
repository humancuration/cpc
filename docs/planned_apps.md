Each module is to be a self-contained Rust crate that the main backend application depends on, and users can install/uninstall them from the options menu (in an app options submenu).

# Apps Checklist

Each app module is to be a self-contained Rust crate that the main backend application depends on. Feel free to add to this list of apps with others that you think would be a good idea! (Always check to see the progress of the app before making further plans for it)

We should implement oauth2 with support for TikTok, Facebook, YouTube, WhatsApp, Instagram, Threads, WeChat, Messenger, Snapchat, Discord, X, Twitch, Gmail, etc.

Leave blank if not started, mark [/] if partially implemented, [x] if implemented more fully, and [z] if it is difficult to find ways for improvement. Aim to start all apps to at least an x, then when that is done, we do a second pass and aim to upgrade everything to the Z tier of features.

## Entertainment


- [x] **Music player**: COMPLETE implementation with all features including timestamped comments, visualizer presets, offline downloads, and social interactions. Fully validated as architectural blueprint.

- [x] **DAW**: Fully featured, allows for creating new tracks, mixing, mastering, and remixing. 
## Communication & Social

- [x] **Messenger**: Real-time encrypted messaging
- [/] **Allat**: Reddit-style forums
    - User-generated communities/subreddits with customizable rules
    - Threaded discussions with nesting (up to 10 levels)
    - Voting system (upvote/downvote) with karma tracking
    - Moderation tools: post removal, user bans, community settings
    - Topic categorization via tags and flairs
    - Rich text and media support (images, videos, links) in posts
    - Search functionality within communities and across platform
    
    **Architectural Considerations**:
      - Use `consent_manager` crate for privacy controls
      - Hexagonal architecture: separate domain, application, and infrastructure layers
      - API-first design: GraphQL API for all public operations
      - Real-time updates via WebSockets for new posts and comments
      - Integration with dabloons system for rewarding content creators

- [/] **Yapper**: Twitter-style microblogging
    - Character-limited posts (280 characters)
    - Real-time feeds with algorithmic and chronological sorting options
    - Hashtag support for topic discovery
    - Engagement metrics (likes, shares, views) per post
    - Media attachments (images, short videos) in posts
    - Follow users and see their posts in your feed
    - Direct messaging capability (via Messenger integration)
    
    **Architectural Considerations**:
      - Use `consent_manager` for privacy controls on posts
      - Screaming architecture: organize by features (posting, feed, profile)
      - Horizontal scalability for feed generation
      - Opt-in data sharing for federation and research
      - Integration with dabloons for tipping and rewards

- **Cross-platform Integration**:
    - Unified feed: Option to view Allat and Yapper content in a single feed
    - Post creation interface: Destination selector (Allat community or Yapper feed)
    - Community selection when posting to Allat
    - Cross-posting toggle (Allat posts to Yapper, with privacy controls)
    - Shared identity and reputation system

- [ ] **Presence**: Status visibility across apps
- [ ] **SocialGraph**: Relationship mapping

## Productivity & Work

These tools can help users manage their work, studies, and personal projects.

- [ ] **Docs**: A streamlined word processor for writing and editing documents. Key features would include basic formatting (bold, italics, lists), the ability to insert images, and options to export to PDF or .docx formats.

- [/] **Sheets**: A simple spreadsheet application for creating budgets, tracking lists, or managing simple data. It should handle basic formulas, charts, and be compatible with formats like .xlsx.

- [ ] **Notes & Memos**: A flexible space for capturing thoughts. This could range from simple text notes to more complex notes that include checklists, images, and attachments. Features like tagging and notebooks for organization would be very useful.

- [/] **Task Manager**: A dedicated to-do list function where users can create tasks, set deadlines and reminders, and categorize them into different projects or lists.

- [/] **Calendar**: An integrated calendar that can sync with external services like Google Calendar and Outlook. It would manage events, set reminders, and offer different views (daily, weekly, monthly).

- [/] **Website Builder**: This is for everyone from individuals to large businesses, fully featured, easy to use, feature rich. Also have a link-in-bio site builder. Both have great UI to let users build for both web and mobile, and both integrate well with things like payment processors (including our own) and sales platforms (or from their own shop, with a storebuilder wizard).

## Finance & Economy

- [/] **Finance-Sheets**: Financial planning templates - *ACTIVE DEVELOPMENT*
  - [ ] Currency Internationalization (Priority 1)
  - [ ] Mobile Optimization (Priority 3)
  - [ ] Dashboard Integration (Priority 4)
  - [ ] Comprehensive Test Suite (Priority 2)
- [ ] **Wallet**: Multi-currency digital wallet
- [ ] **Invoicing**: Automated invoice generation
- [ ] **Budget**: Personal and household budgeting

## Education & Knowledge

- [ ] **Learn**: Interactive learning platform
- [ ] **Wiki**: Decentralized knowledge base
- [ ] **Research**: Collaborative academic tools
- [ ] **Skills**: Skill tracking and certification

## Media & Creativity

- [ ] **Gallery**: Photo and video management
- [ ] **Audio**: Music creation and sharing
- [ ] **Video**: Video editing and streaming
- [ ] **Art**: Digital creation tools

## Health & Wellness

- [ ] **Health**: Personal health tracking
- [ ] **Fitness**: Workout planning and tracking
- [ ] **Nutrition**: Meal planning and logging
- [ ] **Mindfulness**: Meditation and mental health

## Community & Governance

- [ ] **Commons**: Community resource management
- [ ] **Votes**: Decentralized decision making
- [ ] **Proposals**: Idea submission and refinement
- [ ] **Reputation**: Contribution tracking system

## Technical Infrastructure

- [x] **Identity**: Oauth 2.0 Identity management
- [ ] **Storage**: Distributed file storage
- [ ] **Network**: P2P networking layer
- [ ] **Compute**: Distributed computing resources

## Specialized Applications

- [ ] **Farming**: Agricultural planning tools
- [ ] **Manufacturing**: Production planning
- [ ] **Logistics**: Supply chain management
- [ ] **Energy**: Renewable energy management

## Personal Finance

This is a powerful category that can provide immense value to users by helping them understand and manage their financial lives.

- [/] **Budget Planner**: Tools for users to set monthly or weekly budgets for different spending categories (e.g., groceries, entertainment, transport). Visual progress bars can make this engaging.

- [ ] **Expense Tracker**: A simple way to log daily expenses. To make this even easier, you could include features like receipt scanning (using the phone's camera) to automatically pull details.

- [/] **Subscription Manager**: A tool specifically for tracking recurring payments like streaming services, gym memberships, and software subscriptions. It can notify users before a payment is due.

- [/] **Savings Goals**: A feature that allows users to set specific savings goals (e.g., "Vacation Fund," "New Laptop") and track their progress.

- [ ] **Personal BI Dashboard**: This is where your BI idea comes to life for the average person. It wouldn't be complex business intelligence, but rather a personal dashboard that provides simple, clear insights into their finances. This could include charts showing spending trends over time, a breakdown of income versus expenses, and a net worth tracker.

## Health & Wellness

Integrating wellness features can make the app a daily companion for users looking to improve their physical and mental well-being.

- [ ] **Habit Tracker**: Part of the health domain module (`apps/health/`) for tracking positive habits like drinking enough water, exercising, or meditating.
- [ ] **Mood Journal**: Part of the health domain module (`apps/health/`) for logging mood with privacy controls.

- [ ] **Meditation & Mindfulness**: Part of the health domain module (`apps/health/`) with guided meditations and breathing exercises. 

- [ ] **Meal & Hydration Planner**: Part of the health domain module (`apps/health/`) for planning meals and tracking water intake.

- [x] **HIPAA-compliant audit trails**: COMPLETE implementation with comprehensive logging of all Protected Health Information (PHI) access events. Technical highlights include:
  - Research access patterns with NULL user_id handling for anonymization
  - Purpose code tracking (UserView, ProviderAccess, Research, DataSync, Admin)
  - Wearable sync logging with automatic data minimization
  - Fail-safe pattern implementation ensuring system continuity if logging fails
  - AES-256 encryption at rest for all audit logs
  - 1-year active storage with 5-year archival (6-year total retention)

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

- [ ] **Scientific Journal**: Interactive notebook environment for research and data analysis

## Implementation Notes

Each application will:
- Follow screaming architecture principles
- Implement hexagonal architecture
- Support vertical slice delivery
- Integrate with the dabloons universal income system
- Enable opt-in data sharing for federation improvements
- Prioritize accessibility and mobile-first design
- Include comprehensive test coverage (unit, integration, UI)
- Support internationalization from initial development

## Architecture Notes

### Modular Approach

- Users can "install" or enable other apps based on their needs
- Keeps the interface clean for those who only want basic functionality

### Seamless Integration
- Different parts work together (e.g., link "Vacation" savings goal to "Plan Vacation" task list)
- Unified design language across all modules
- Focus on providing the most essential functionality in a clean and accessible way, then build upon the core features with improvements later.


### Core Design Philosophy

* **True Modularity**: Each app module must function as a standalone, self-contained unit that can be developed, tested, and deployed independently
* **User Empowerment**: Users should be able to enable/disable modules at runtime without restarting the application (optional, if this is impossible)
* **Cooperative Values**: Architecture must support transparency, user control, and community participation in feature development

