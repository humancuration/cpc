use jni::{JNIEnv, objects::{JObject, JString}};
use jni::sys::{jboolean, jbyteArray, jsize};
use jni::errors::Result as JniResult;
use std::ptr::null_mut;
use base64; // Add base64 crate to Cargo.toml if not already present
use once_cell::sync::OnceCell;
use jni::JavaVM;
use std::sync::Mutex;

// Global JavaVM instance
static JVM: OnceCell<Mutex<JavaVM>> = OnceCell::new();

/// Initialize the global JavaVM reference
pub fn init_jvm(vm: JavaVM) {
    JVM.set(Mutex::new(vm)).expect("JVM already initialized");
}

/// Helper to convert Java string to Rust string with proper cleanup
fn get_rust_string(env: &JNIEnv, jstr: JString) -> JniResult<String> {
    if jstr.is_null() {
        return Err(jni::errors::Error::NullPtr("JString is null".to_string()));
    }
    let str = env.get_string(jstr)?;
    let rust_str = str.to_str()?.to_owned();
    env.delete_local_ref(jstr)?;
    Ok(rust_str)
}

/// Helper to convert Java byte array to Rust Vec<u8>
fn get_rust_bytes(env: &JNIEnv, jarray: jbyteArray) -> JniResult<Vec<u8>> {
    if jarray.is_null() {
        return Err(jni::errors::Error::NullPtr("jbyteArray is null".to_string()));
    }
    let len = env.get_array_length(jarray)?;
    let mut buf = vec![0; len as usize];
    env.get_byte_array_region(jarray, 0, &mut buf)?;
    Ok(buf)
}

/// Get JNIEnv for current thread
fn get_env() -> JniResult<JNIEnv<'static>> {
    let vm = JVM.get().expect("JVM not initialized").lock().unwrap();
    vm.attach_current_thread_permanently()
}

/// Store a value in secure storage with encryption
#[no_mangle]
pub extern "system" fn Java_com_wtf_SecureStorage_store(
    env: JNIEnv,
    _: JObject,
    key: JString,
    value: JString,
) -> jboolean {
    let result = || -> JniResult<jboolean> {
        // Null safety checks
        if key.is_null() || value.is_null() {
            env.throw_new("java/lang/NullPointerException", "Key or value cannot be null")?;
            return Ok(0);
        }
        
        let key_alias = get_rust_string(&env, key)?;
        let value_data = get_rust_string(&env, value)?;
        
        log::info!("Storing secure value for key: {}", key_alias);
        
        // Get Android context
        let context = env.call_static_method(
            "android/app/ActivityThread",
            "currentApplication",
            "()Landroid/app/Application;",
            &[]
        )?.l()?;

        // Get SharedPreferences
        let prefs = env.call_method(
            context,
            "getSharedPreferences",
            "(Ljava/lang/String;I)Landroid/content/SharedPreferences;",
            &[
                env.new_string("SecureStore")?.into(),
                (0 as jsize).into() // MODE_PRIVATE
            ]
        )?.l()?;

        // Encrypt data
        let encrypted = Java_com_wtf_SecureStorage_encrypt(
            env,
            _,
            env.new_string(&key_alias)?.into(),
            env.byte_array_from_slice(value_data.as_bytes())?
        )?;

        // Store encrypted data
        let editor = env.call_method(
            prefs,
            "edit",
            "()Landroid/content/SharedPreferences$Editor;",
            &[]
        )?.l()?;

        env.call_method(
            editor,
            "putString",
            "(Ljava/lang/String;Ljava/lang/String;)Landroid/content/SharedPreferences$Editor;",
            &[
                env.new_string(&key_alias)?.into(),
                env.new_string(base64::encode(unsafe { env.get_elements(&encrypted) }))?.into()
            ]
        )?.l()?;

        env.call_method(editor, "apply", "()V", &[])?;
        Ok(1)
    };

    result().unwrap_or(0)
}

