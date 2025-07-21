<script>
  import { onMount, afterUpdate } from 'svelte';
  import { Chart, PieController, ArcElement, Tooltip, Legend } from 'chart.js';
  import { Pie } from 'svelte-chartjs';

  Chart.register(PieController, ArcElement, Tooltip, Legend);

  export let breakdown = {};

  let chart;
  let chartElement;

  // Initialize chart when component mounts
  onMount(() => {
    if (chartElement) {
      chart = new Chart(chartElement, {
        type: 'pie',
        data: {
          labels: breakdown.labels || [],
          datasets: [{
            data: breakdown.values || [],
            backgroundColor: [
              '#FF6384', '#36A2EB', '#FFCE56', '#4BC0C0', 
              '#9966FF', '#FF9F40', '#8AC926', '#1982C4'
            ]
          }]
        },
        options: {
          responsive: true,
          plugins: {
            legend: {
              position: 'right'
            },
            tooltip: {
              callbacks: {
                label: function(context) {
                  const value = context.raw || 0;
                  const gb = (value / 1e9).toFixed(2);
                  return `${context.label}: ${gb} GB`;
                }
              }
            }
          }
        }
      });
    }
  });

  // Update chart when data changes
  afterUpdate(() => {
    if (chart) {
      chart.data.labels = breakdown.labels || [];
      chart.data.datasets[0].data = breakdown.values || [];
      chart.update();
    }
  });
</script>

<div class="chart-container">
  <canvas bind:this={chartElement} />
</div>

<style>
  .chart-container {
    width: 100%;
    height: 300px;
    margin-top: 20px;
  }
</style>