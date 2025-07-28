package com.wtf.cpc.ffi

import com.sun.jna.Pointer

object VerificationNative {
    init {
        System.loadLibrary("cpc_core")
    }

    /**
     * Verifies an impact report signature
     * @return Pointer to VerificationResult
     */
    external fun verify_impact_report_signature(reportJson: String, publicKey: String): Pointer

    /**
     * @return 0 for Valid, 1 for Invalid
     */
    external fun get_verification_result_type(resultPtr: Pointer): Int

    /**
     * @return Error message or null if valid
     */
    external fun get_verification_result_error(resultPtr: Pointer): String?

    /**
     * Free verification result memory
     */
    external fun free_verification_result(resultPtr: Pointer)
}