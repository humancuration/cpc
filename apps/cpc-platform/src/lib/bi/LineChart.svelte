<script>
  import { onMount } from 'svelte';
  
  export let data = [];
  export let height = 300;
  
  let canvas;
  let ctx;
  
  const colors = {
    line: '#2196F3',
    grid: '#e0e0e0',
    text: '#666'
  };
  
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
    
    // Sort data by timestamp
    const sortedData = [...data].sort((a, b) => a.timestamp - b.timestamp);
    const values = sortedData.map(d => d.impactValue || d.score || 0);
    const maxValue = Math.max(...values);
    const minValue = Math.min(...values);
    const range = maxValue - minValue || 1;
    
    // Draw grid
    ctx.strokeStyle = colors.grid;
    ctx.lineWidth = 1;
    
    // Horizontal grid lines
    for (let i = 0; i <= 5; i++) {
      const y = padding + (chartHeight / 5) * i;
      ctx.beginPath();
      ctx.moveTo(padding, y);
      ctx.lineTo(width - padding, y);
      ctx.stroke();
    }
    
    // Vertical grid lines
    for (let i = 0; i <= 5; i++) {
      const x = padding + (chartWidth / 5) * i;
      ctx.beginPath();
      ctx.moveTo(x, padding);
      ctx.lineTo(x, height - padding);
      ctx.stroke();
    }
    
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
    
    // Draw line
    if (sortedData.length > 1) {
      ctx.strokeStyle = colors.line;
      ctx.lineWidth = 3;
      ctx.beginPath();
      
      sortedData.forEach((item, index) => {
        const value = item.impactValue || item.score || 0;
        const x = padding + (index / (sortedData.length - 1)) * chartWidth;
        const y = height - padding - ((value - minValue) / range) * chartHeight;
        
        if (index === 0) {
          ctx.moveTo(x, y);
        } else {
          ctx.lineTo(x, y);
        }
      });
      
      ctx.stroke();
      
      // Draw points
      ctx.fillStyle = colors.line;
      sortedData.forEach((item, index) => {
        const value = item.impactValue || item.score || 0;
        const x = padding + (index / (sortedData.length - 1)) * chartWidth;
        const y = height - padding - ((value - minValue) / range) * chartHeight;
        
        ctx.beginPath();
        ctx.arc(x, y, 4, 0, 2 * Math.PI);
        ctx.fill();
      });
    }
    
    // Draw labels
    ctx.fillStyle = colors.text;
    ctx.font = '12px Arial';
    
    // Y-axis labels
    ctx.textAlign = 'right';
    for (let i = 0; i <= 5; i++) {
      const value = minValue + (range / 5) * i;
      const y = height - padding - (chartHeight / 5) * i;
      ctx.fillText(value.toFixed(1), padding - 10, y + 4);
    }
    
    // X-axis labels
    ctx.textAlign = 'center';
    ctx.save();
    sortedData.forEach((item, index) => {
      const x = padding + (index / (sortedData.length - 1)) * chartWidth;
      const label = new Date(item.date || item.timestamp).toLocaleDateString();
      
      ctx.save();
      ctx.translate(x, height - padding + 15);
      ctx.rotate(-Math.PI / 4);
      ctx.fillText(label, 0, 0);
      ctx.restore();
    });
    ctx.restore();
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

<div class="line-chart" style="height: {height}px;">
  <canvas bind:this={canvas}></canvas>
</div>

<style>
  .line-chart {
    position: relative;
    width: 100%;
  }
  
  canvas {
    width: 100%;
    height: 100%;
    display: block;
  }
</style>