import { writable, derived } from 'svelte/store';
import { persistStore } from '$lib/stores/persistStore';

// Tool registry store
export const toolRegistry = writable({
  tools: [],
  categories: [],
  recentTools: [],
  searchQuery: '',
  loading: false,
  error: null
});

// Persist recent tools to localStorage
export const recentTools = persistStore('business-tools-recent', []);

// Tool metadata structure
// {
//   id: 'unique-identifier',
//   name: 'Tool Display Name',
//   description: 'Brief description of the tool',
//   icon: 'icon-name',
//   category: 'category-id',
//   component: SvelteComponent,
//   tags: ['tag1', 'tag2'],
//   permissions: ['permission1', 'permission2'],
//   isBeta: false,
//   version: '1.0.0',
//   lastUsed: Date,
//   usageCount: 0
// }

// Predefined tool categories
export const TOOL_CATEGORIES = {
  ACCOUNTING: {
    id: 'accounting',
    name: 'Accounting',
    icon: 'calculator',
    description: 'Financial management and accounting tools',
    color: '#10b981'
  },
  COOPERATIVE: {
    id: 'cooperative',
    name: 'Cooperative',
    icon: 'users',
    description: 'Cooperative governance and member management',
    color: '#8b5cf6'
  },
  INVENTORY: {
    id: 'inventory',
    name: 'Inventory',
    icon: 'package',
    description: 'Supply chain and inventory management',
    color: '#f59e0b'
  },
  REPORTING: {
    id: 'reporting',
    name: 'Reporting',
    icon: 'chart-bar',
    description: 'Business intelligence and analytics',
    color: '#3b82f6'
  },
  COLLABORATION: {
    id: 'collaboration',
    name: 'Collaboration',
    icon: 'users-group',
    description: 'Project management and team collaboration',
    color: '#ef4444'
  },
  COMPLIANCE: {
    id: 'compliance',
    name: 'Compliance',
    icon: 'shield-check',
    description: 'Regulatory compliance and audit tools',
    color: '#6366f1'
  }
};

// Default tools registry
const DEFAULT_TOOLS = [
  {
    id: 'accounting-dashboard',
    name: 'Accounting Dashboard',
    description: 'Overview of financial health and key metrics',
    icon: 'chart-pie',
    category: 'accounting',
    component: null, // Will be dynamically imported
    tags: ['finance', 'dashboard', 'overview'],
    permissions: ['accounting.read'],
    isBeta: false,
    version: '1.0.0'
  },
  {
    id: 'general-ledger',
    name: 'General Ledger',
    description: 'Complete record of all financial transactions',
    icon: 'book-open',
    category: 'accounting',
    component: null,
    tags: ['accounting', 'transactions', 'ledger'],
    permissions: ['accounting.read', 'accounting.write'],
    isBeta: false,
    version: '1.0.0'
  },
  {
    id: 'invoicing-dashboard',
    name: 'Invoicing Dashboard',
    description: 'Monitor invoices, revenue, and outstanding payments.',
    icon: 'document-chart-bar',
    category: 'accounting',
    component: null, // This will be the Yew component
    tags: ['invoicing', 'dashboard', 'finance'],
    permissions: ['invoicing.read'],
    isBeta: true,
    version: '0.1.0'
  },
  {
    id: 'member-directory',
    name: 'Member Directory',
    description: 'Manage cooperative member information and roles',
    icon: 'address-book',
    category: 'cooperative',
    component: null,
    tags: ['members', 'directory', 'roles'],
    permissions: ['cooperative.read'],
    isBeta: false,
    version: '1.0.0'
  },
  {
    id: 'inventory-manager',
    name: 'Inventory Manager',
    description: 'Track stock levels, suppliers, and procurement',
    icon: 'warehouse',
    category: 'inventory',
    component: null,
    tags: ['inventory', 'stock', 'suppliers'],
    permissions: ['inventory.read', 'inventory.write'],
    isBeta: false,
    version: '1.0.0'
  },
  {
    id: 'impact-reports',
    name: 'Impact Reports',
    description: 'Generate social and environmental impact reports',
    icon: 'leaf',
    category: 'reporting',
    component: null,
    tags: ['impact', 'reports', 'sustainability'],
    permissions: ['reporting.read'],
    isBeta: false,
    version: '1.0.0'
  },
  {
    id: 'project-tracker',
    name: 'Project Tracker',
    description: 'Manage projects, tasks, and team collaboration',
    icon: 'kanban',
    category: 'collaboration',
    component: null,
    tags: ['projects', 'tasks', 'collaboration'],
    permissions: ['collaboration.read', 'collaboration.write'],
    isBeta: true,
    version: '0.9.0'
  },
  {
    id: 'financial-forecasting',
    name: 'Financial Forecasting',
    description: 'Cash flow projections and scenario modeling',
    icon: 'chart-line',
    category: 'reporting',
    component: null,
    tags: ['finance', 'forecasting', 'projections'],
    permissions: ['manage_financial_forecasting'],
    isBeta: false,
    version: '1.0.0'
  }
];

