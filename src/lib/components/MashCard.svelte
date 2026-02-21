<script lang="ts">
	import type { Mash, MashType } from '$lib/types';
	import { MashStatus } from '$lib/types';
	import type { ViewMode } from '$lib/stores/settings.svelte';
	import { getDateFormat, getTimeFormat } from '$lib/stores/settings.svelte';
	import { deleteMash, setMashStatus, updateMash, loadMashes } from '$lib/stores/mashes.svelte';
	import { t } from '$lib/i18n/index.svelte';
	import type { MessageKey } from '$lib/i18n/index.svelte';
	import { formatDateTime } from '$lib/utils/datetime';

	let {
		mash,
		view = 'list',
		mode = 'mashes',
		currentStatus = ''
	}: {
		mash: Mash;
		view?: ViewMode;
		mode?: 'mashes' | 'still';
		currentStatus?: string;
	} = $props();

	const mashTypes: { type: MashType; titleKey: MessageKey; btnClass: string }[] = [
		{ type: '결정', titleKey: 'type.결정', btnClass: 'btn-warning' },
		{ type: '문제', titleKey: 'type.문제', btnClass: 'btn-error' },
		{ type: '인사이트', titleKey: 'type.인사이트', btnClass: 'btn-success' },
		{ type: '질문', titleKey: 'type.질문', btnClass: 'btn-info' }
	];

	async function handleSetStatus(id: string, status: MashStatus) {
		try {
			await setMashStatus(id, status);
			await loadMashes(currentStatus);
		} catch {
			// toast already shown by store
		}
	}

	async function handleDelete(id: string) {
		try {
			await deleteMash(id);
			await loadMashes(currentStatus);
		} catch {
			// toast already shown by store
		}
	}

	const typeBadgeClass: Record<MashType, string> = {
		'결정': 'badge-warning',
		'문제': 'badge-error',
		'인사이트': 'badge-success',
		'질문': 'badge-info'
	};

	const typeBorderClass: Record<MashType, string> = {
		'결정': 'border-l-warning',
		'문제': 'border-l-error',
		'인사이트': 'border-l-success',
		'질문': 'border-l-info'
	};

	const typeBgClass: Record<MashType, string> = {
		'결정': 'border-warning/30',
		'문제': 'border-error/30',
		'인사이트': 'border-success/30',
		'질문': 'border-info/30'
	};

	function typeLabel(type: MashType): string {
		return t(`type.${type}` as MessageKey);
	}

	let showConfirm = $state(false);

	// Edit mode
	let editing = $state(false);
	let editType = $state<MashType>('결정');
	let editSummary = $state('');
	let editContext = $state('');
	let editMemo = $state('');
	let saving = $state(false);

	function startEdit() {
		editType = mash.type;
		editSummary = mash.summary;
		editContext = mash.context;
		editMemo = mash.memo;
		editing = true;
	}

	function cancelEdit() {
		editing = false;
	}

	async function saveEdit() {
		if (!editSummary.trim()) return;
		saving = true;
		try {
			await updateMash(mash.id, {
				type: editType,
				summary: editSummary.trim(),
				context: editContext.trim(),
				memo: editMemo.trim()
			});
			editing = false;
			await loadMashes(currentStatus);
		} catch {
			// toast already shown by store
		} finally {
			saving = false;
		}
	}

	let canSave = $derived(editSummary.trim().length > 0);
</script>