/// Retrieve and decrypt value from secure storage
#[no_mangle]
pub extern "system" fn Java_com_wtf_SecureStorage_retrieve(
    env: JNIEnv,
    _: JObject,
    key: JString,
) -> jbyteArray {
    let result = || -> JniResult<jbyteArray> {
        // Null safety check
        if key.is_null() {
            env.throw_new("java/lang/NullPointerException", "Key cannot be null")?;
            return Ok(null_mut());
        }
        
        let key_alias = get_rust_string(&env, key)?;
        log::info!("Retrieving secure value for key: {}", key_alias);
        
        // Get Android context
        let context = env.call_static_method(
            "android/app/ActivityThread",
            "currentApplication",
            "()Landroid/app/Application;",
            &[]
        )?.l()?;

        // Get SharedPreferences
        let prefs = env.call_method(
            context,
            "getSharedPreferences",
            "(Ljava/lang/String;I)Landroid/content/SharedPreferences;",
            &[
                env.new_string("SecureStore")?.into(),
                (0 as jsize).into() // MODE_PRIVATE
            ]
        )?.l()?;

        // Retrieve encrypted data
        let encrypted = env.call_method(
            prefs,
            "getString",
            "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;",
            &[
                env.new_string(&key_alias)?.into(),
                null_mut().into()
            ]
        )?.l()?;

        if encrypted.is_null() {
            return Ok(null_mut());
        }

        let encrypted_str = get_rust_string(&env, encrypted.into())?;
        let encrypted_bytes = base64::decode(&encrypted_str)?;
        
        // Decrypt data
        Java_com_wtf_SecureStorage_decrypt(
            env,
            _,
            env.new_string(&key_alias)?.into(),
            env.byte_array_from_slice(&encrypted_bytes)?
        )
    };

    result().unwrap_or(null_mut())
}

