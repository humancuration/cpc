<script>
  export let event;
  export let onEdit = () => {};
  export let onDelete = () => {};
</script>

<div class="event-card">
  <h3>{event.title}</h3>
  <p>{event.description}</p>
  <div class="event-time">
    <span>{event.start_time.format("%Y-%m-%d %H:%M")} to {event.end_time.format("%H:%M")}</span>
  </div>
  {#if event.location}
    <div class="event-location">
      <span>Location: {event.location}</span>
    </div>
  {/if}
  {#if event.training_doc_id}
    <div class="training-doc">
      <a href={`/training/docs/${event.training_doc_id}`} target="_blank">
        View Training Document
      </a>
    </div>
  {/if}
  <div class="event-actions">
    <button on:click={onEdit}>Edit</button>
    <button on:click={onDelete}>Delete</button>
  </div>
</div>

<style>
  .event-card {
    border: 1px solid #ccc;
    border-radius: 8px;
    padding: 16px;
    margin-bottom: 16px;
    background-color: white;
    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
  }
  
  .event-time {
    color: #666;
    font-size: 0.9em;
    margin: 8px 0;
  }
  
  .event-location, .training-doc {
    margin-bottom: 8px;
  }
  
  .training-doc a {
    color: #0066cc;
    text-decoration: none;
  }
  
  .training-doc a:hover {
    text-decoration: underline;
  }
  
  .event-actions {
    display: flex;
    gap: 8px;
    margin-top: 16px;
  }
  
  button {
    padding: 8px 16px;
    background-color: #f0f0f0;
    border: 1px solid #ccc;
    border-radius: 4px;
    cursor: pointer;
  }
  
  button:hover {
    background-color: #e0e0e0;
  }
</style>