# EthicalScanner Health Engine Design

## Overview
The Health Engine component is responsible for calculating health scores for various product types including food, cosmetics, supplements, and healthcare products. Each product type has its own scoring algorithm based on relevant factors.

## Product Type Scoring Algorithms

### Food Products
Food products are scored based on:
- Nutritional information (calories, protein, carbs, fats, sugars, fiber, sodium)
- Ingredient quality (positive, neutral, or negative health impact)

The score is calculated as a weighted average of nutritional score (70%) and ingredient score (30%).

### Cosmetic Products
Cosmetic products are scored based on:
- EWG (Environmental Working Group) scores of ingredients (0-10 hazard score)
- Allergen content

The base score starts at 1.0 and is reduced based on ingredient hazard scores and allergen content.

### Supplement Products
Supplement products are scored based on:
- Daily value percentage (optimal is around 100%)
- Verification status (third-party tested)
- Warning labels

The score starts at 0.5 and is adjusted based on these factors.

### Healthcare Products
Healthcare products receive a neutral score of 0.7 as they are typically regulated and prescribed by professionals.

## Implementation Details
The health engine uses pattern matching on the product type to dispatch to the appropriate scoring function. Each scoring function returns a score between 0.0 and 1.0, where 1.0 is the healthiest/best option.

## Error Handling
The health engine can return `HealthEngineError` for cases such as insufficient data or calculation errors.