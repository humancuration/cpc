use anyhow::{anyhow, bail, Result};
use regex::Regex;
use std::collections::BTreeMap;

/// Known generic bounds/capabilities
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum GenericBound {
    Add,
    Default,
    Ord,
    Serialize,
    RandomSeed,
    Streamable,
    Eq,
    PartialEq,
    Clone,
    Copy,
    Debug,
    Hash,
}

impl GenericBound {
    /// Parse a string into a GenericBound
    pub fn parse(s: &str) -> Result<GenericBound> {
        match s {
            "Add" => Ok(GenericBound::Add),
            "Default" => Ok(GenericBound::Default),
            "Ord" => Ok(GenericBound::Ord),
            "Serialize" => Ok(GenericBound::Serialize),
            "RandomSeed" => Ok(GenericBound::RandomSeed),
            "Streamable" => Ok(GenericBound::Streamable),
            "Eq" => Ok(GenericBound::Eq),
            "PartialEq" => Ok(GenericBound::PartialEq),
            "Clone" => Ok(GenericBound::Clone),
            "Copy" => Ok(GenericBound::Copy),
            "Debug" => Ok(GenericBound::Debug),
            "Hash" => Ok(GenericBound::Hash),
            _ => bail!("Unknown generic bound: '{}'", s),
        }
    }
    
    /// Check if a type satisfies this bound
    pub fn is_satisfied_by(&self, ty: &Type) -> bool {
        match self {
            GenericBound::Add => {
                // Add bound: numeric types (i64, f64, decimal)
                matches!(ty, Type::Scalar(ScalarType::I64) |
                              Type::Scalar(ScalarType::F64) |
                              Type::Scalar(ScalarType::Decimal))
            }
            GenericBound::Default => {
                // Default bound: types that can have a default value
                // All scalar types have defaults
                // Optional types have defaults (None)
                // Lists, maps, tuples can have empty defaults
                matches!(ty,
                    Type::Scalar(_) |
                    Type::Option(_) |
                    Type::List(_) |
                    Type::Map(_) |
                    Type::Tuple(_))
            }
            GenericBound::Ord => {
                // Ord bound: types that can be ordered
                // All scalar types except bool can be ordered
                // Lists, maps, tuples can be ordered if their elements can
                match ty {
                    Type::Scalar(s) => !matches!(s, ScalarType::Bool),
                    Type::List(inner) => GenericBound::Ord.is_satisfied_by(inner),
                    Type::Map(value) => GenericBound::Ord.is_satisfied_by(value),
                    Type::Option(inner) => GenericBound::Ord.is_satisfied_by(inner),
                    Type::Tuple(elements) => elements.iter().all(|e| GenericBound::Ord.is_satisfied_by(e)),
                    _ => false,
                }
            }
            GenericBound::Serialize => {
                // Serialize bound: types that can be serialized
                // All types in our system can be serialized
                true
            }
            GenericBound::RandomSeed => {
                // RandomSeed bound: types that can be used as random seeds
                // Only integers and bytes can be used as seeds
                matches!(ty, Type::Scalar(ScalarType::I64) | Type::Scalar(ScalarType::Bytes))
            }
            GenericBound::Streamable => {
                // Streamable bound: types that can be streamed
                // All types can be streamed
                true
            }
            GenericBound::Eq => {
                // Eq bound: types that support equality comparison
                // All types except floats support Eq (due to NaN)
                match ty {
                    Type::Scalar(s) => !matches!(s, ScalarType::F64),
                    Type::List(inner) => GenericBound::Eq.is_satisfied_by(inner),
                    Type::Map(value) => GenericBound::Eq.is_satisfied_by(value),
                    Type::Option(inner) => GenericBound::Eq.is_satisfied_by(inner),
                    Type::Tuple(elements) => elements.iter().all(|e| GenericBound::Eq.is_satisfied_by(e)),
                    Type::Struct(struct_ty) => struct_ty.fields.iter().all(|f| GenericBound::Eq.is_satisfied_by(&f.ty)),
                    Type::Enum(enum_ty) => enum_ty.variants.iter().all(|v|
                        v.ty.as_ref().map_or(true, |t| GenericBound::Eq.is_satisfied_by(t))),
                    _ => true, // Generic types are assumed to satisfy Eq
                }
            }
            GenericBound::PartialEq => {
                // PartialEq bound: types that support partial equality comparison
                // All types support PartialEq
                true
            }
            GenericBound::Clone => {
                // Clone bound: types that can be cloned
                // All types in our system can be cloned
                true
            }
            GenericBound::Copy => {
                // Copy bound: types that are copyable
                // Only scalar types are copyable
                matches!(ty, Type::Scalar(_))
            }
            GenericBound::Debug => {
                // Debug bound: types that can be debug-formatted
                // All types in our system support debug formatting
                true
            }
            GenericBound::Hash => {
                // Hash bound: types that can be hashed
                // Types that satisfy Eq can be hashed
                self.is_satisfied_by(ty) && GenericBound::Eq.is_satisfied_by(ty)
            }
        }
    }
}

