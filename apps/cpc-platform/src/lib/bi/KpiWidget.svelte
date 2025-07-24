<script>
  export let title = '';
  export let value = 0;
  export let format = 'number'; // 'number', 'currency', 'percentage'
  export let trend = null; // 'up', 'down', 'neutral'
  export let subtitle = '';
  
  function formatValue(val, format) {
    switch (format) {
      case 'currency':
        return new Intl.NumberFormat('en-US', {
          style: 'currency',
          currency: 'USD'
        }).format(val);
      case 'percentage':
        return `${(val * 100).toFixed(1)}%`;
      default:
        return val.toLocaleString();
    }
  }
  
  function getTrendIcon(trend) {
    switch (trend) {
      case 'up':
        return '↑';
      case 'down':
        return '↓';
      default:
        return '';
    }
  }
  
  function getTrendColor(trend) {
    switch (trend) {
      case 'up':
        return '#4CAF50';
      case 'down':
        return '#f44336';
      default:
        return '#666';
    }
  }
</script>

<div class="kpi-widget">
  <div class="kpi-header">
    <h3 class="kpi-title">{title}</h3>
    {#if trend}
      <span 
        class="kpi-trend" 
        style="color: {getTrendColor(trend)}"
        title="Trend: {trend}"
      >
        {getTrendIcon(trend)}
      </span>
    {/if}
  </div>
  
  <div class="kpi-value">
    {formatValue(value, format)}
  </div>
  
  {#if subtitle}
    <div class="kpi-subtitle">{subtitle}</div>
  {/if}
</div>

<style>
  .kpi-widget {
    background: white;
    border-radius: 8px;
    padding: 1.5rem;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
    border: 1px solid #e0e0e0;
    text-align: center;
  }

  .kpi-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1rem;
  }

  .kpi-title {
    font-size: 0.875rem;
    font-weight: 600;
    color: #666;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .kpi-trend {
    font-size: 1.25rem;
    font-weight: bold;
  }

  .kpi-value {
    font-size: 2rem;
    font-weight: 700;
    color: #1a1a1a;
    margin-bottom: 0.5rem;
  }

  .kpi-subtitle {
    font-size: 0.875rem;
    color: #666;
  }

  @media (max-width: 768px) {
    .kpi-widget {
      padding: 1rem;
    }

    .kpi-value {
      font-size: 1.5rem;
    }
  }
</style>