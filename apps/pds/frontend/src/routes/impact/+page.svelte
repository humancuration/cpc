<script>
  import ImpactReportScreen from '$lib/impact/ImpactReportScreen.svelte';
  
  // For now, we'll use a mock user ID
  // In a real app, this would come from auth context or route params
  const userId = 'test-user-123';
</script>

<svelte:head>
  <title>Impact Report - CPC Desktop</title>
  <meta name="description" content="View your ethical impact analysis and contribution metrics" />
</svelte:head>

<div class="impact-page">
  <ImpactReportScreen {userId} />
</div>

<style>
  .impact-page {
    min-height: 100vh;
    background-color: #f9fafb;
  }
</style>