/// Represents a Shtairir type with full ADT support
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Type {
    /// Scalar primitive types
    Scalar(ScalarType),
    /// List type with element type
    List(Box<Type>),
    /// Map type with string keys and value type
    Map(Box<Type>),
    /// Optional type with inner type
    Option(Box<Type>),
    /// Tuple type with element types
    Tuple(Vec<Type>),
    /// Stream type with element type
    Stream(Box<Type>),
    /// Event type with payload type
    Event(Box<Type>),
    /// Struct type with named fields
    Struct(StructType),
    /// Enum type with variants
    Enum(EnumType),
    /// Generic type parameter
    Generic(String),
    /// Structured types
    Structured(StructuredType),
}

/// Scalar primitive types
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ScalarType {
    I64,
    F64,
    Bool,
    String,
    Bytes,
    Decimal,
    DateTime,
    Duration,
    Uuid,
}

/// Structured types for object and array representations
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum StructuredType {
    /// Heterogeneous collection
    Object,
    
    /// Heterogeneous list
    Array,
    
    /// Null value
    Null,
}

/// Struct type with named fields
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct StructType {
    pub name: String,
    pub fields: Vec<StructField>,
}

/// Field in a struct
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct StructField {
    pub name: String,
    pub ty: Type,
}

/// Enum type with variants
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct EnumType {
    pub name: String,
    pub variants: Vec<EnumVariant>,
}

/// Variant in an enum
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct EnumVariant {
    pub name: String,
    /// None for unit variants, Some for tuple/struct variants
    pub ty: Option<Type>,
}

impl Type {
    /// Parse a type string into a Type
    pub fn parse(ty_str: &str) -> Result<Type> {
        let ty_str = ty_str.trim();
        
        // First check for scalar types
        if let Ok(scalar) = parse_scalar_type(ty_str) {
            return Ok(Type::Scalar(scalar));
        }
        
        // Check for composite types
        if let Some(inner) = ty_str.strip_prefix("list<").and_then(|s| s.strip_suffix('>')) {
            let inner_ty = Type::parse(inner.trim())?;
            return Ok(Type::List(Box::new(inner_ty)));
        }
        
        if let Some(inner) = ty_str.strip_prefix("option<").and_then(|s| s.strip_suffix('>')) {
            let inner_ty = Type::parse(inner.trim())?;
            return Ok(Type::Option(Box::new(inner_ty)));
        }
        
        if let Some(rest) = ty_str.strip_prefix("map<").and_then(|s| s.strip_suffix('>')) {
            let parts: Vec<&str> = rest.split(',').map(|s| s.trim()).collect();
            if parts.len() != 2 {
                bail!("map expects two type params, got '{}'", ty_str);
            }
            if parts[0] != "string" {
                bail!("map key must be 'string', got '{}'", parts[0]);
            }
            let value_ty = Type::parse(parts[1])?;
            return Ok(Type::Map(Box::new(value_ty)));
        }
        
        if ty_str.starts_with("Stream<") && ty_str.ends_with('>') {
            let inner = &ty_str[7..ty_str.len()-1];
            let inner_ty = Type::parse(inner.trim())?;
            return Ok(Type::Stream(Box::new(inner_ty)));
        }
        
        if ty_str.starts_with("Event<") && ty_str.ends_with('>') {
            let inner = &ty_str[6..ty_str.len()-1];
            let inner_ty = Type::parse(inner.trim())?;
            return Ok(Type::Event(Box::new(inner_ty)));
        }
        
        if ty_str.starts_with('(') && ty_str.ends_with(')') {
            let inner = &ty_str[1..ty_str.len()-1];
            if inner.is_empty() {
                return Ok(Type::Tuple(vec![])); // Empty tuple
            }
            let mut elements = Vec::new();
            for part in inner.split(',') {
                elements.push(Type::parse(part.trim())?);
            }
            return Ok(Type::Tuple(elements));
        }
        
        if ty_str.starts_with("Struct{") && ty_str.ends_with('}') {
            return parse_struct_type(ty_str);
        }
        
        if ty_str.starts_with("Enum{") && ty_str.ends_with('}') {
            return parse_enum_type(ty_str);
        }
        
        // Check for generic type parameters (single uppercase letter or word)
        let re = Regex::new(r"^[A-Z][a-zA-Z0-9]*$").unwrap();
        if re.is_match(ty_str) {
            return Ok(Type::Generic(ty_str.to_string()));
        }
        
        // Check for structured types
        match ty_str {
            "object" => return Ok(Type::Structured(StructuredType::Object)),
            "array" => return Ok(Type::Structured(StructuredType::Array)),
            "null" => return Ok(Type::Structured(StructuredType::Null)),
            _ => {}
        }
        
        bail!("Unknown type: '{}'", ty_str)
    }
    
