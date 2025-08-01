# CPay Social and Volunteerism Enhancements

## Overview
This document describes the enhancements made to the CPay system to support social sharing and volunteerism features.

## Changes Made

### 1. Data Model Updates

#### PaymentRequest Model
- Added `is_public: bool` - Whether the transaction is public
- Added `share_to_social: bool` - Whether to share the transaction to social media
- Added `cause_id: Option<Uuid>` - Optional cause ID for donations
- Added `volunteer_hours: Option<Decimal>` - Optional volunteer hours associated with the transaction

#### Transaction Model
- Added `social_post_id: Option<Uuid>` - Optional social post ID
- Added `volunteer_hours: Option<Decimal>` - Optional volunteer hours associated with the transaction

#### New Models
- `Cause` - Structure for donation causes
- `SkillRate` - Structure for skill exchange rates

### 2. Database Schema Updates

#### Migration Script
Created `20250801000002_add_social_and_volunteer_fields_to_traditional_currency_transactions.sql`:
- Added `social_post_id` column to `traditional_currency_transactions` table
- Added `volunteer_hours` column to `traditional_currency_transactions` table
- Added indexes for the new columns

### 3. Transaction Engine Updates

#### New Methods
- `perform_post_transaction_actions` - Performs post-transaction actions based on request flags
- `create_social_post` - Creates a social post for the transaction
- `record_donation` - Records a donation to a cause
- `calculate_volunteer_hours` - Calculates and stores volunteer hours

### 4. gRPC Service Updates

#### New Endpoints
- `GetFeaturedCauses` - Returns featured causes for donations
- `GetSkillExchangeRates` - Returns skill exchange rates for volunteer hour conversion

#### Updated Endpoints
- `ProcessPayment` - Now handles new fields in PaymentRequest
- `GetTransactionHistory` - Now includes social_post_id and volunteer_hours in response

### 5. Proto Definition Updates

#### New Messages
- `Cause` - Cause structure
- `SkillRate` - Skill rate structure
- `FeaturedCausesRequest` - Request for featured causes
- `FeaturedCausesResponse` - Response with featured causes
- `SkillExchangeRatesRequest` - Request for skill exchange rates
- `SkillExchangeRatesResponse` - Response with skill exchange rates

#### Updated Messages
- `PaymentRequest` - Added new fields
- `Transaction` - Added new fields

## Next Steps

### UI Implementation
- Add social sharing options to payment form
- Add cause selection dropdown
- Implement volunteer hour input with conversion display
- Add social indicators to transaction history

### Backend Implementation
- Implement actual social post creation via SocialIntegrationService
- Implement actual donation recording
- Implement actual volunteer hour calculation
- Connect to real causes and skills repositories

## Testing
- Update existing tests to cover new functionality
- Add new tests for post-transaction actions
- Add tests for new gRPC endpoints