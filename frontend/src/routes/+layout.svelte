<script>
    import Header from '$lib/components/Header.svelte';
    import Footer from '$lib/components/Footer.svelte';
    import './main.scss';

</script>

<Header/>
<main class="container mt-5">
    <slot/>
</main>
<Footer/>
