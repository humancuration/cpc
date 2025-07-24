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
    const centerX = width / 2;
    const centerY = height / 2;
    const radius = Math.min(width, height) / 2 - 40;
    
    ctx.clearRect(0, 0, width, height);
    
    const total = data.reduce((sum, item) => sum + (item.weight || item.amount || 0), 0);
    let currentAngle = -Math.PI / 2;
    
    data.forEach((item, index) => {
      const value = item.weight || item.amount || 0;
      const sliceAngle = (value / total) * 2 * Math.PI;
      
      // Draw slice
      ctx.beginPath();
      ctx.moveTo(centerX, centerY);
      ctx.arc(centerX, centerY, radius, currentAngle, currentAngle + sliceAngle);
      ctx.closePath();
      ctx.fillStyle = colors[index % colors.length];
      ctx.fill();
      
      // Draw border
      ctx.strokeStyle = '#fff';
      ctx.lineWidth = 2;
      ctx.stroke();
      
      // Draw label
      const labelAngle = currentAngle + sliceAngle / 2;
      const labelX = centerX + Math.cos(labelAngle) * (radius * 0.7);
      const labelY = centerY + Math.sin(labelAngle) * (radius * 0.7);
      
      ctx.fillStyle = '#fff';
      ctx.font = 'bold 12px Arial';
      ctx.textAlign = 'center';
      ctx.textBaseline = 'middle';
      
      const percentage = ((value / total) * 100).toFixed(1);
      ctx.fillText(`${percentage}%`, labelX, labelY);
      
      currentAngle += sliceAngle;
    });
    
    // Draw legend
    const legendX = 10;
    let legendY = 10;
    
    data.forEach((item, index) => {
      const label = item.category || item.name || `Item ${index + 1}`;
      
      // Color box
      ctx.fillStyle = colors[index % colors.length];
      ctx.fillRect(legendX, legendY, 12, 12);
      
      // Label
      ctx.fillStyle = '#333';
      ctx.font = '12px Arial';
      ctx.textAlign = 'left';
      ctx.fillText(label, legendX + 20, legendY + 9);
      
      legendY += 20;
    });
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

<div class="pie-chart" style="height: {height}px;">
  <canvas bind:this={canvas}></canvas>
</div>

<style>
  .pie-chart {
    position: relative;
    width: 100%;
  }
  
  canvas {
    width: 100%;
    height: 100%;
    display: block;
  }
</style>