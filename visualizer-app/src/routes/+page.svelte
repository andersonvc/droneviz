<h1>Welcome to SvelteKit</h1>
<h2>Page: oo</h2>
<p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>

<script>
    //import hello-wasm 

    import init, { greet } from '../../../droneviz/pkg';
    import { onMount } from 'svelte';
    
    onMount(async() => {
        await init();
        greet();
    });
</script>