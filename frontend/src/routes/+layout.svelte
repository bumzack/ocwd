<script lang="ts">
    import Header from '$lib/components/Header.svelte';
    import Footer from '$lib/components/Footer.svelte';
    import './main.scss';
    import { onMount, type Snippet } from 'svelte';

    let {children}: { children: Snippet } = $props();

    // for drop down menus
    onMount(async () => {
        import('bootstrap/dist/js/bootstrap.bundle.min.js');
    });

</script>

<Header/>

<main class="container-fluid mt-12">
    {@render children()}
</main>

<Footer/>