    /// Convert Type back to its string representation
    pub fn to_string(&self) -> String {
        match self {
            Type::Scalar(scalar) => scalar.to_string(),
            Type::List(inner) => format!("list<{}>", inner.to_string()),
            Type::Map(value) => format!("map<string,{}>", value.to_string()),
            Type::Option(inner) => format!("option<{}>", inner.to_string()),
            Type::Tuple(elements) => {
                if elements.is_empty() {
                    "()".to_string()
                } else {
                    let elems: Vec<String> = elements.iter().map(|t| t.to_string()).collect();
                    format!("({})", elems.join(","))
                }
            }
            Type::Stream(inner) => format!("Stream<{}>", inner.to_string()),
            Type::Event(inner) => format!("Event<{}>", inner.to_string()),
            Type::Struct(struct_ty) => {
                let fields: Vec<String> = struct_ty.fields.iter()
                    .map(|f| format!("{}:{}", f.name, f.ty.to_string()))
                    .collect();
                format!("Struct{{{}}}", fields.join(","))
            }
            Type::Enum(enum_ty) => {
                let variants: Vec<String> = enum_ty.variants.iter()
                    .map(|v| match &v.ty {
                        None => v.name.clone(),
                        Some(ty) => format!("{}({})", v.name, ty.to_string()),
                    })
                    .collect();
                format!("Enum{{{}}}", variants.join(","))
            }
            Type::Generic(name) => name.clone(),
            Type::Structured(structured) => match structured {
                StructuredType::Object => "object".to_string(),
                StructuredType::Array => "array".to_string(),
                StructuredType::Null => "null".to_string(),
            },
        }
    }
    
