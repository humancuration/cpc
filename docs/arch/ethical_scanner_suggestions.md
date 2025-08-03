# EthicalScanner Suggestions Engine Design

## Overview
The Suggestions Engine component is responsible for recommending alternative products based on a reference product. It supports cross-category alternatives, particularly for healthcare products which can be substituted with supplements.

## Alternative Finding Algorithms

### Standard Alternatives
For most product types (food, cosmetics, supplements), the engine uses standard matching algorithms that:
- Query database for similar products
- Apply matching algorithms based on category, health score, ethical score, etc.
- Rank alternatives by various criteria
- Return ranked list of suggestions

### Healthcare to Supplement Alternatives
For healthcare products, the engine specifically looks for supplement alternatives:
- Query database for supplements with similar benefits
- Apply matching algorithms based on health conditions, ingredients, etc.
- Rank alternatives by various criteria
- Return ranked list of suggestions

## Ranking Algorithm
Alternatives are ranked based on user preferences with the following weights:
- Health improvement (configurable weight)
- Ethical improvement (configurable weight)
- Local producer bonus (configurable weight)

## Implementation Details
The suggestions service uses pattern matching on the product type to determine which alternative finding algorithm to use. For healthcare products, it dispatches to the supplement alternatives function; for all other products, it uses the standard alternatives function.

## Error Handling
The suggestions service can return `SuggestionsError` for cases such as data fetch errors or matching errors.