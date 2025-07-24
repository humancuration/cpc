<script>
  import PieChart from './PieChart.svelte';
  import BarChart from './BarChart.svelte';
  import LineChart from './LineChart.svelte';
  
  export let type = 'pie';
  export let title = '';
  export let data = [];
  export let height = 300;
  export let width = '100%';
  
  const chartComponents = {
    pie: PieChart,
    bar: BarChart,
    line: LineChart
  };
  
  $: ChartComponent = chartComponents[type] || PieChart;
</script>

<div class="chart-widget" style="width: {width};">
  {#if title}
    <h3 class="chart-title">{title}</h3>
  {/if}
  
  <div class="chart-container">
    {#if data && data.length > 0}
      <svelte:component this={ChartComponent} {data} {height} />
    {:else}
      <div class="empty-state">
        <p>No data available</p>
      </div>
    {/if}
  </div>
</div>

<style>
  .chart-widget {
    background: white;
    border-radius: 8px;
    padding: 1.5rem;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
    border: 1px solid #e0e0e0;
  }

  .chart-title {
    font-size: 1.125rem;
    font-weight: 600;
    margin-bottom: 1rem;
    color: #1a1a1a;
  }

  .chart-container {
    position: relative;
    width: 100%;
  }

  .empty-state {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 200px;
    color: #666;
    font-style: italic;
  }

  @media (max-width: 768px) {
    .chart-widget {
      padding: 1rem;
    }
  }
</style>