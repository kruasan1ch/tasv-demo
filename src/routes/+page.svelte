<script lang="ts">
    import { onMount } from "svelte";
    import { invoke } from '@tauri-apps/api';
    import { appDataDir } from '@tauri-apps/api/path';
    let appDataDirPath:any
    let todos:any[] = []
    onMount(() => {
        invoke('todos_list').then((t) => {
            todos = [... todos, t]
        })
        appDataDir().then(p => {
            appDataDirPath = p
        });
    })

</script>


<h1>Welcome to SvelteKit</h1>
<p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>

{todos}
{appDataDirPath}