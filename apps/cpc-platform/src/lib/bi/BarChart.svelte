<script>
  import { onMount } from 'svelte';
  
  export let data = [];
  export let height = 300;
  
  let canvas;
  let ctx;
  
  const colors = [
    '#4CAF50', '#2196F3', '#FFC107', '#9C27B0', '#E91E63',
    '#00BCD4', '#FF9800', '#795548', '#607D8B', '#8BC34A'
  ];
  
  onMount(() => {
    if (canvas && data.length > 0) {
      drawChart();
    }
  });
  
  $: if (ctx && data.length > 0) {
    drawChart();
  }
  
  function drawChart() {
    if (!ctx) return;
    
    const width = canvas.width;
    const height = canvas.height;
    const padding = 60;
    const chartWidth = width - padding * 2;
    const chartHeight = height - padding * 2;
    
    ctx.clearRect(0, 0, width, height);
    
    const values = data.map(d => d.impactScore || d.amount || 0);
    const maxValue = Math.max(...values);
    const minValue = Math.min(...values);
    const range = maxValue - minValue || 1;
    
    const barWidth = chartWidth / data.length * 0.8;
    const barSpacing = chartWidth / data.length * 0.2;
    
    // Draw axes
    ctx.strokeStyle = '#333';
    ctx.lineWidth = 2;
    
    // Y-axis
    ctx.beginPath();
    ctx.moveTo(padding, padding);
    ctx.lineTo(padding, height - padding);
    ctx.stroke();
    
    // X-axis
    ctx.beginPath();
    ctx.moveTo(padding, height - padding);
    ctx.lineTo(width - padding, height - padding);
    ctx.stroke();
    
    // Draw bars
    data.forEach((item, index) => {
      const value = item.impactScore || item.amount || 0;
      const barHeight = (value / maxValue) * chartHeight;
      const x = padding + index * (barWidth + barSpacing) + barSpacing / 2;
      const y = height - padding - barHeight;
      
      // Draw bar
      ctx.fillStyle = colors[index % colors.length];
      ctx.fillRect(x, y, barWidth, barHeight);
      
      // Draw value label
      ctx.fillStyle = '#333';
      ctx.font = '12px Arial';
      ctx.textAlign = 'center';
      ctx.fillText(value.toFixed(1), x + barWidth / 2, y - 5);
      
      // Draw category label
      const label = item.category || item.name || `Item ${index + 1}`;
      ctx.save();
      ctx.translate(x + barWidth / 2, height - padding + 15);
      ctx.rotate(-Math.PI / 6);
      ctx.textAlign = 'right';
      ctx.fillText(label, 0, 0);
      ctx.restore();
    });
    
    // Draw y-axis labels
    ctx.fillStyle = '#666';
    ctx.font = '12px Arial';
    ctx.textAlign = 'right';
    for (let i = 0; i <= 5; i++) {
      const value = (maxValue / 5) * i;
      const y = height - padding - (chartHeight / 5) * i;
      ctx.fillText(value.toFixed(1), padding - 10, y + 4);
    }
  }
  
  function handleResize() {
    if (canvas) {
      const rect = canvas.getBoundingClientRect();
      canvas.width = rect.width * window.devicePixelRatio;
      canvas.height = height * window.devicePixelRatio;
      ctx = canvas.getContext('2d');
      ctx.scale(window.devicePixelRatio, window.devicePixelRatio);
      
      if (data.length > 0) {
        drawChart();
      }
    }
  }
  
  onMount(() => {
    if (canvas) {
      handleResize();
      window.addEventListener('resize', handleResize);
    }
    
    return () => {
      window.removeEventListener('resize', handleResize);
    };
  });
</script>

<div class="bar-chart" style="height: {height}px;">
  <canvas bind:this={canvas}></canvas>
</div>

<style>
  .bar-chart {
    position: relative;
    width: 100%;
  }
  
  canvas {
    width: 100%;
    height: 100%;
    display: block;
  }
</style>