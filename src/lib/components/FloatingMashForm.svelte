<script lang="ts">
	import type { MashType } from '$lib/types';
	import { addMash } from '$lib/stores/mashes.svelte';
	import { t } from '$lib/i18n/index.svelte';
	import type { MessageKey } from '$lib/i18n/index.svelte';
	import { invoke } from '@tauri-apps/api/core';

	const mashTypes: { type: MashType; titleKey: MessageKey; btnClass: string }[] = [
		{ type: '결정', titleKey: 'type.결정', btnClass: 'btn-warning' },
		{ type: '문제', titleKey: 'type.문제', btnClass: 'btn-error' },
		{ type: '인사이트', titleKey: 'type.인사이트', btnClass: 'btn-success' },
		{ type: '질문', titleKey: 'type.질문', btnClass: 'btn-info' }
	];

	let selectedType = $state<MashType | null>(null);
	let summary = $state('');
	let context = $state('');
	let memo = $state('');
	let saving = $state(false);

	let canSubmit = $derived(selectedType !== null && summary.trim().length > 0 && !saving);

	async function hide() {
		try {
			await invoke('hide_floating_memo');
		} catch {
			// ignore — window hide is best-effort
		}
	}

	async function handleSubmit(e: Event) {
		e.preventDefault();
		if (!selectedType || !summary.trim() || saving) return;

		saving = true;
		try {
			await addMash({
				type: selectedType,
				summary: summary.trim(),
				context: context.trim(),
				memo: memo.trim()
			});

			selectedType = null;
			summary = '';
			context = '';
			memo = '';

			await hide();
		} catch {
			// toast already shown by store — keep form values
		} finally {
			saving = false;
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
		<div class="flex gap-1">
			{#each mashTypes as mt}
				<button
					type="button"
					class="btn btn-outline btn-xs flex-1 {mt.btnClass}"
					class:btn-active={selectedType === mt.type}
					onclick={() => (selectedType = selectedType === mt.type ? null : mt.type)}
				>
					{t(mt.titleKey)}
				</button>
			{/each}
		</div>

		<input
			type="text"
			class="input input-sm w-full bg-white/[0.12] border-white/[0.18] focus:border-primary placeholder:text-base-content/30"
			placeholder={t('form.summary')}
			bind:value={summary}
		/>

		<textarea
			class="textarea textarea-sm w-full bg-white/[0.12] border-white/[0.18] focus:border-primary text-sm placeholder:text-base-content/30"
			placeholder={t('form.context')}
			bind:value={context}
			rows="2"
		></textarea>

		<textarea
			class="textarea textarea-sm w-full bg-white/[0.12] border-white/[0.18] focus:border-primary text-sm placeholder:text-base-content/30"
			placeholder={t('form.memo')}
			bind:value={memo}
			rows="2"
		></textarea>

		<div class="flex justify-end gap-2">
			<button type="button" class="btn btn-ghost btn-xs" onclick={hide}>
				{t('common.cancel')}
			</button>
			<button type="submit" class="btn btn-primary btn-xs" disabled={!canSubmit}>
				{#if saving}
					<span class="loading loading-spinner loading-xs"></span>
				{:else}
					{t('form.submit')}
				{/if}
			</button>
		</div>
	</form>
</div>
