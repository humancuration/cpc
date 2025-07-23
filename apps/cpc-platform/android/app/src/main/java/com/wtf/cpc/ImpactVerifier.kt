package com.wtf.cpc

import android.util.Log
import com.wtf.cpc.VerificationWrapper

class ImpactVerifier {
    fun verifyReportSignature(reportJson: String, publicKey: String): Boolean {
        return try {
            VerificationWrapper.verifyReportSignature(reportJson, publicKey)
        } catch (e: Exception) {
            Log.e("ImpactVerifier", "Verification error", e)
            false
        }
    }
}