#[no_mangle]
pub extern "system" fn Java_com_wtf_SecureStorage_encrypt(
    env: JNIEnv,
    _: JObject,
    key_alias: JString,
    data: jbyteArray,
) -> jbyteArray {
    let result = || -> JniResult<jbyteArray> {
        if key_alias.is_null() || data.is_null() {
            env.throw_new("java/lang/NullPointerException", "Key alias or data cannot be null")?;
            return Ok(null_mut());
        }
        
        let key_alias = get_rust_string(&env, key_alias)?;
        let data_bytes = get_rust_bytes(&env, data)?;
        
        log::info!("Encrypting data for key: {}", key_alias);
        
        // Get KeyStore instance
        let key_store = env.find_class("java/security/KeyStore")?;
        let key_store = env.call_static_method(
            key_store,
            "getInstance",
            "(Ljava/lang/String;)Ljava/security/KeyStore;",
            &[env.new_string("AndroidKeyStore")?.into()]
        )?.l()?;
        
        // Load KeyStore
        env.call_method(key_store, "load", "(Ljava/security/KeyStore$LoadStoreParameter;)V", &[null_mut().into()])?;
        
        let key = if env.call_method(key_store, "containsAlias", "(Ljava/lang/String;)Z", &[env.new_string(&key_alias)?.into()])?.z()? {
            // Get existing key
            env.call_method(key_store, "getKey", "(Ljava/lang/String;[C)Ljava/security/Key;", &[
                env.new_string(&key_alias)?.into(),
                null_mut().into()
            ])?.l()?
        } else {
            // Generate new key
            let key_gen = env.find_class("javax/crypto/KeyGenerator")?;
            let key_gen = env.call_static_method(
                key_gen,
                "getInstance",
                "(Ljava/lang/String;Ljava/lang/String;)Ljavax/crypto/KeyGenerator;",
                &[
                    env.new_string("AES")?.into(),
                    env.new_string("AndroidKeyStore")?.into()
                ]
            )?.l()?;
            
            // Create key specification
            let key_spec = env.find_class("android/security/keystore/KeyGenParameterSpec")?;
            let builder = env.call_method(
                key_spec,
                "Builder",
                "(Ljava/lang/String;I)V",
                &[
                    env.new_string(&key_alias)?.into(),
                    (KeyProperties::PURPOSE_ENCRYPT | KeyProperties::PURPOSE_DECRYPT).into()
                ]
            )?.l()?;
            
            env.call_method(
                builder,
                "setBlockModes",
                "([Ljava/lang/String;)Landroid/security/keystore/KeyGenParameterSpec$Builder;",
                &[env.new_object_array(1, "java/lang/String", env.new_string("GCM")?)?.into()]
            )?.l()?;
            
            env.call_method(
                builder,
                "setEncryptionPaddings",
                "([Ljava/lang/String;)Landroid/security/keystore/KeyGenParameterSpec$Builder;",
                &[env.new_object_array(1, "java/lang/String", env.new_string("NoPadding")?)?.into()]
            )?.l()?;
            
            env.call_method(
                builder,
                "setKeySize",
                "(I)Landroid/security/keystore/KeyGenParameterSpec$Builder;",
                &[256.into()]
            )?.l()?;
            
            // Initialize key generator
            env.call_method(
                key_gen,
                "init",
                "(Ljava/security/spec/AlgorithmParameterSpec;)V",
                &[builder.into()]
            )?;
            
            // Generate key
            env.call_method(key_gen, "generateKey", "()Ljavax/crypto/SecretKey;", &[])?.l()?
        };
        
        // Get cipher instance
        let cipher = env.find_class("javax/crypto/Cipher")?;
        let cipher = env.call_static_method(
            cipher,
            "getInstance",
            "(Ljava/lang/String;)Ljavax/crypto/Cipher;",
            &[env.new_string("AES/GCM/NoPadding")?.into()]
        )?.l()?;
        
        // Initialize cipher for encryption
        env.call_method(
            cipher,
            "init",
            "(ILjava/security/Key;)V",
            &[
                Cipher::ENCRYPT_MODE.into(),
                key.into()
            ]
        )?;
        
        // Encrypt data
        let encrypted = env.call_method(
            cipher,
            "doFinal",
            "([B)[B",
            &[data.into()]
        )?.l()?;
        
        // Get IV
        let iv = env.call_method(cipher, "getIV", "()[B", &[])?.l()?;
        
        // Combine IV + encrypted data
        let iv_bytes = get_rust_bytes(&env, iv.into())?;
        let encrypted_bytes = get_rust_bytes(&env, encrypted.into())?;
        let combined = [iv_bytes, encrypted_bytes].concat();
        
        // Return combined IV + encrypted data
        env.byte_array_from_slice(&combined)
    };

    result().unwrap_or(null_mut())
}

#[no_mangle]
pub extern "system" fn Java_com_wtf_SecureStorage_decrypt(
    env: JNIEnv,
    _: JObject,
    key_alias: JString,
    encrypted_data: jbyteArray,
) -> jbyteArray {
    let result = || -> JniResult<jbyteArray> {
        if key_alias.is_null() || encrypted_data.is_null() {
            env.throw_new("java/lang/NullPointerException", "Key alias or encrypted data cannot be null")?;
            return Ok(null_mut());
        }
        
        let key_alias = get_rust_string(&env, key_alias)?;
        let encrypted_bytes = get_rust_bytes(&env, encrypted_data)?;
        
        log::info!("Decrypting data for key: {}", key_alias);
        
        // Split IV and encrypted data
        let iv = &encrypted_bytes[0..12];
        let data = &encrypted_bytes[12..];
        
        // Get KeyStore instance
        let key_store = env.find_class("java/security/KeyStore")?;
        let key_store = env.call_static_method(
            key_store,
            "getInstance",
            "(Ljava/lang/String;)Ljava/security/KeyStore;",
            &[env.new_string("AndroidKeyStore")?.into()]
        )?.l()?;
        
        // Load KeyStore
        env.call_method(key_store, "load", "(Ljava/security/KeyStore$LoadStoreParameter;)V", &[null_mut().into()])?;
        
        // Get key
        let key = env.call_method(key_store, "getKey", "(Ljava/lang/String;[C)Ljava/security/Key;", &[
            env.new_string(&key_alias)?.into(),
            null_mut().into()
        ])?.l()?;
        
        // Get cipher instance
        let cipher = env.find_class("javax/crypto/Cipher")?;
        let cipher = env.call_static_method(
            cipher,
            "getInstance",
            "(Ljava/lang/String;)Ljavax/crypto/Cipher;",
            &[env.new_string("AES/GCM/NoPadding")?.into()]
        )?.l()?;
        
        // Create GCMParameterSpec
        let gcm_spec = env.find_class("javax/crypto/spec/GCMParameterSpec")?;
        let gcm_spec = env.new_object(
            gcm_spec,
            "(I[B)V",
            &[
                128.into(),
                env.byte_array_from_slice(iv)?.into()
            ]
        )?;
        
        // Initialize cipher for decryption
        env.call_method(
            cipher,
            "init",
            "(ILjava/security/Key;Ljava/security/spec/AlgorithmParameterSpec;)V",
            &[
                Cipher::DECRYPT_MODE.into(),
                key.into(),
                gcm_spec.into()
            ]
        )?;
        
        // Decrypt data
        let decrypted = env.call_method(
            cipher,
            "doFinal",
            "([B)[B",
            &[env.byte_array_from_slice(data)?.into()]
        )?.l()?;
        
        // Return decrypted data
        Ok(decrypted.into_inner())
    };

    result().unwrap_or(null_mut())
}