// Initialize tool registry
export function initializeToolRegistry() {
  toolRegistry.update(state => ({
    ...state,
    tools: DEFAULT_TOOLS,
    categories: Object.values(TOOL_CATEGORIES),
    loading: false,
    error: null
  }));
}

// Add a new tool to the registry
export function registerTool(tool) {
  toolRegistry.update(state => ({
    ...state,
    tools: [...state.tools, { ...tool, id: generateToolId(tool.name) }]
  }));
}

// Remove a tool from the registry
export function unregisterTool(toolId) {
  toolRegistry.update(state => ({
    ...state,
    tools: state.tools.filter(tool => tool.id !== toolId)
  }));
}

// Update tool metadata
export function updateTool(toolId, updates) {
  toolRegistry.update(state => ({
    ...state,
    tools: state.tools.map(tool => 
      tool.id === toolId ? { ...tool, ...updates } : tool
    )
  }));
}

// Track tool usage
export function trackToolUsage(toolId) {
  const now = new Date();
  
  // Update in registry
  toolRegistry.update(state => ({
    ...state,
    tools: state.tools.map(tool => 
      tool.id === toolId 
        ? { ...tool, lastUsed: now, usageCount: (tool.usageCount || 0) + 1 }
        : tool
    )
  }));
  
  // Update recent tools
  recentTools.update(tools => {
    const filtered = tools.filter(id => id !== toolId);
    return [toolId, ...filtered].slice(0, 10); // Keep last 10
  });
}

// Search tools
export const searchTools = derived(
  [toolRegistry, recentTools],
  ([$registry, $recentTools]) => {
    const { tools, searchQuery } = $registry;
    
    if (!searchQuery.trim()) {
      return tools;
    }
    
    const query = searchQuery.toLowerCase();
    return tools.filter(tool => 
      tool.name.toLowerCase().includes(query) ||
      tool.description.toLowerCase().includes(query) ||
      tool.tags.some(tag => tag.toLowerCase().includes(query))
    );
  }
);

// Get tools by category
export const getToolsByCategory = derived(
  toolRegistry,
  $registry => (categoryId) => {
    return $registry.tools.filter(tool => tool.category === categoryId);
  }
);

// Get recent tools with full metadata
export const getRecentTools = derived(
  [toolRegistry, recentTools],
  ([$registry, $recentTools]) => {
    return $recentTools
      .map(toolId => $registry.tools.find(tool => tool.id === toolId))
      .filter(Boolean);
  }
);

// Get featured tools (most used or newest)
export const getFeaturedTools = derived(
  toolRegistry,
  $registry => {
    return $registry.tools
      .filter(tool => !tool.isBeta)
      .sort((a, b) => (b.usageCount || 0) - (a.usageCount || 0))
      .slice(0, 6);
  }
);

// Check if user has permission for a tool
export function hasToolPermission(tool, userPermissions) {
  if (!tool.permissions || tool.permissions.length === 0) {
    return true;
  }
  return tool.permissions.every(permission => userPermissions.includes(permission));
}

// Filter tools by user permissions
export const getAccessibleTools = derived(
  toolRegistry,
  $registry => (userPermissions = []) => {
    return $registry.tools.filter(tool => 
      hasToolPermission(tool, userPermissions)
    );
  }
);

// Utility functions
function generateToolId(name) {
  return name
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, '-')
    .replace(/^-+|-+$/g, '');
}

// Export for initialization
export const toolRegistryService = {
  initialize: initializeToolRegistry,
  register: registerTool,
  unregister: unregisterTool,
  update: updateTool,
  trackUsage: trackToolUsage
};