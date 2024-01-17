<script lang="ts">
  import { invoke } from "@tauri-apps/api";
  import { todo_list } from "./Store";
  import { onMount } from "svelte";

  onMount(async () => {
    const tasks: Array<{ id: number; text: string }> =
      await invoke("read_from_file");
    todo_list.set(tasks);
  });

  let todos: Array<{ id: number; text: string }> = [];

  $: {
    todos = $todo_list;
    if (todos.length > 0) {
      invoke("write_to_file", { tasks: JSON.stringify(todos) });
    }
  }
</script>

<div class="display">
  <ul>
    {#each todos as todo (todo.id)}
      <li>ID: {todo.id} | {todo.text}</li>
    {/each}
  </ul>
</div>
