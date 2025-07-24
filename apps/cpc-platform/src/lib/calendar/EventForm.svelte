<script>
  import { onMount } from 'svelte';
  import CalendarService from './CalendarService';
  import { showToast } from '../stores/toast';
  import { currentUser } from '../stores/auth';
  
  export let event = null;
  export let onSubmit;
  
  let title = '';
  let description = '';
  let startTime = '';
  let endTime = '';
  let location = '';
  let trainingDocId = '';
  let attendees = [];
  let availableMembers = [];
  
  onMount(async () => {
    // Fetch available members for attendee selection
    // This would come from the membership database
    // Placeholder - in real implementation we'd fetch from API
    availableMembers = [
      { id: '1', name: "Member 1" },
      { id: '2', name: "Member 2" },
      { id: '3', name: "Member 3" }
    ];
  });
  
  $: if (event) {
    title = event.title;
    description = event.description;
    startTime = event.start_time.toISOString().slice(0, 16);
    endTime = event.end_time.toISOString().slice(0, 16);
    location = event.location || '';
    trainingDocId = event.training_doc_id || '';
    attendees = [...event.attendees];
  }
  
  const handleSubmit = async () => {
    const $user = currentUser.get();
    if (!$user) {
        showToast.set({ message: "You must be logged in", type: "error" });
        return;
    }

    const eventData = {
        title: title,
        description: description,
        start_time: new Date(startTime),
        end_time: new Date(endTime),
        location: location || null,
        attendees: attendees,
        training_doc_id: trainingDocId || null
    };
    
    try {
      let result;
      if (event) {
        result = await CalendarService.updateEvent(
          event.id,
          eventData,
          $user.id
        );
      } else {
        result = await CalendarService.createEvent(
          eventData,
          $user.id
        );
      }
      showToast.set({ message: 'Event saved successfully', type: 'success' });
      onSubmit(result);
    } catch (error) {
      showToast.set({ message: `Error: ${error.message}`, type: 'error' });
    }
  };
</script>

<form on:submit|preventDefault={handleSubmit}>
  <div class="form-group">
    <label for="title">Title *</label>
    <input type="text" id="title" bind:value={title} required />
  </div>
  
  <div class="form-group">
    <label for="description">Description</label>
    <textarea id="description" bind:value={description} rows="4" />
  </div>
  
  <div class="form-row">
    <div class="form-group">
      <label for="startTime">Start Time *</label>
      <input type="datetime-local" id="startTime" bind:value={startTime} required />
    </div>
    
    <div class="form-group">
      <label for="endTime">End Time *</label>
      <input type="datetime-local" id="endTime" bind:value={endTime} required />
    </div>
  </div>
  
  <div class="form-group">
    <label for="location">Location</label>
    <input type="text" id="location" bind:value={location} />
  </div>
  
  <div class="form-group">
    <label for="trainingDocId">Training Document ID</label>
    <input type="text" id="trainingDocId" bind:value={trainingDocId} />
    <small>Reference to training document in our system</small>
  </div>
  
  <div class="form-group">
    <label>Attendees</label>
    {#each availableMembers as member}
      <div class="checkbox-item">
        <input
          type="checkbox"
          id={member.id}
          bind:group={attendees}
          value={member.id}
        />
        <label for={member.id}>{member.name}</label>
      </div>
    {/each}
  </div>
  
  <button type="submit" class="submit-btn">Save Event</button>
</form>

<style>
  .form-group {
    margin-bottom: 20px;
  }
  
  .form-row {
    display: flex;
    gap: 20px;
  }
  
  .form-row .form-group {
    flex: 1;
  }
  
  label {
    display: block;
    margin-bottom: 5px;
    font-weight: bold;
  }
  
  input[type="text"],
  input[type="datetime-local"],
  textarea {
    width: 100%;
    padding: 8px;
    border: 1px solid #ccc;
    border-radius: 4px;
  }
  
  .checkbox-item {
    margin: 5px 0;
  }
  
  .submit-btn {
    background-color: #4CAF50;
    color: white;
    padding: 10px 20px;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-size: 16px;
  }
  
  .submit-btn:hover {
    background-color: #45a049;
  }
  
  small {
    display: block;
    font-size: 0.8em;
    color: #666;
    margin-top: 4px;
  }
</style>