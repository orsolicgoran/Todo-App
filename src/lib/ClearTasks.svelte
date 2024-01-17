<script lang="ts">
  import { todo_list } from "./Store";

  let todos: Array<{ id: number; text: string }> = [];

  $: {
    todos = $todo_list;
  }

  function clear_tasks(event: MouseEvent) {
    $todo_list = [];
  }
</script>

<div class="clear-tasks">
  <button class="clear-button" on:click={clear_tasks}>Clear Tasks</button>
</div>

<style>
  .clear-button {
    background-color: #2f2f2f;
    color: #f6f6f6;
    border: none;
    padding: 10px 20px;
    text-align: center;
    text-decoration: none;
    display: inline-block;
    font-size: 16px;
    margin: 4px 2px;
    cursor: pointer;
    transition: background-color 0.3s ease;
  }

  .clear-button:hover {
    background-color: #f6f6f6; /* Light background for hover effect */
    color: #2f2f2f; /* Dark text to contrast with the light background */
  }
</style>
