# Architectural Decision Record: EthicalScanner App Design

## Status
Proposed

## Context
We're creating a new EthicalScanner app that combines product health scoring (Yuka-like) with supply chain transparency (Import Yeti-like) for food, cosmetics, supplements, and healthcare products. This requires integrating:
- Product scanning capabilities
- Health/ingredient safety scoring (category-specific)
- Supply chain tracking (category-specific ethical considerations)
- Alternative suggestions
- Federation data sharing

## Decision
We'll implement EthicalScanner using hexagonal architecture with vertical slices for each major feature. The app will consist of:

1. **Core Domain Modules**
   - `scanner`: Barcode/QR recognition
   - `health_engine`: Health and ingredient safety scoring (category-specific)
   - `supply_chain`: Ethical scoring (category-specific)
   - `suggestions`: Alternative recommendations (cross-category where appropriate)

2. **Infrastructure**
   - PostgreSQL for product/supplier database
   - Sled for local item caching
   - GraphQL for public API
   - gRPC for internal services

3. **Integration Points**
   - Consent Manager for data permissions
   - Health app for health standards
   - Cosmetics app for ingredient databases
   - Healthcare app for safety data
   - Social app for sharing features

## Consequences
- Positive: Clear separation of concerns via hexagonal architecture
- Positive: Vertical slices enable independent feature development
- Negative: Requires careful coordination with health app team for data model alignment
- Risk: Supply chain data integration may have latency issues

## Federation Integration Plan

1. **Data Sharing Opt-in**
   - Users can choose to share anonymized scan data
   - Consent Manager controls data permissions
   - Granular control over what data is shared

2. **Crowdsourced Product Database**
   - Users contribute product information
   - Community validation through voting system
   - Version history for data corrections

3. **Real-time Updates**
   - Push notifications for supply chain changes
   - WebSocket integration for live data

4. **Integration Points**
   - Health app: Health standards alignment
   - Cosmetics app: Ingredient safety data
   - Healthcare app: Product safety alerts
   - Social app: Sharing scan results
   - Wallet app: Ethical brand rewards

5. **API Access**
   - Public GraphQL API for product data
   - Rate-limited for non-members
   - Enhanced access for federation contributors