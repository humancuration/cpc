package com.wtf.cpc.report

import com.wtf.cpc.ImpactVerifier
import android.util.Log
import com.wtf.cpc.network.ApiException

class ReportHandler(private val verifier: ImpactVerifier) {

    fun processReport(reportJson: String, publicKey: String): Boolean {
        return try {
            val isValid = verifier.verifyReportSignature(reportJson, publicKey)
            if (!isValid) {
                Log.w("ReportHandler", "Invalid report signature")
            }
            isValid
        } catch (e: Exception) {
            Log.e("ReportHandler", "Report processing error", e)
            false
        }
    }
}