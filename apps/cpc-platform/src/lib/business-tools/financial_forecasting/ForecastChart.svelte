<script>
  import { LineChart, BarChart } from 'svelte-charts';
  import { onMount } from 'svelte';
  
  export let projections = [];
  
  let chartData = [];
  let chartLabels = [];
  let chartSeries = [
    { name: 'Inflow', color: '#10B981' },
    { name: 'Outflow', color: '#EF4444' },
    { name: 'Net Cash Flow', color: '#3B82F6' }
  ];
  let chartType = 'line';
  let dateRange = {
    start: projections[0]?.date || new Date(),
    end: projections[projections.length - 1]?.date || new Date()
  };
  
  onMount(() => {
    updateChartData();
  });
  
  function updateChartData() {
    // Filter projections by date range
    const filteredProjections = projections.filter(p =>
      p.date >= dateRange.start && p.date <= dateRange.end
    );
    
    chartLabels = filteredProjections.map(p => p.date.toISOString().split('T')[0]);
    chartData = [
      filteredProjections.map(p => p.inflow),
      filteredProjections.map(p => p.outflow),
      filteredProjections.map(p => p.net_cash_flow)
    ];
  }
  
  function updateDateRange() {
    updateChartData();
  }
  
  $: if (projections.length > 0) {
    dateRange.start = projections[0].date;
    dateRange.end = projections[projections.length - 1].date;
    updateChartData();
  }
</script>

<div class="forecast-chart">
  <div class="chart-header">
    <h3>Cash Flow Projections</h3>
    <div class="chart-controls">
      <select bind:value={chartType} on:change={() => analytics.track('analysis_run', { analysis_type: 'chart_change', chart_type: chartType })}>
        <option value="line">Line Chart</option>
        <option value="bar">Bar Chart</option>
        <option value="waterfall">Waterfall Chart</option>
      </select>
      <div class="date-range">
        <label>From: <input type="date" bind:value={dateRange.start} on:change={() => {
          updateDateRange();
          analytics.track('analysis_run', { analysis_type: 'date_range_change' });
        }} /></label>
        <label>To: <input type="date" bind:value={dateRange.end} on:change={() => {
          updateDateRange();
          analytics.track('analysis_run', { analysis_type: 'date_range_change' });
        }} /></label>
      </div>
    </div>
  </div>
  
  {#if chartType === 'line'}
    <LineChart {chartData} {chartLabels} {chartSeries} />
  {:else if chartType === 'bar'}
    <BarChart {chartData} {chartLabels} {chartSeries} />
  {:else}
    <!-- Waterfall chart would require custom implementation -->
    <BarChart {chartData} {chartLabels} {chartSeries} />
  {/if}
</div>

<style>
  .forecast-chart {
    margin-bottom: 20px;
    padding: 15px;
    background: white;
    border-radius: 8px;
    box-shadow: 0 1px 3px rgba(0,0,0,0.1);
  }
  
  .chart-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 15px;
  }
  
  .chart-controls {
    display: flex;
    gap: 15px;
    align-items: center;
  }
  
  .date-range {
    display: flex;
    gap: 10px;
  }
</style>