# Product Model Alignment Analysis

## Overview
This document compares the legacy Android Kotlin product models with the Rust core implementation, identifies gaps, and proposes changes to achieve full functional parity.

## Field Comparison

| Field | Kotlin (Product.kt) | Rust (model.rs) | Gap | Proposed Change |
|-------|---------------------|-----------------|-----|-----------------|
| **id** | `String` | `String` | None | - |
| **name** | `String` | `String` | None | - |
| **brand** | `String?` | `Option<String>` | None | - |
| **description** | `String` | `String` | Kotlin has non-null, Rust has non-optional | Convert to `Option<String>` |
| **barcode** | `String?` | `Option<String>` | None | - |
| **carbonFootprint** | `Double?` | `f32` | Type and nullability mismatch | Change to `Option<f32>` |
| **packagingType** | `String?` | `Option<String>` | None | - |
| **nutritionalInfo** | `String?` | `Option<String>` | None | - |
| **manufacturer** | `String?` | `Option<String>` | None | - |
| **materialCost** | `Double?` | `f32` | Type and nullability mismatch | Change to `Option<f32>` |
| **laborCost** | `Double?` | `f32` | Type and nullability mismatch | Change to `Option<f32>` |
| **supplier** | `String?` | `Option<String>` | None | - |
| **currentStock** | `Int?` | `u32` | Type and nullability mismatch | Change to `Option<u32>` |
| **reorderLevel** | `Int?` | `u32` | Type and nullability mismatch | Change to `Option<u32>` |
| **supplyChain** | `String?` | `Option<SupplyChain>` | Type mismatch (String vs struct) | Use `Option<SupplyChain>` |
| **cost** | `Money?` | Missing | Entire field missing | Add `Option<Money>` |
| **location** | `WarehouseLocation?` | Missing | Entire field missing | Add `Option<WarehouseLocation>` |

## Missing Functionality in Rust Core

1. **Cost field support**:
   - Money struct exists in models.rs but not integrated with Product
   - Validation logic differs (Kotlin has runtime checks, Rust uses validator crate)

2. **Location field support**:
   - WarehouseLocation struct exists but not connected to Product
   - No conversion logic in serialization

3. **Nullability handling**:
   - Rust uses non-optional types for numeric fields vs Kotlin's nullable types
   - Causes potential data loss and mismatched semantics

4. **Validation**:
   - Kotlin validates Money in init block
   - Rust has validation attributes but not consistently applied

## Proposed Rust Core Modifications

### 1. Update Product Model (model.rs)
```rust
use crate::models::{Money, WarehouseLocation};

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Product {
    #[validate(length(min = 1, message = "ID is required"))]
    pub id: String,
    
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    
    #[validate(length(max = 100))]
    pub brand: Option<String>,
    
    #[validate(length(max = 1000))]
    pub description: Option<String>,
    
    pub barcode: Option<String>,
    
    #[validate(range(min = 0))]
    pub carbon_footprint: Option<f32>,
    
    #[validate(length(max = 50))]
    pub packaging_type: Option<String>,
    
    pub nutritional_info: Option<String>,
    
    #[validate(length(max = 100))]
    pub manufacturer: Option<String>,
    
    #[validate(range(min = 0))]
    pub material_cost: Option<f32>,
    
    #[validate(range(min = 0))]
    pub labor_cost: Option<f32>,
    
    #[validate(length(max = 100))]
    pub supplier: Option<String>,
    
    #[validate(range(min = 0))]
    pub current_stock: Option<u32>,
    
    #[validate(range(min = 0))]
    pub reorder_level: Option<u32>,
    
    pub supply_chain: Option<SupplyChain>,
    
    #[validate]
    pub cost: Option<Money>,
    
    #[validate]
    pub location: Option<WarehouseLocation>,
}
```

### 2. Update Serialization Logic (serialization.rs)
```rust
impl From<&Product> for AndroidProduct {
    fn from(product: &Product) -> Self {
        AndroidProduct {
            // Existing fields...
            cost: product.cost.as_ref().map(|money| AndroidMoney {
                amount: money.amount,
                currency: money.currency.clone(),
            }),
            location: product.location.as_ref().map(|loc| AndroidWarehouseLocation {
                id: loc.id.clone(),
                name: loc.name.clone(),
            }),
        }
    }
}
```

### 3. Add JNI Helper Functions (jni_bridge.rs)
```rust
#[no_mangle]
pub extern "system" fn Java_com_cpc_android_JniBridge_getProductCost(
    env: JNIEnv,
    _class: JClass,
    product_ptr: jlong
) -> jobject {
    let product = unsafe { &*(product_ptr as *const Product) };
    match &product.cost {
        Some(money) => {
            let class = env.find_class("com/cpc/social/models/Money").unwrap();
            let amount = env.new_double(money.amount).unwrap();
            let currency = env.new_string(&money.currency).unwrap();
            env.new_object(class, "(DLjava/lang/String;)V", &[amount.into(), currency.into()]).unwrap().into_raw()
        }
        None => std::ptr::null_mut(),
    }
}

// Similar functions for location access
```

## JNI Compatibility Assessment

### Current Status
- Kotlin → Rust conversion in JniConversion.kt partially implemented but incomplete
- Rust → Kotlin conversion missing for new fields
- Type mismatches in numeric fields cause potential data loss

### Required Updates
1. **JniConversion.kt**:
   - Add cost/location handling in `productToJni()` and `fromJniProduct()`
   - Implement nullability conversions for numeric fields

2. **jni_bridge.rs**:
   - Add proper null handling for optional numeric fields
   - Implement getters/setters for cost and location
   - Add validation error handling

### Critical Issues
1. **Type Safety**:
   - Kotlin's Double? maps to Rust's Option<f32> (potential precision loss)
   - Consider using f64 in Rust or explicit conversion

2. **Validation Parity**:
   - Kotlin validates in constructor, Rust uses attributes
   - Need consistent validation approach

## Next Steps
1. Implement proposed Rust model changes
2. Update serialization logic
3. Enhance JNI bridge with null handling
4. Align validation approaches
5. Add comprehensive tests for cross-language interoperability

## Dependencies
- Add validator dependency in Cargo.toml if not present
- Ensure protobuf definitions include new fields
- Update Android build to include new JNI methods