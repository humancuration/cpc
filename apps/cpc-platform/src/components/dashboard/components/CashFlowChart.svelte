<script>
  import { onMount } from 'svelte';
  import { Chart } from 'chart.js/auto';
  
  export let projections = [];
  export let selectedScenario = null;
  
  let canvas;
  let chart;
  
  onMount(() => {
    if (canvas && projections.length > 0) {
      createChart();
    }
    
    return () => {
      if (chart) {
        chart.destroy();
      }
    };
  });
  
  $: if (chart && projections.length > 0) {
    updateChart();
  }
  
  function createChart() {
    const ctx = canvas.getContext('2d');
    
    chart = new Chart(ctx, {
      type: 'line',
      data: {
        labels: projections.map(p => p.period),
        datasets: [
          {
            label: 'Income',
            data: projections.map(p => p.income),
            borderColor: '#27ae60',
            backgroundColor: 'rgba(39, 174, 96, 0.1)',
            tension: 0.4,
            fill: true
          },
          {
            label: 'Expenses',
            data: projections.map(p => p.expenses),
            borderColor: '#e74c3c',
            backgroundColor: 'rgba(231, 76, 60, 0.1)',
            tension: 0.4,
            fill: true
          },
          {
            label: 'Net Cash Flow',
            data: projections.map(p => p.net_cash_flow),
            borderColor: '#3498db',
            backgroundColor: 'rgba(52, 152, 219, 0.1)',
            tension: 0.4,
            fill: true
          }
        ]
      },
      options: {
        responsive: true,
        maintainAspectRatio: false,
        plugins: {
          legend: {
            position: 'top',
          },
          title: {
            display: true,
            text: 'Cash Flow Projections'
          }
        },
        scales: {
          y: {
            beginAtZero: true,
            ticks: {
              callback: function(value) {
                return '$' + value.toLocaleString();
              }
            }
          }
        }
      }
    });
  }
  
  function updateChart() {
    if (chart) {
      chart.data.labels = projections.map(p => p.period);
      chart.data.datasets[0].data = projections.map(p => p.income);
      chart.data.datasets[1].data = projections.map(p => p.expenses);
      chart.data.datasets[2].data = projections.map(p => p.net_cash_flow);
      chart.update();
    }
  }
</script>

<div class="cash-flow-chart">
  <canvas bind:this={canvas}></canvas>
</div>

<style>
  .cash-flow-chart {
    position: relative;
    height: 400px;
    width: 100%;
  }
  
  canvas {
    max-width: 100%;
  }
</style>