    /// Check if this type is compatible with another type
    pub fn is_compatible_with(&self, other: &Type) -> bool {
        match (self, other) {
            // Scalar types must match exactly
            (Type::Scalar(a), Type::Scalar(b)) => a == b,
            
            // List compatibility: inner types must be compatible
            (Type::List(a), Type::List(b)) => a.is_compatible_with(b),
            
            // Map compatibility: value types must be compatible
            (Type::Map(a), Type::Map(b)) => a.is_compatible_with(b),
            
            // Option compatibility: inner types must be compatible
            (Type::Option(a), Type::Option(b)) => a.is_compatible_with(b),
            
            // Tuple compatibility: same length and compatible elements
            (Type::Tuple(a), Type::Tuple(b)) => {
                if a.len() != b.len() {
                    return false;
                }
                a.iter().zip(b.iter()).all(|(a_ty, b_ty)| a_ty.is_compatible_with(b_ty))
            }
            
            // Stream compatibility: inner types must be compatible
            (Type::Stream(a), Type::Stream(b)) => a.is_compatible_with(b),
            
            // Event compatibility: payload types must be compatible
            (Type::Event(a), Type::Event(b)) => a.is_compatible_with(b),
            
            // Struct compatibility: same name and compatible fields
            (Type::Struct(a), Type::Struct(b)) => {
                if a.name != b.name {
                    return false;
                }
                if a.fields.len() != b.fields.len() {
                    return false;
                }
                a.fields.iter().zip(b.fields.iter())
                    .all(|(a_field, b_field)| 
                        a_field.name == b_field.name && 
                        a_field.ty.is_compatible_with(&b_field.ty)
                    )
            }
            
            // Enum compatibility: same name and compatible variants
            (Type::Enum(a), Type::Enum(b)) => {
                if a.name != b.name {
                    return false;
                }
                if a.variants.len() != b.variants.len() {
                    return false;
                }
                a.variants.iter().zip(b.variants.iter())
                    .all(|(a_var, b_var)| 
                        a_var.name == b_var.name && 
                        match (&a_var.ty, &b_var.ty) {
                            (None, None) => true,
                            (Some(a_ty), Some(b_ty)) => a_ty.is_compatible_with(b_ty),
                            _ => false,
                        }
                    )
            }
            
            // Generic compatibility: names must match
            (Type::Generic(a), Type::Generic(b)) => a == b,
            
            // Incompatible types
            _ => false,
        }
    }
    
    /// Check if this type is a subtype of another (for variance)
    pub fn is_subtype_of(&self, other: &Type) -> bool {
        // For now, we'll use a simple rule: types must be exactly compatible
        // In a more sophisticated system, we could implement proper subtyping rules
        self.is_compatible_with(other)
    }
    
    /// Get all generic type parameters used in this type
    pub fn generic_params(&self) -> Vec<String> {
        match self {
            Type::Scalar(_) => Vec::new(),
            Type::List(inner) => inner.generic_params(),
            Type::Map(value) => value.generic_params(),
            Type::Option(inner) => inner.generic_params(),
            Type::Tuple(elements) => {
                let mut params = Vec::new();
                for elem in elements {
                    params.extend(elem.generic_params());
                }
                params.sort();
                params.dedup();
                params
            }
            Type::Stream(inner) => inner.generic_params(),
            Type::Event(inner) => inner.generic_params(),
            Type::Struct(struct_ty) => {
                let mut params = Vec::new();
                for field in &struct_ty.fields {
                    params.extend(field.ty.generic_params());
                }
                params.sort();
                params.dedup();
                params
            }
            Type::Enum(enum_ty) => {
                let mut params = Vec::new();
                for variant in &enum_ty.variants {
                    if let Some(ty) = &variant.ty {
                        params.extend(ty.generic_params());
                    }
                }
                params.sort();
                params.dedup();
                params
            }
            Type::Generic(name) => vec![name.clone()],
        }
    }
    
