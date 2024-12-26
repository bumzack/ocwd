<script lang="ts">
	import { onDestroy } from 'svelte';

	export let show = false;

	// Optional: Add event listener to prevent clicks
	let overlayEl;

	const handleClick = (event) => {
		event.preventDefault();
		event.stopPropagation();
	};

	// Function to add event listener
	function addEventListener() {
		if (overlayEl) {
			overlayEl.addEventListener('click', handleClick);
		}
	}

	// Function to remove event listener
	function removeEventListener() {
		if (overlayEl) {
			overlayEl.removeEventListener('click', handleClick);
		}
	}

	// Add event listener when show is true
	$: if (show) {
		addEventListener();
	} else {
		removeEventListener();
	}

	// Remove event listener on component destroy
	onDestroy(removeEventListener);
</script>

<div bind:this={overlayEl} class="overlay" style="display: {show ? 'block' : 'none'};">
	<div class="spinner-grow" role="status" style="width: 6rem; height: 6rem;">
		<span class="visually-hidden">Loading...</span>
	</div>
</div>

<style>
    .overlay {
        position: fixed;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        background: rgba(0, 0, 0, 0.5);
        z-index: 1000;
    }
</style>
