# EthicalScanner Supply Chain Design

## Overview
The Supply Chain component is responsible for tracking and scoring the ethical aspects of product supply chains. It considers factors like labor conditions, environmental impact, and industry-specific criteria for different product types.

## Base Ethical Scoring
The base ethical score is calculated from:
- Average ethical ratings across supply chain nodes
- Average environmental impact across supply chain nodes

The score is primarily based on ethical ratings (80% weight) but penalized for environmental impact (20% weight).

## Product Type Modifiers

### Cosmetic Products
Additional modifiers for cosmetic products:
- Penalty for animal testing (ethical rating < 0.5)
- Penalty for microplastics use (environmental impact > 0.7)

### Supplement Products
Additional modifiers for supplement products:
- Bonus for third-party testing (ethical rating > 0.7)
- Penalty for contamination risk (environmental impact > 0.6)

### Healthcare Products
Additional modifiers for healthcare products:
- Bonus for FDA compliance (ethical rating > 0.8)
- Bonus for clinical trials (ethical rating > 0.7)

### Food Products
No additional modifiers for food products.

## Implementation Details
The supply chain service first calculates a base score using the standard algorithm, then applies product-type-specific modifiers based on the supply chain nodes.

## Error Handling
The supply chain service can return `SupplyChainError` for cases such as data fetch errors, consent errors, or scoring errors.