    impl std::fmt::Display for GenericBound {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                GenericBound::Add => write!(f, "Add"),
                GenericBound::Default => write!(f, "Default"),
                GenericBound::Ord => write!(f, "Ord"),
                GenericBound::Serialize => write!(f, "Serialize"),
                GenericBound::RandomSeed => write!(f, "RandomSeed"),
                GenericBound::Streamable => write!(f, "Streamable"),
                GenericBound::Eq => write!(f, "Eq"),
                GenericBound::PartialEq => write!(f, "PartialEq"),
                GenericBound::Clone => write!(f, "Clone"),
                GenericBound::Copy => write!(f, "Copy"),
                GenericBound::Debug => write!(f, "Debug"),
                GenericBound::Hash => write!(f, "Hash"),
            }
        }
    }
    
    /// Substitute generic parameters with concrete types
    pub fn substitute(&self, substitutions: &BTreeMap<String, Type>) -> Result<Type> {
        match self {
            Type::Scalar(_) => Ok(self.clone()),
            Type::List(inner) => {
                let new_inner = inner.substitute(substitutions)?;
                Ok(Type::List(Box::new(new_inner)))
            }
            Type::Map(value) => {
                let new_value = value.substitute(substitutions)?;
                Ok(Type::Map(Box::new(new_value)))
            }
            Type::Option(inner) => {
                let new_inner = inner.substitute(substitutions)?;
                Ok(Type::Option(Box::new(new_inner)))
            }
            Type::Tuple(elements) => {
                let new_elements: Result<Vec<Type>> = elements
                    .iter()
                    .map(|e| e.substitute(substitutions))
                    .collect();
                Ok(Type::Tuple(new_elements?))
            }
            Type::Stream(inner) => {
                let new_inner = inner.substitute(substitutions)?;
                Ok(Type::Stream(Box::new(new_inner)))
            }
            Type::Event(inner) => {
                let new_inner = inner.substitute(substitutions)?;
                Ok(Type::Event(Box::new(new_inner)))
            }
            Type::Struct(struct_ty) => {
                let new_fields: Result<Vec<StructField>> = struct_ty
                    .fields
                    .iter()
                    .map(|f| {
                        let new_ty = f.ty.substitute(substitutions)?;
                        Ok(StructField {
                            name: f.name.clone(),
                            ty: new_ty,
                        })
                    })
                    .collect();
                Ok(Type::Struct(StructType {
                    name: struct_ty.name.clone(),
                    fields: new_fields?,
                }))
            }
            Type::Enum(enum_ty) => {
                let new_variants: Result<Vec<EnumVariant>> = enum_ty
                    .variants
                    .iter()
                    .map(|v| {
                        let new_ty = match &v.ty {
                            None => None,
                            Some(ty) => Some(ty.substitute(substitutions)?),
                        };
                        Ok(EnumVariant {
                            name: v.name.clone(),
                            ty: new_ty,
                        })
                    })
                    .collect();
                Ok(Type::Enum(EnumType {
                    name: enum_ty.name.clone(),
                    variants: new_variants?,
                }))
            }
            Type::Generic(name) => {
                substitutions
                    .get(name)
                    .cloned()
                    .ok_or_else(|| anyhow!("No substitution for generic parameter '{}'", name))
            }
        }
    }
    
    /// Validate that this type satisfies the given bounds
    pub fn validate_bounds(&self, bounds: &[GenericBound]) -> Result<()> {
        for bound in bounds {
            if !bound.is_satisfied_by(self) {
                bail!("Type '{}' does not satisfy bound '{}'", self.to_string(), bound.to_string());
            }
        }
        Ok(())
    }
}

impl std::fmt::Display for ScalarType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScalarType::I64 => write!(f, "i64"),
            ScalarType::F64 => write!(f, "f64"),
            ScalarType::Bool => write!(f, "bool"),
            ScalarType::String => write!(f, "string"),
            ScalarType::Bytes => write!(f, "bytes"),
            ScalarType::Decimal => write!(f, "decimal"),
            ScalarType::DateTime => write!(f, "datetime"),
            ScalarType::Duration => write!(f, "duration"),
            ScalarType::Uuid => write!(f, "uuid"),
        }
    }
}

fn parse_scalar_type(ty_str: &str) -> Result<ScalarType> {
    match ty_str {
        "i64" => Ok(ScalarType::I64),
        "f64" => Ok(ScalarType::F64),
        "bool" => Ok(ScalarType::Bool),
        "string" => Ok(ScalarType::String),
        "bytes" => Ok(ScalarType::Bytes),
        "decimal" => Ok(ScalarType::Decimal),
        "datetime" => Ok(ScalarType::DateTime),
        "duration" => Ok(ScalarType::Duration),
        "uuid" => Ok(ScalarType::Uuid),
        _ => bail!("Unknown scalar type: '{}'", ty_str),
    }
}

