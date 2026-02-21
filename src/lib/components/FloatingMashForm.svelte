<script lang="ts">
	import { addMashWithAI } from '$lib/stores/mashes.svelte';
	import { t } from '$lib/i18n/index.svelte';
	import { invoke } from '@tauri-apps/api/core';

	let text = $state('');
	let analyzing = $state(false);

	let canSubmit = $derived(text.trim().length > 0 && !analyzing);

	async function hide() {
		try {
			await invoke('hide_floating_memo');
		} catch {
			// ignore — window hide is best-effort
		}
	}

	async function handleSubmit(e: Event) {
		e.preventDefault();
		if (!text.trim() || analyzing) return;

		analyzing = true;
		try {
			await addMashWithAI(text.trim());
			text = '';
			await hide();
		} catch {
			// toast already shown by store — keep form values
		} finally {
			analyzing = false;
		}
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Escape') {
			hide();
		}
	}
</script>

<svelte:window onkeydown={handleKeydown} />

<div class="flex flex-col h-full">
	<!-- Drag region -->
	<div
		class="flex items-center justify-between px-3 py-2 cursor-move select-none"
		data-tauri-drag-region
	>
		<span class="text-xs font-semibold text-base-content/70" data-tauri-drag-region>
			{t('floating.title')}
		</span>
		<button type="button" class="btn btn-ghost btn-xs" onclick={hide}>
			✕
		</button>
	</div>

	<!-- Form -->
	<form class="flex flex-col gap-2 px-3 pb-3 flex-1" onsubmit={handleSubmit}>
		<textarea
			class="textarea textarea-sm w-full flex-1 bg-base-content/[0.08] border-base-content/[0.12] focus:border-primary text-sm placeholder:text-base-content/45"
			placeholder={t('form.text')}
			bind:value={text}
			rows="5"
			disabled={analyzing}
		></textarea>

		<div class="flex justify-end gap-2">
			<button type="button" class="btn btn-ghost btn-xs" onclick={hide}>
				{t('common.cancel')}
			</button>
			<button type="submit" class="btn btn-primary btn-xs" disabled={!canSubmit}>
				{#if analyzing}
					<span class="loading loading-spinner loading-xs"></span>
					{t('form.analyzing')}
				{:else}
					{t('form.submit')}
				{/if}
			</button>
		</div>
	</form>
</div>
