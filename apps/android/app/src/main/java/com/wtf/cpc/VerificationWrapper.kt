package com.wtf.cpc
import com.wtf.cpc.ffi.VerificationNative

/**
 * Wrapper for Rust verification functions that provides a simpler Kotlin-friendly API.
 */
object VerificationWrapper {
    fun verifyReportSignature(reportJson: String, publicKey: String): Boolean {
        val resultPtr = VerificationNative.verify_impact_report_signature(reportJson, publicKey)
        try {
            return when (VerificationNative.get_verification_result_type(resultPtr)) {
                0 -> true  // Valid result
                else -> {
                    val error = VerificationNative.get_verification_result_error(resultPtr)
                    android.util.Log.e("Verification", "Report verification failed: $error")
                    false
                }
            }
        } finally {
            VerificationNative.free_verification_result(resultPtr)
        }
    }
}