fn parse_struct_type(ty_str: &str) -> Result<Type> {
    // Extract the content between "Struct{" and "}"
    let content = &ty_str[7..ty_str.len()-1];
    
    // Split into name and fields parts
    let parts: Vec<&str> = content.splitn(2, '{').collect();
    if parts.len() != 2 {
        bail!("Invalid struct type format: '{}'", ty_str);
    }
    
    let name = parts[0].trim();
    let fields_content = parts[1];
    
    // Remove trailing "}" if present
    let fields_content = if fields_content.ends_with('}') {
        &fields_content[..fields_content.len()-1]
    } else {
        fields_content
    };
    
    let mut fields = Vec::new();
    for field_str in fields_content.split(',') {
        let field_str = field_str.trim();
        if field_str.is_empty() {
            continue;
        }
        
        // Split field into name and type
        let field_parts: Vec<&str> = field_str.splitn(2, ':').collect();
        if field_parts.len() != 2 {
            bail!("Invalid struct field format: '{}'", field_str);
        }
        
        let field_name = field_parts[0].trim();
        let field_type = Type::parse(field_parts[1].trim())?;
        
        fields.push(StructField {
            name: field_name.to_string(),
            ty: field_type,
        });
    }
    
    Ok(Type::Struct(StructType {
        name: name.to_string(),
        fields,
    }))
}

