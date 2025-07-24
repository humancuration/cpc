<script>
  import { onMount } from 'svelte';
  import { submitFeedback } from '$lib/graphql/userTestingMutations';
  
  let rating = 3;
  let comment = '';
  let contactEmail = '';
  let submitting = false;
  let submitError = null;
  let submitSuccess = false;
  
  async function handleSubmit() {
    submitting = true;
    submitError = null;
    
    try {
      await submitFeedback({
        rating,
        comment,
        contact_email: contactEmail
      });
      
      submitSuccess = true;
      // Reset form after successful submission
      setTimeout(() => {
        rating = 3;
        comment = '';
        contactEmail = '';
        submitSuccess = false;
      }, 2000);
    } catch (err) {
      submitError = err.message;
    }
    
    submitting = false;
  }
</script>

<div class="feedback-widget">
  <h3>Feedback</h3>
  {#if submitSuccess}
    <p class="success">Thank you for your feedback!</p>
  {:else}
    <form on:submit|preventDefault={handleSubmit}>
      <div class="form-group">
        <label>Rating:</label>
        <div class="rating">
          {#each [1,2,3,4,5] as num}
            <button 
              type="button"
              class:active={rating === num}
              on:click={() => rating = num}
            >
              ★
            </button>
          {/each}
        </div>
      </div>
      
      <div class="form-group">
        <label for="comment">Comments:</label>
        <textarea 
          id="comment" 
          bind:value={comment} 
          placeholder="What did you like or dislike about this feature?" 
          required
        />
      </div>
      
      <div class="form-group">
        <label for="email">Email (optional):</label>
        <input 
          type="email" 
          id="email" 
          bind:value={contactEmail} 
          placeholder="Enter if you'd like follow-up" 
        />
      </div>
      
      {#if submitError}
        <p class="error">{submitError}</p>
      {/if}
      
      <button type="submit" disabled={submitting}>
        {submitting ? 'Submitting...' : 'Submit Feedback'}
      </button>
    </form>
  {/if}
</div>

<style>
  .feedback-widget {
    padding: 15px;
    background: #f8f9fa;
    border-radius: 8px;
    border: 1px solid #e9ecef;
    margin-top: 20px;
  }
  
  .form-group {
    margin-bottom: 15px;
  }
  
  label {
    display: block;
    margin-bottom: 5px;
    font-weight: 500;
  }
  
  textarea, input {
    width: 100%;
    padding: 8px;
    border: 1px solid #ced4da;
    border-radius: 4px;
  }
  
  textarea {
    min-height: 100px;
  }
  
  .rating {
    display: flex;
    gap: 5px;
  }
  
  .rating button {
    background: none;
    border: none;
    font-size: 24px;
    cursor: pointer;
    color: #e9ecef;
    padding: 0;
  }
  
  .rating button.active {
    color: #ffc107;
  }
  
  button[type="submit"] {
    background: #3b82f6;
    color: white;
    border: none;
    padding: 8px 15px;
    border-radius: 4px;
    cursor: pointer;
  }
  
  button[type="submit"]:disabled {
    opacity: 0.7;
    cursor: not-allowed;
  }
  
  .error {
    color: #ef4444;
    margin-bottom: 10px;
  }
  
  .success {
    color: #10b981;
  }
</style>