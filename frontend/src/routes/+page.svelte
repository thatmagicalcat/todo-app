<script lang="ts">
  import { onMount } from "svelte";
  let loading = true
  let error: string | undefined 
  let todos: {
    name: string;
    checked: boolean;
  }[] = []
  onMount(() => {
    fetch("http://127.0.0.1:8888/todos")
      .then((res) => res.json())
      .then((data) => {
        todos = data
        loading = false
      })
      .catch((e) => {
        loading = false
        error = e.toString()
      });
  });
</script>
{#if loading}
  <h1>Loading....</h1>
{/if}
{#if error}
  <h1>{error}</h1>
{/if}
{#each todos as todo }
  {#if todo.checked}
  		<h1>{todo.name}</h1>
  {:else}
    <h1><s>{todo.name}</s></h1>
  {/if}
{/each}