// Constants for KeyProperties (matching Android SDK)
mod KeyProperties {
    pub const PURPOSE_ENCRYPT: i32 = 1;
    pub const PURPOSE_DECRYPT: i32 = 2;
}

// Constants for Cipher modes
mod Cipher {
    pub const ENCRYPT_MODE: i32 = 1;
    pub const DECRYPT_MODE: i32 = 2;
}

/// Tauri command to securely store a value
#[command]
pub fn secure_store(key: String, value: String) -> Result<(), String> {
    let env = get_env().map_err(|e| e.to_string())?;
    
    let key_jstr = env.new_string(&key)
        .map_err(|e| e.to_string())?;
    let value_jstr = env.new_string(&value)
        .map_err(|e| e.to_string())?;
    
    let result = Java_com_wtf_SecureStorage_store(
        env,
        JObject::null(),
        key_jstr.into(),
        value_jstr.into()
    );
    
    if result == 1 {
        Ok(())
    } else {
        // Check if Java exception occurred and get its message
        let exception = env.exception_occurred()
            .map(|e| {
                let _ = env.exception_clear();
                let message = env.call_method(e, "getMessage", "()Ljava/lang/String;", &[])
                    .and_then(|v| get_rust_string(&env, v.l()?.into()))
                    .unwrap_or_else(|_| "Unknown error".to_string());
                format!("Java exception: {}", message)
            });
        
        Err(exception.unwrap_or_else(|| "Failed to store secure value".into()))
    }
}

/// Tauri command to securely retrieve a value
#[command]
pub fn secure_retrieve(key: String) -> Result<Option<String>, String> {
    let env = get_env().map_err(|e| e.to_string())?;
    
    let key_jstr = env.new_string(&key)
        .map_err(|e| e.to_string())?;
    
    let result = Java_com_wtf_SecureStorage_retrieve(
        env,
        JObject::null(),
        key_jstr.into()
    );
    
    if result.is_null() {
        return Ok(None);
    }
    
    let bytes = unsafe { env.get_elements(result) };
    let value = String::from_utf8(bytes.to_vec())
        .map_err(|e| e.to_string())?;
    
    Ok(Some(value))
}

/// Tauri command to securely store a value
#[command]
pub fn secure_store(key: String, value: String) -> Result<(), String> {
    // This will eventually call into Android JNI
    println!("Secure store: {} = {}", key, value);
    Ok(())
}

/// Tauri command to securely retrieve a value
#[command]
pub fn secure_retrieve(key: String) -> Result<Option<String>, String> {
    // This will eventually call into Android JNI
    println!("Secure retrieve: {}", key);
    Ok(None)
}