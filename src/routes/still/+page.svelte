<script lang="ts">
	import { onMount } from 'svelte';
	import MashList from '$lib/components/MashList.svelte';
	import * as pipelineCmd from '$lib/commands/pipeline';
	import type { PipelineStatus } from '$lib/commands/pipeline';
	import { t } from '$lib/i18n/index.svelte';
	import { getDateFormat, getTimeFormat } from '$lib/stores/settings.svelte';
	import { formatDateTime } from '$lib/utils/datetime';

	let status = $state<PipelineStatus | null>(null);
	let now = $state(Date.now());

	let timer = $derived.by(() => {
		if (!status?.nextRun) return null;

		const intervalMs = status.intervalMin * 60 * 1000;
		const startMs = status.nextRun - intervalMs;
		const diff = status.nextRun - now;

		if (diff <= 0) {
			return { progress: 1, rh: 0, rm: 0, nextLabel: t('still.pipelineSoon') };
		}

		const progress = Math.min((now - startMs) / intervalMs, 1);
		const rh = Math.floor(diff / 3_600_000);
		const rm = Math.floor((diff % 3_600_000) / 60_000);

		const nextLabel = formatDateTime(status.nextRun, getDateFormat(), getTimeFormat());

		return { progress, rh, rm, nextLabel };
	});

	onMount(() => {
		loadStatus();
		const statusInterval = setInterval(loadStatus, 30_000);
		const tickInterval = setInterval(() => { now = Date.now(); }, 30_000);
		return () => {
			clearInterval(statusInterval);
			clearInterval(tickInterval);
		};
	});

	async function loadStatus() {
		try {
			status = await pipelineCmd.getPipelineStatus();
			now = Date.now();
		} catch {
			// silently fail
		}
	}
</script>

<div class="max-w-2xl mx-auto flex flex-col gap-8">
	<div>
		<h1 class="text-2xl font-semibold tracking-tight">{t('still.title')}</h1>
		<p class="text-sm text-base-content/60 mt-1">{t('still.subtitle')}</p>
	</div>

	{#if timer}
		<div class="flex items-center gap-3">
			<span class="text-[11px] text-base-content/45 shrink-0">{t('still.nextPipeline')}</span>
			<div class="flex-1 h-1 rounded-full bg-base-content/[0.08] overflow-hidden">
				<div
					class="h-full rounded-full bg-primary transition-[width] duration-500"
					style="width: {timer.progress * 100}%"
				></div>
			</div>
			<span class="text-[11px] text-base-content/60 font-medium tabular-nums shrink-0">
				{#if timer.rh > 0}
					{timer.rh}{t('still.hours')} {timer.rm}{t('still.minutes')} ({timer.nextLabel})
				{:else if timer.rm > 0}
					{timer.rm}{t('still.minutes')} ({timer.nextLabel})
				{:else}
					{timer.nextLabel}
				{/if}
			</span>
		</div>
	{/if}

	<MashList mode="still" />
</div>
