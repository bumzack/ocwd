<script lang="ts">
    import Header from '$lib/components/Header.svelte';
    import Footer from '$lib/components/Footer.svelte';
    import './main.scss';
    import type {Snippet} from "svelte";


    let {children}: { children: Snippet } = $props();

</script>

<Header/>
<main class="container mt-5">
    {@render children()}
</main>
<Footer/>
