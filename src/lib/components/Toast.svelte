<script lang="ts">
	import { getToast, dismissToast } from '$lib/stores/toast.svelte';

	const toast = $derived(getToast());

	const alertClass = $derived(
		toast.type === 'error'
			? 'alert-error'
			: toast.type === 'success'
				? 'alert-success'
				: 'alert-info'
	);
</script>

{#if toast.visible}
	<div class="toast toast-center toast-bottom z-50">
		<button class="alert {alertClass} shadow-lg cursor-pointer text-sm py-2 px-4" onclick={dismissToast}>
			<span>{toast.message}</span>
		</button>
	</div>
{/if}
