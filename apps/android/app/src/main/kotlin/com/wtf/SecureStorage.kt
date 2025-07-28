package com.wtf

import android.content.Context
import android.security.keystore.KeyGenParameterSpec
import android.security.keystore.KeyProperties
import android.util.Base64
import java.security.KeyStore
import javax.crypto.Cipher
import javax.crypto.KeyGenerator
import javax.crypto.SecretKey
import javax.crypto.spec.GCMParameterSpec

class SecureStorage(context: Context) {
    private val keyStore: KeyStore = KeyStore.getInstance("AndroidKeyStore").apply {
        load(null)
    }

    private val sharedPreferences = context.getSharedPreferences("SecureStore", Context.MODE_PRIVATE)

    fun store(key: String, value: String): Boolean {
        return try {
            val encryptedData = encryptData(value.toByteArray(), key)
            val editor = sharedPreferences.edit()
            editor.putString(key, Base64.encodeToString(encryptedData, Base64.DEFAULT))
            editor.apply()
            true
        } catch (e: Exception) {
            false
        }
    }

    fun retrieve(key: String): String? {
        return try {
            val encryptedData = sharedPreferences.getString(key, null) ?: return null
            val decodedData = Base64.decode(encryptedData, Base64.DEFAULT)
            String(decryptData(decodedData, key))
        } catch (e: Exception) {
            null
        }
    }

    private fun getOrCreateKey(keyAlias: String): SecretKey {
        if (!keyStore.containsAlias(keyAlias)) {
            val keyGenerator = KeyGenerator.getInstance(
                KeyProperties.KEY_ALGORITHM_AES,
                "AndroidKeyStore"
            )
            keyGenerator.init(
                KeyGenParameterSpec.Builder(
                    keyAlias,
                    KeyProperties.PURPOSE_ENCRYPT or KeyProperties.PURPOSE_DECRYPT
                )
                .setBlockModes(KeyProperties.BLOCK_MODE_GCM)
                .setEncryptionPaddings(KeyProperties.ENCRYPTION_PADDING_NONE)
                .setKeySize(256)
                .build()
            )
            return keyGenerator.generateKey()
        }
        return keyStore.getKey(keyAlias, null) as SecretKey
    }

    private fun encryptData(data: ByteArray, keyAlias: String): ByteArray {
        val key = getOrCreateKey(keyAlias)
        val cipher = Cipher.getInstance("AES/GCM/NoPadding")
        cipher.init(Cipher.ENCRYPT_MODE, key)
        val iv = cipher.iv
        val encrypted = cipher.doFinal(data)
        return iv + encrypted
    }

    private fun decryptData(encryptedData: ByteArray, keyAlias: String): ByteArray {
        val key = getOrCreateKey(keyAlias)
        val cipher = Cipher.getInstance("AES/GCM/NoPadding")
        val iv = encryptedData.copyOfRange(0, 12)
        val data = encryptedData.copyOfRange(12, encryptedData.size)
        val spec = GCMParameterSpec(128, iv)
        cipher.init(Cipher.DECRYPT_MODE, key, spec)
        return cipher.doFinal(data)
    }
}