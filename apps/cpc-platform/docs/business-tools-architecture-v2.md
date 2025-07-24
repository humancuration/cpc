# Business Tools Architecture (v2)

## We are refactoring from svelte to Yew!

## 1. Executive Summary

The existing architecture for the financial forecasting module is insufficient for providing a robust and scalable user experience. It relies on synchronous, blocking GraphQL mutations, contains critically flawed business logic, and lacks a backend-driven notification system.

This document proposes a revised architecture that addresses these shortcomings by introducing:

- **Asynchronous Job Processing:** Utilizing gRPC to offload long-running calculations from the `backend` to a `cpc-node` worker.
- **GraphQL Subscriptions for Results:** Providing real-time updates to the UI when a job is complete.
- **Corrected and Enhanced Forecasting Models:** Ensuring the accuracy and analytical power of the tools.
- **A Backend-Driven Notification System:** To deliver timely and relevant updates to users.

This refactor will align the implementation with the project's vision, improve performance and scalability, and provide a solid foundation for future business intelligence features.

## 2. Proposed Architecture

### 2.1. High-Level Diagram

```mermaid
graph TD
    subgraph "User Interface (SvelteKit)"
        A[ForecastingDashboard]
    end

    subgraph "Backend (Axum)"
        B[GraphQL API]
        C[gRPC Server (Job Dispatcher)]
        D[Notification Service]
    end

    subgraph "Worker (cpc-node)"
        E[gRPC Client (Job Subscriber)]
        F[Forecasting Engine]
    end

    subgraph "Data Stores"
        G[PostgreSQL Database]
    end

    A -- "1. Initiate Forecast (GraphQL Mutation)" --> B
    B -- "2. Create Job & Return Task ID" --> G
    B -- "3. Dispatch Job" --> C
    C -- "4. Stream Job to Worker" --> E
    E -- "5. Execute Calculation" --> F
    F -- "6. Store Result" --> G
    F -- "7. Notify Backend of Completion" --> C
    C -- "8. Trigger Notification" --> D
    D -- "9. Send Update via GraphQL Subscription" --> A
```

### 2.2. Core Components & Data Flow

**Step 1: Initiating a Forecast**

1.  The user interacts with the `ForecastingDashboard.svelte` component.
2.  A GraphQL Mutation (e.g., `createForecastJob`) is sent to the `backend`.
3.  The mutation is no longer blocking. It immediately creates a job record in the database with a `pending` status and a unique `task_id`.
4.  The `task_id` is returned to the UI.

**Step 2: Subscribing to Results**

1.  The UI uses the returned `task_id` to subscribe to a new GraphQL Subscription (e.g., `onForecastResult(taskId: ID!)`).
2.  This subscription will notify the UI when the job's status changes (e.g., to `completed` or `failed`).

**Step 3: Job Dispatch and Execution**

1.  The `backend`'s gRPC server, now acting as the job dispatcher, adds the new job to a queue.
2.  A `cpc-node` worker, subscribed to the `backend`'s gRPC job stream, receives the forecasting task.
3.  The `cpc-node` executes the intensive forecasting calculations using the corrected and enhanced logic in `packages/cpc-core/src/business/financial_forecasting.rs`.

**Step 4: Storing and Communicating Results**

1.  Upon completion, the `cpc-node` saves the forecast results to the database, associating them with the `task_id`.
2.  The `cpc-node` updates the job's status to `completed` in the database.
3.  The `backend` is notified of the job's completion (either via a direct gRPC call or database trigger).
4.  The `backend`'s `Notification Service` pushes the final result to the subscribed UI via the GraphQL Subscription.

## 3. Detailed Implementation Plan

### 3.1. Priority 1: Asynchronous Job Processing (The "Happy Path")

**Task 1: Create gRPC Service Definitions (`packages/cpc-protos`)**

- Define a new `JobService` with two primary RPCs:
    - `SubscribeToJobs(stream JobRequest)`: A server-streaming RPC for the `cpc-node` to receive jobs.
    - `UpdateJobStatus(JobStatusUpdate)`: A unary RPC for the `cpc-node` to report completion.

**Task 2: Implement gRPC Server in `backend`**

- Create a new module `apps/backend/src/grpc/job_server.rs`.
- Implement the `JobService`, including logic for managing a job queue.
- Integrate the gRPC server into the main `backend` application.

**Task 3: Refactor `cpc-node`**

- Remove the deprecated `OrchestratorClient`.
- Implement a new gRPC client to connect to the `backend`'s `JobService`.
- Subscribe to the job stream and execute the forecasting logic when a job is received.

**Task 4: Modify GraphQL API**

- Rename `createForecast` to `createForecastJob`.
- Update the mutation to be non-blocking, returning a `task_id`.
- Implement the `onForecastResult` subscription.

### 3.2. Priority 2: Core Logic and Notifications

**Task 5: Correct the Forecasting Engine**

- In `packages/cpc-core/src/business/financial_forecasting.rs`:
    - Fix the `run_sensitivity_analysis` function to use the original historical data.
    - Implement the `monte_carlo` and `regression_analysis` algorithms.

**Task 6: Build the Backend Notification Service**

- Create a new `apps/backend/src/notifications.rs` module.
- This service will be responsible for pushing updates to the GraphQL subscription endpoint when a job is complete.

### 3.3. Priority 3: Documentation and UI

**Task 7: Update Documentation**

- Replace `business-tools-architecture.md` with this document.
- Update all workflow diagrams and file paths in other `.md` files to reflect the new architecture.

**Task 8: Update Frontend Components**

- Modify the Svelte components to call the new `createForecastJob` mutation and handle the `onForecastResult` subscription.
- Update the UI to show a "processing" state while waiting for results.

## 4. Development Task List

1.  **[Architect]** Finalize and approve this architectural plan.
2.  **[Code]** Create gRPC protobuf definitions for the `JobService`.
3.  **[Code]** Implement the gRPC server in the `backend`.
4.  **[Code]** Refactor the `cpc-node` to act as a job worker.
5.  **[Code]** Update the GraphQL API (mutations and subscriptions).
6.  **[Code]** Correct the `financial_forecasting.rs` logic.
7.  **[Code]** Implement the backend notification service.
8.  **[Architect]** Update all related documentation.
9.  **[Code]** Update the frontend Svelte components.
10. **[QA]** Conduct end-to-end user testing based on the updated `test-scenarios.md`.