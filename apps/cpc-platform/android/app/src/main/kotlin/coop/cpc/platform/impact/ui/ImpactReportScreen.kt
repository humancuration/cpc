package coop.cpc.platform.impact.ui

import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.lifecycle.viewmodel.compose.viewModel
import coop.cpc.platform.impact.viewmodel.ImpactReportViewModel
import java.util.UUID

@Composable
fun ImpactReportScreen(userId: UUID) {
    val viewModel: ImpactReportViewModel = viewModel()
    val uiState by viewModel.uiState.collectAsState()
    
    when (val state = uiState) {
        is ImpactReportViewModel.UiState.Loading -> LoadingView()
        is ImpactReportViewModel.UiState.Success -> ReportContentView(report = state.report)
        is ImpactReportViewModel.UiState.Error -> ErrorView(message = state.message)
    }
}

@Composable
fun LoadingView() {
    // Show loading indicator
}

@Composable
fun ReportContentView(report: ImpactReport) {
    // Show impact report content using DistributionView, TimelineView, BreakdownView
}

@Composable
fun ErrorView(message: String) {
    // Show error message
}