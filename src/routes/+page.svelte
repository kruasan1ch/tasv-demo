<script lang="ts">
    import { onMount } from "svelte";
    import { invoke } from '@tauri-apps/api';

    let todos:any[] = []
    onMount(() => {
        invoke('todos_list').then((t) => {
            todos = [... todos, t]
        })
    })
</script>


<h1>Welcome to SvelteKit</h1>
<p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>

{todos}
