<script lang="ts">
	import type { MashType } from '$lib/types';
	import { addMash } from '$lib/stores/mashes.svelte';
	import { t } from '$lib/i18n/index.svelte';
	import type { MessageKey } from '$lib/i18n/index.svelte';

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

	async function handleSubmit(e: Event) {
		e.preventDefault();
		if (!selectedType || !summary.trim()) return;

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
		} catch {
			// toast already shown by store — keep form values
		}
	}

	let canSubmit = $derived(selectedType !== null && summary.trim().length > 0);
</script>

<form class="flex flex-col gap-4" onsubmit={handleSubmit}>
	<div class="flex gap-2">
		{#each mashTypes as mt}
			<button
				type="button"
				class="btn btn-outline btn-sm flex-1 {mt.btnClass}"
				class:btn-active={selectedType === mt.type}
				onclick={() => (selectedType = selectedType === mt.type ? null : mt.type)}
			>
				{t(mt.titleKey)}
			</button>
		{/each}
	</div>

	<input
		type="text"
		class="input w-full bg-white/[0.12] border-white/[0.18] focus:border-primary placeholder:text-base-content/30"
		placeholder={t('form.summary')}
		bind:value={summary}
	/>

	<textarea
		class="textarea w-full bg-white/[0.12] border-white/[0.18] focus:border-primary text-sm placeholder:text-base-content/30"
		placeholder={t('form.context')}
		bind:value={context}
		rows="2"
	></textarea>

	<textarea
		class="textarea w-full bg-white/[0.12] border-white/[0.18] focus:border-primary text-sm placeholder:text-base-content/30"
		placeholder={t('form.memo')}
		bind:value={memo}
		rows="2"
	></textarea>

	<div class="flex justify-end">
		<button type="submit" class="btn btn-primary btn-sm" disabled={!canSubmit}>
			{t('form.submit')}
		</button>
	</div>
</form>
