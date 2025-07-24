<script>
  import { page } from '$app/stores';
  
  const navItems = [
    { path: '/social', label: 'Social Feed' },
    { path: '/bi-dashboard', label: 'BI Dashboard' }
  ];
</script>

<nav class="main-nav">
  <div class="nav-container">
    {#each navItems as item}
      <a 
        href={item.path}
        class="nav-link"
        class:active={$page.url.pathname === item.path}
        aria-current={$page.url.pathname === item.path ? 'page' : undefined}
      >
        {item.label}
      </a>
    {/each}
  </div>
</nav>

<style>
  .main-nav {
    background: white;
    border-bottom: 1px solid #e0e0e0;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  }
  
  .nav-container {
    display: flex;
    max-width: 1200px;
    margin: 0 auto;
    padding: 0 20px;
  }
  
  .nav-link {
    padding: 16px 24px;
    text-decoration: none;
    color: #666;
    font-weight: 500;
    transition: all 0.2s ease;
    border-bottom: 2px solid transparent;
  }
  
  .nav-link:hover {
    color: #007bff;
    background: #f8f9fa;
  }
  
  .nav-link.active {
    color: #007bff;
    border-bottom-color: #007bff;
  }
  
  @media (max-width: 768px) {
    .nav-container {
      padding: 0 10px;
    }
    
    .nav-link {
      padding: 12px 16px;
      font-size: 14px;
    }
  }
</style>