fn parse_enum_type(ty_str: &str) -> Result<Type> {
    // Extract the content between "Enum{" and "}"
    let content = &ty_str[5..ty_str.len()-1];
    
    // Split into name and variants parts
    let parts: Vec<&str> = content.splitn(2, '{').collect();
    if parts.len() != 2 {
        bail!("Invalid enum type format: '{}'", ty_str);
    }
    
    let name = parts[0].trim();
    let variants_content = parts[1];
    
    // Remove trailing "}" if present
    let variants_content = if variants_content.ends_with('}') {
        &variants_content[..variants_content.len()-1]
    } else {
        variants_content
    };
    
    let mut variants = Vec::new();
    for variant_str in variants_content.split(',') {
        let variant_str = variant_str.trim();
        if variant_str.is_empty() {
            continue;
        }
        
        // Check if it's a unit variant or has a type
        if variant_str.contains('(') && variant_str.ends_with(')') {
            // Tuple variant
            let open_paren = variant_str.find('(').unwrap();
            let variant_name = &variant_str[..open_paren];
            let variant_type = &variant_str[open_paren+1..variant_str.len()-1];
            
            let ty = Type::parse(variant_type.trim())?;
            variants.push(EnumVariant {
                name: variant_name.to_string(),
                ty: Some(ty),
            });
        } else {
            // Unit variant
            variants.push(EnumVariant {
                name: variant_str.to_string(),
                ty: None,
            });
        }
    }
    
    Ok(Type::Enum(EnumType {
        name: name.to_string(),
        variants,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_scalar_types() {
        assert_eq!(Type::parse("i64").unwrap(), Type::Scalar(ScalarType::I64));
        assert_eq!(Type::parse("string").unwrap(), Type::Scalar(ScalarType::String));
        assert!(Type::parse("invalid").is_err());
    }
    
    #[test]
    fn test_parse_list_type() {
        let list_type = Type::parse("list<i64>").unwrap();
        assert_eq!(list_type, Type::List(Box::new(Type::Scalar(ScalarType::I64))));
    }
    
    #[test]
    fn test_parse_option_type() {
        let option_type = Type::parse("option<string>").unwrap();
        assert_eq!(option_type, Type::Option(Box::new(Type::Scalar(ScalarType::String))));
    }
    
    #[test]
    fn test_parse_map_type() {
        let map_type = Type::parse("map<string,i64>").unwrap();
        assert_eq!(map_type, Type::Map(Box::new(Type::Scalar(ScalarType::I64))));
    }
    
    #[test]
    fn test_parse_tuple_type() {
        let tuple_type = Type::parse("(i64,string)").unwrap();
        assert_eq!(tuple_type, Type::Tuple(vec![
            Type::Scalar(ScalarType::I64),
            Type::Scalar(ScalarType::String),
        ]));
    }
    
    #[test]
    fn test_parse_stream_type() {
        let stream_type = Type::parse("Stream<i64>").unwrap();
        assert_eq!(stream_type, Type::Stream(Box::new(Type::Scalar(ScalarType::I64))));
    }
    
    #[test]
    fn test_parse_event_type() {
        let event_type = Type::parse("Event<string>").unwrap();
        assert_eq!(event_type, Type::Event(Box::new(Type::Scalar(ScalarType::String))));
    }
    
    #[test]
    fn test_parse_struct_type() {
        let struct_type = Type::parse("StructPerson{name:string,age:i64}").unwrap();
        let expected = Type::Struct(StructType {
            name: "Person".to_string(),
            fields: vec![
                StructField {
                    name: "name".to_string(),
                    ty: Type::Scalar(ScalarType::String),
                },
                StructField {
                    name: "age".to_string(),
                    ty: Type::Scalar(ScalarType::I64),
                },
            ],
        });
        assert_eq!(struct_type, expected);
    }
    
    #[test]
    fn test_parse_enum_type() {
        let enum_type = Type::parse("EnumColor{Red,Green,Blue}").unwrap();
        let expected = Type::Enum(EnumType {
            name: "Color".to_string(),
            variants: vec![
                EnumVariant { name: "Red".to_string(), ty: None },
                EnumVariant { name: "Green".to_string(), ty: None },
                EnumVariant { name: "Blue".to_string(), ty: None },
            ],
        });
        assert_eq!(enum_type, expected);
    }
    
    #[test]
    fn test_parse_enum_with_data() {
        let enum_type = Type::parse("EnumResult{Ok(T),Err(string)}").unwrap();
        let expected = Type::Enum(EnumType {
            name: "Result".to_string(),
            variants: vec![
                EnumVariant { 
                    name: "Ok".to_string(), 
                    ty: Some(Type::Generic("T".to_string())) 
                },
                EnumVariant { 
                    name: "Err".to_string(), 
                    ty: Some(Type::Scalar(ScalarType::String)) 
                },
            ],
        });
        assert_eq!(enum_type, expected);
    }
    
    #[test]
    fn test_type_compatibility() {
        let type1 = Type::parse("i64").unwrap();
        let type2 = Type::parse("i64").unwrap();
        assert!(type1.is_compatible_with(&type2));
        
        let type3 = Type::parse("string").unwrap();
        assert!(!type1.is_compatible_with(&type3));
        
        let list1 = Type::parse("list<i64>").unwrap();
        let list2 = Type::parse("list<i64>").unwrap();
        assert!(list1.is_compatible_with(&list2));
        
        let list3 = Type::parse("list<string>").unwrap();
        assert!(!list1.is_compatible_with(&list3));
        
        let struct1 = Type::parse("StructPerson{name:string,age:i64}").unwrap();
        let struct2 = Type::parse("StructPerson{name:string,age:i64}").unwrap();
        assert!(struct1.is_compatible_with(&struct2));
        
        let struct3 = Type::parse("StructPerson{name:string,age:i64,address:string}").unwrap();
        assert!(!struct1.is_compatible_with(&struct3));
    }
    
    #[test]
    fn test_to_string() {
        let ty = Type::parse("list<map<string,i64>>").unwrap();
        assert_eq!(ty.to_string(), "list<map<string,i64>>");
        
        let ty = Type::parse("StructPerson{name:string,age:i64}").unwrap();
        assert_eq!(ty.to_string(), "StructPerson{name:string,age:i64}");
        
        let ty = Type::parse("EnumResult{Ok(T),Err(string)}").unwrap();
        assert_eq!(ty.to_string(), "EnumResult{Ok(T),Err(string)}");
    }
    
    #[test]
    fn test_generic_params() {
        let ty = Type::parse("list<T>").unwrap();
        assert_eq!(ty.generic_params(), vec!["T"]);
        
        let ty = Type::parse("StructPoint{x:T,y:T}").unwrap();
        assert_eq!(ty.generic_params(), vec!["T"]);
        
        let ty = Type::parse("EnumResult{Ok(T),Err(E)}").unwrap();
        let mut params = ty.generic_params();
        params.sort();
        assert_eq!(params, vec!["E", "T"]);
    }
    
    #[test]
    fn test_substitute() {
        let ty = Type::parse("list<T>").unwrap();
        let mut substitutions = BTreeMap::new();
        substitutions.insert("T".to_string(), Type::Scalar(ScalarType::I64));
        let substituted = ty.substitute(&substitutions).unwrap();
        assert_eq!(substituted, Type::parse("list<i64>").unwrap());
        
        let ty = Type::parse("StructPoint{x:T,y:T}").unwrap();
        let substituted = ty.substitute(&substitutions).unwrap();
        assert_eq!(substituted, Type::parse("StructPoint{x:i64,y:i64}").unwrap());
    }
}