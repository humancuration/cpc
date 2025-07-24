<script>
  import { createEventDispatcher } from 'svelte';
  
  export let content = '';
  export let placeholder = "What's on your mind?";
  export let maxLength = 2000;
  export let disabled = false;
  
  const dispatch = createEventDispatcher();
  
  let textarea;
  let isBold = false;
  let isItalic = false;
  
  $: characterCount = content.length;
  
  function handleInput(event) {
    content = event.target.value;
    dispatch('change', { content });
  }
  
  function handleKeyDown(event) {
    // Handle keyboard shortcuts
    if (event.ctrlKey || event.metaKey) {
      switch (event.key) {
        case 'b':
          event.preventDefault();
          toggleFormat('bold');
          break;
        case 'i':
          event.preventDefault();
          toggleFormat('italic');
          break;
      }
    }
  }
  
  function toggleFormat(format) {
    if (format === 'bold') {
      isBold = !isBold;
    } else if (format === 'italic') {
      isItalic = !isItalic;
    }
  }
  
  function insertLink() {
    const url = prompt('Enter URL:');
    if (url) {
      const selectionStart = textarea.selectionStart;
      const selectionEnd = textarea.selectionEnd;
      const selectedText = content.substring(selectionStart, selectionEnd);
      
      const linkText = selectedText || url;
      const linkMarkdown = `[${linkText}](${url})`;
      
      content = content.substring(0, selectionStart) + 
                linkMarkdown + 
                content.substring(selectionEnd);
      
      dispatch('change', { content });
      
      // Reset cursor position
      setTimeout(() => {
        textarea.focus();
        textarea.setSelectionRange(
          selectionStart + linkMarkdown.length,
          selectionStart + linkMarkdown.length
        );
      }, 0);
    }
  }
  
  export function clear() {
    content = '';
    dispatch('change', { content });
  }
  
  export function insertText(text) {
    const start = textarea.selectionStart;
    const end = textarea.selectionEnd;
    
    content = content.substring(0, start) + text + content.substring(end);
    dispatch('change', { content });
    
    // Reset cursor position
    setTimeout(() => {
      textarea.focus();
      textarea.setSelectionRange(start + text.length, start + text.length);
    }, 0);
  }
</script>

<div class="rich-text-editor">
  <div class="toolbar">
    <button
      type="button"
      class="toolbar-btn"
      class:active={isBold}
      on:click={() => toggleFormat('bold')}
      title="Bold (Ctrl+B)"
      disabled={disabled}
    >
      <strong>B</strong>
    </button>
    <button
      type="button"
      class="toolbar-btn"
      class:active={isItalic}
      on:click={() => toggleFormat('italic')}
      title="Italic (Ctrl+I)"
      disabled={disabled}
    >
      <em>I</em>
    </button>
    <button
      type="button"
      class="toolbar-btn"
      on:click={insertLink}
      title="Insert link"
      disabled={disabled}
    >
      🔗
    </button>
  </div>
  
  <textarea
    bind:this={textarea}
    class="editor"
    bind:value={content}
    placeholder={placeholder}
    maxlength={maxLength}
    rows={4}
    {disabled}
    on:input={handleInput}
    on:keydown={handleKeyDown}
  />
  
  <div class="editor-info">
    <span class="format-hint">
      Markdown supported • **bold** • *italic* • [link](url)
    </span>
  </div>
</div>

<style>
  .rich-text-editor {
    border: 1px solid #ddd;
    border-radius: 8px;
    overflow: hidden;
  }
  
  .toolbar {
    display: flex;
    gap: 1px;
    background: #f8f9fa;
    border-bottom: 1px solid #ddd;
    padding: 4px;
  }
  
  .toolbar-btn {
    background: none;
    border: none;
    padding: 6px 8px;
    cursor: pointer;
    border-radius: 4px;
    font-size: 14px;
    transition: all 0.2s;
  }
  
  .toolbar-btn:hover:not(:disabled) {
    background: #e9ecef;
  }
  
  .toolbar-btn.active {
    background: #007bff;
    color: white;
  }
  
  .toolbar-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
  
  .editor {
    width: 100%;
    border: none;
    padding: 12px;
    font-family: inherit;
    font-size: 14px;
    resize: vertical;
    min-height: 100px;
    outline: none;
  }
  
  .editor:focus {
    background: #f8f9fa;
  }
  
  .editor-info {
    padding: 4px 8px;
    font-size: 11px;
    color: #666;
    background: #f8f9fa;
    border-top: 1px solid #eee;
  }
  
  .format-hint {
    font-family: monospace;
  }
</style>