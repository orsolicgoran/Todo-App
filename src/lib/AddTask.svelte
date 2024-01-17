<script lang="ts">
  import { todo_list } from "./Store";

  let new_task: string = "";
  let message: string = "";

  function add_task() {
    const task = {
      id: $todo_list.length + 1,
      text: new_task,
    };
    if (new_task === "") {
      message = "Please enter a task!";
    } else {
      $todo_list = [...$todo_list, task];
      new_task = "";
      console.log($todo_list);
    }
    setTimeout(() => {
      message = "";
    }, 2000);
  }
</script>

<div class="add-task">
  <input type="text" bind:value={new_task} />
  <button class="add-button" on:click={add_task}>Add Task</button>
  {#if message}
    <div class="message">{message}</div>
  {/if}
</div>

<style>
  .add-task {
    display: flex;
    flex-direction: column;
    align-items: center;
  }

  .add-task input[type="text"] {
    background-color: #2f2f2f;
    color: #f6f6f6;
    border: none;
    padding: 10px;
    font-size: 16px;
    margin: 4px 2px;
    transition: background-color 0.3s ease;
  }

  .add-task input[type="text"]:focus {
    background-color: #f6f6f6;
    color: #2f2f2f;
  }

  .add-button {
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

  .add-button:hover {
    background-color: #f6f6f6;
    color: #2f2f2f;
  }
</style>