{#if editing}
	<article class="p-4 border border-primary/40 rounded-lg bg-base-content/[0.04] flex flex-col gap-3">
		<div class="flex gap-2">
			{#each mashTypes as mt}
				<button
					type="button"
					class="btn btn-outline btn-xs flex-1 {mt.btnClass}"
					class:btn-active={editType === mt.type}
					onclick={() => (editType = mt.type)}
				>
					{t(mt.titleKey)}
				</button>
			{/each}
		</div>

		<input
			type="text"
			class="input input-sm w-full bg-base-content/[0.08] border-base-content/[0.12] focus:border-primary placeholder:text-base-content/45"
			placeholder={t('form.summary')}
			bind:value={editSummary}
		/>

		<textarea
			class="textarea textarea-sm w-full bg-base-content/[0.08] border-base-content/[0.12] focus:border-primary text-sm placeholder:text-base-content/45"
			placeholder={t('form.context')}
			bind:value={editContext}
			rows="2"
		></textarea>

		<textarea
			class="textarea textarea-sm w-full bg-base-content/[0.08] border-base-content/[0.12] focus:border-primary text-sm placeholder:text-base-content/45"
			placeholder={t('form.memo')}
			bind:value={editMemo}
			rows="2"
		></textarea>

		<div class="flex justify-end gap-2">
			<button class="btn btn-ghost btn-xs" onclick={cancelEdit} disabled={saving}>{t('common.cancel')}</button>
			<button class="btn btn-primary btn-xs" onclick={saveEdit} disabled={!canSave || saving}>
				{#if saving}
					<span class="loading loading-spinner loading-xs"></span>
				{:else}
					{t('common.save')}
				{/if}
			</button>
		</div>
	</article>
{:else if view === 'card'}
	<article class="p-4 border {typeBgClass[mash.type]} rounded-lg hover:bg-base-content/[0.06] transition-colors group flex flex-col gap-2">
		<div class="flex items-center justify-between">
			<span class="badge badge-sm badge-outline {typeBadgeClass[mash.type]}">{typeLabel(mash.type)}</span>
			<div class="flex items-center gap-1">
				{#if mode === 'mashes'}
					<button
						class="btn btn-ghost btn-xs text-base-content/40 hover:text-base-content/60"
						title={t('common.edit')}
						onclick={startEdit}
					>
						<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
							<path d="M11 4H4a2 2 0 00-2 2v14a2 2 0 002 2h14a2 2 0 002-2v-7"/>
							<path d="M18.5 2.5a2.121 2.121 0 013 3L12 15l-4 1 1-4 9.5-9.5z"/>
						</svg>
					</button>
					<button
						class="btn btn-ghost btn-xs text-base-content/40 hover:text-primary/70"
						title={t('card.putOnStill')}
						onclick={() => handleSetStatus(mash.id, MashStatus.ON_STILL)}
					>
						<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
							<circle cx="18" cy="5" r="3"/>
							<circle cx="6" cy="12" r="3"/>
							<circle cx="18" cy="19" r="3"/>
							<line x1="8.59" y1="13.51" x2="15.42" y2="17.49"/>
							<line x1="15.41" y1="6.51" x2="8.59" y2="10.49"/>
						</svg>
					</button>
				{:else if mode === 'still'}
					<button
						class="btn btn-ghost btn-xs text-base-content/45 hover:text-warning"
						title={t('card.takeOffStill')}
						onclick={() => handleSetStatus(mash.id, MashStatus.MASH_TUN)}
					>
						<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
							<path d="M9 21H5a2 2 0 01-2-2V5a2 2 0 012-2h4"/>
							<polyline points="16 17 21 12 16 7"/>
							<line x1="21" y1="12" x2="9" y2="12"/>
						</svg>
					</button>
				{/if}
				{#if showConfirm}
					<button class="btn btn-ghost btn-xs text-error" onclick={() => handleDelete(mash.id)}>{t('common.delete')}</button>
					<button class="btn btn-ghost btn-xs text-base-content/60" onclick={() => (showConfirm = false)}>{t('common.cancel')}</button>
				{:else}
					<button class="btn btn-ghost btn-xs text-base-content/45 hover:text-error" title={t('common.delete')} onclick={() => (showConfirm = true)}>
						<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
							<polyline points="3 6 5 6 21 6"/>
							<path d="M19 6v14a2 2 0 01-2 2H7a2 2 0 01-2-2V6m3 0V4a2 2 0 012-2h4a2 2 0 012 2v2"/>
						</svg>
					</button>
				{/if}
			</div>
		</div>

		<p class="text-[15px] font-medium leading-relaxed">{mash.summary}</p>

		{#if mash.context}
			<p class="text-sm text-base-content/65 leading-relaxed line-clamp-2">{mash.context}</p>
		{/if}

		{#if mash.memo}
			<p class="text-sm text-base-content/45 italic leading-relaxed line-clamp-1">{mash.memo}</p>
		{/if}

		<div class="flex items-center justify-between mt-auto pt-1">
			<span class="text-xs text-base-content/40">{formatDateTime(mash.createdAt, getDateFormat(), getTimeFormat())}</span>
		</div>
	</article>
{:else}
	<article class="p-4 border border-base-300 rounded-lg border-l-[3px] {typeBorderClass[mash.type]} hover:bg-base-content/[0.06] transition-colors group">
		<div class="flex items-center justify-between mb-2">
			<span class="badge badge-sm badge-outline {typeBadgeClass[mash.type]}">{typeLabel(mash.type)}</span>
			<div class="flex items-center gap-2">
				{#if mode === 'mashes'}
					<button
						class="btn btn-ghost btn-xs text-base-content/45 hover:text-base-content/60"
						title={t('common.edit')}
						onclick={startEdit}
					>
						<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
							<path d="M11 4H4a2 2 0 00-2 2v14a2 2 0 002 2h14a2 2 0 002-2v-7"/>
							<path d="M18.5 2.5a2.121 2.121 0 013 3L12 15l-4 1 1-4 9.5-9.5z"/>
						</svg>
					</button>
					<button
						class="btn btn-ghost btn-xs text-base-content/45 hover:text-primary/70"
						title={t('card.putOnStill')}
						onclick={() => handleSetStatus(mash.id, MashStatus.ON_STILL)}
					>
						<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
							<circle cx="18" cy="5" r="3"/>
							<circle cx="6" cy="12" r="3"/>
							<circle cx="18" cy="19" r="3"/>
							<line x1="8.59" y1="13.51" x2="15.42" y2="17.49"/>
							<line x1="15.41" y1="6.51" x2="8.59" y2="10.49"/>
						</svg>
					</button>
				{:else if mode === 'still'}
					<button
						class="btn btn-ghost btn-xs text-base-content/45 hover:text-warning"
						title={t('card.takeOffStill')}
						onclick={() => handleSetStatus(mash.id, MashStatus.MASH_TUN)}
					>
						<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
							<path d="M9 21H5a2 2 0 01-2-2V5a2 2 0 012-2h4"/>
							<polyline points="16 17 21 12 16 7"/>
							<line x1="21" y1="12" x2="9" y2="12"/>
						</svg>
					</button>
				{/if}
				{#if showConfirm}
					<button class="btn btn-ghost btn-xs text-error" onclick={() => handleDelete(mash.id)}>{t('common.delete')}</button>
					<button class="btn btn-ghost btn-xs text-base-content/60" onclick={() => (showConfirm = false)}>{t('common.cancel')}</button>
				{:else}
					<button class="btn btn-ghost btn-xs text-base-content/45 hover:text-error" title={t('common.delete')} onclick={() => (showConfirm = true)}>
						<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
							<polyline points="3 6 5 6 21 6"/>
							<path d="M19 6v14a2 2 0 01-2 2H7a2 2 0 01-2-2V6m3 0V4a2 2 0 012-2h4a2 2 0 012 2v2"/>
						</svg>
					</button>
				{/if}
				<span class="text-xs text-base-content/45">{formatDateTime(mash.createdAt, getDateFormat(), getTimeFormat())}</span>
			</div>
		</div>

		<p class="text-[15px] font-medium leading-relaxed">{mash.summary}</p>

		{#if mash.context}
			<p class="mt-1.5 text-sm text-base-content/65 leading-relaxed">{mash.context}</p>
		{/if}

		{#if mash.memo}
			<p class="mt-1.5 text-sm text-base-content/45 italic leading-relaxed">{mash.memo}</p>
		{/if}
	</article>
{/if}
