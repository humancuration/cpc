fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute(".", "#[derive(validator::Validate)]")
        .type_attribute("cpc.core.WarehouseLocation", "#[derive(Validate)]")
        .field_attribute(".cpc.core.Product.id", "#[validate(length(min = 1, message = \"ID is required\"))]")
        .field_attribute(".cpc.core.Product.name", "#[validate(length(min = 1, max = 100, message = \"Name must be between 1-100 characters\"))]")
        .field_attribute(".cpc.core.Product.brand", "#[validate(length(max = 100))]")
        .field_attribute(".cpc.core.Product.description", "#[validate(length(max = 1000))]")
        .field_attribute(".cpc.core.Product.carbon_footprint", "#[validate(range(min = 0))]")
        .field_attribute(".cpc.core.Product.packaging_type", "#[validate(length(max = 50))]")
        .field_attribute(".cpc.core.Product.manufacturer", "#[validate(length(max = 100))]")
        .field_attribute(".cpc.core.Product.material_cost", "#[validate(range(min = 0))]")
        .field_attribute(".cpc.core.Product.labor_cost", "#[validate(range(min = 0))]")
        .field_attribute(".cpc.core.Product.supplier", "#[validate(length(max = 100))]")
        .field_attribute(".cpc.core.Product.current_stock", "#[validate(range(min = 0))]")
        .field_attribute(".cpc.core.Product.reorder_level", "#[validate(range(min = 0))]")
        .field_attribute(".cpc.core.WarehouseLocation.id", "#[validate(length(min = 1, message = \"ID must be 1-50 characters\"))]")
        .field_attribute(".cpc.core.WarehouseLocation.name", "#[validate(length(min = 1, max = 100, message = \"Name must be 1-100 characters\"))]")
        .compile(
            &[
                "proto/job_service.proto",
                "proto/file_hosting.proto",
                "proto/social_features.proto",
                "proto/financial_forecast.proto",
                "proto/invoicing.proto",
                "proto/metrics.proto",
                "proto/core/product.proto",
                "protos/impact.proto",
                "protos/android_data.proto"
            ],
            &["proto", "protos"],
        )?;
        
    generate_jni_bindings()?;
    Ok(())
}

fn generate_jni_bindings() -> Result<(), Box<dyn std::error::Error>> {
    use std::fs::File;
    use std::io::Write;
    use std::path::PathBuf;
    
    let out_dir = PathBuf::from(std::env::var("OUT_DIR")?);
    let jni_file = out_dir.join("jni_bindings.rs");
    let mut f = File::create(&jni_file)?;
    
    writeln!(f, "// AUTO-GENERATED JNI BINDINGS\n")?;
    writeln!(f, "use jni::{{JNIEnv, objects::JClass, sys::{{jbyteArray, jlong}}}};")?;
    writeln!(f, "use prost::Message;")?;
    writeln!(f, "use cpc_protos::core::*;\n")?;
    
    let models = vec!["Product", "User", "Comment", "Post", "Proposal", "FeedItem", "SupplyChain"];
    
    for model in models {
        writeln!(f, "
#[no_mangle]
pub extern \"system\" fn Java_com_cpc_android_JniBridge_serialize{0}(
    env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jbyteArray {{
    let obj = unsafe {{ &*(ptr as *const {0}) }};
    let bytes = obj.encode_to_vec();
    env.byte_array_from_slice(&bytes).unwrap()
}}

#[no_mangle]
pub extern \"system\" fn Java_com_cpc_android_JniBridge_deserialize{0}(
    env: JNIEnv,
    _class: JClass,
    bytes: jbyteArray,
) -> jlong {{
    let bytes = env.convert_byte_array(bytes).unwrap();
    let obj = {0}::decode(&bytes[..]).unwrap();
    Box::into_raw(Box::new(obj)) as jlong
}}", model)?;
    }
    
    println!("cargo:rerun-if-changed=proto/core");
    Ok(())
}