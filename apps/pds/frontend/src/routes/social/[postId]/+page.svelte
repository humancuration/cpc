<script>
  import PostDetail from '$lib/social/PostDetail.svelte';
  import { page } from '$app/stores';
</script>

<svelte:head>
  <title>Post Details</title>
</svelte:head>

<PostDetail postId={$page.params.postId} />