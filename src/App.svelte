<script lang="ts">
  import Display from "./lib/Display.svelte";
  import AddTask from "./lib/AddTask.svelte";
  import RemoveTask from "./lib/RemoveTask.svelte";
  import ClearTasks from "./lib/ClearTasks.svelte";
</script>

<main class="container">
  <div class="column left">
    <AddTask />
    <RemoveTask />
    <ClearTasks />
  </div>
  <div class="column right">
    <Display />
  </div>
</main>

<style>
</style>
