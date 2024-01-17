<script lang="ts">
  import { todo_list } from "./Store";

  let todos: Array<{ id: number; text: string }> = [];
  let message: string = "";
  let id_to_remove: number;

  $: {
    todos = $todo_list;
  }

  function remove_task(event: MouseEvent) {
    const taskToRemove = $todo_list.find((todo) => todo.id === id_to_remove);
    if (taskToRemove) {
      $todo_list = $todo_list.filter((todo) => todo.id !== id_to_remove);
      message = "Task removed successfully!";
    } else {
      message = "Task not found!";
    }
    setTimeout(() => {
      message = "";
    }, 2000);
    id_to_remove = 0;
  }
</script>

<div class="remove-task">
  <input type="number" bind:value={id_to_remove} />
  <button class="remove-button" on:click={remove_task}>Remove Task</button>
  {#if message}
    <div class="message">{message}</div>
  {/if}
</div>

<style>
  .remove-task {
    display: flex;
    flex-direction: column;
    align-items: center;
  }

  input[type="number"] {
    background-color: #2f2f2f;
    color: #f6f6f6;
    border: none;
    padding: 10px;
    font-size: 16px;
    margin: 4px 2px;
    transition: background-color 0.3s ease;
  }

  input[type="number"]:focus {
    background-color: #f6f6f6;
    color: #2f2f2f;
  }

  input[type="number"]::-webkit-inner-spin-button,
  input[type="number"]::-webkit-outer-spin-button {
    -webkit-appearance: none;
    margin: 0;
  }

  input[type="number"] {
    -moz-appearance: textfield;
    -webkit-appearance: none;
    appearance: none;
  }

  .remove-button {
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

  .remove-button:hover {
    background-color: #f6f6f6;
    color: #2f2f2f;
  }
</style>
