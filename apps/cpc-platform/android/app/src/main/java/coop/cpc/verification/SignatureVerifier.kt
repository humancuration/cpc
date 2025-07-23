package coop.cpc.verification

class SignatureVerifier {
    external fun verify_impact_report_signature(
        reportJson: String,
        publicKey: String
    ): Long  // Returns pointer to VerificationResult

    external fun free_verification_result(ptr: Long)
    external fun get_verification_result_type(ptr: Long): Int
    external fun get_verification_result_error(ptr: Long): Long  // Returns pointer to error string (0 for none)
    external fun free_error_string(ptr: Long)  // To free error string
    external fun ptr_to_string(ptr: Long): Long  // Returns pointer to duplicated string

    fun toResult(ptr: Long): VerificationResult {
        return when (get_verification_result_type(ptr)) {
            0 -> VerificationResult.Valid
            1 -> {
                val errorPtr = get_verification_result_error(ptr)
                if (errorPtr == 0L) {
                    VerificationResult.Invalid("Unknown error")
                } else {
                    val dupPtr = ptr_to_string(errorPtr)  // Duplicate string
                    val error = dupPtr.toCPointer<ByteVar>()?.toKString() ?: "Unknown error"
                    free_error_string(dupPtr)  // Free duplicated string
                    free_error_string(errorPtr)  // Free original error string
                    VerificationResult.Invalid(error)
                }
            }
            else -> throw IllegalStateException("Invalid verification result")
        }
    }

    companion object {
        init {
            System.loadLibrary("cpc_core")
        }
    }
}

sealed class VerificationResult {
    object Valid : VerificationResult()
    data class Invalid(val error: String) : VerificationResult()
}