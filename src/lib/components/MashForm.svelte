<script lang="ts">
	import { addMashWithAI } from '$lib/stores/mashes.svelte';
	import { t } from '$lib/i18n/index.svelte';

	let text = $state('');
	let analyzing = $state(false);

	async function handleSubmit(e: Event) {
		e.preventDefault();
		if (!text.trim() || analyzing) return;

		analyzing = true;
		try {
			await addMashWithAI(text.trim());
			text = '';
		} catch {
			// toast already shown by store — keep form values
		} finally {
			analyzing = false;
		}
	}

	let canSubmit = $derived(text.trim().length > 0 && !analyzing);
</script>

<form class="flex flex-col gap-4" onsubmit={handleSubmit}>
	<textarea
		class="textarea w-full bg-white/[0.12] border-white/[0.18] focus:border-primary placeholder:text-base-content/30"
		placeholder={t('form.text')}
		bind:value={text}
		rows="4"
		disabled={analyzing}
	></textarea>

	<div class="flex justify-end">
		<button type="submit" class="btn btn-primary btn-sm" disabled={!canSubmit}>
			{#if analyzing}
				<span class="loading loading-spinner loading-xs"></span>
				{t('form.analyzing')}
			{:else}
				{t('form.submit')}
			{/if}
		</button>
	</div>
</form>
