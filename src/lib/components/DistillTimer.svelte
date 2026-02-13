<script lang="ts">
	import { onMount } from 'svelte';
	import * as pipelineCmd from '$lib/commands/pipeline';
	import type { PipelineStatus } from '$lib/commands/pipeline';
	import { t } from '$lib/i18n/index.svelte';

	let status = $state<PipelineStatus | null>(null);

	onMount(() => {
		loadStatus();
		const interval = setInterval(loadStatus, 30_000);
		return () => clearInterval(interval);
	});

	async function loadStatus() {
		try {
			status = await pipelineCmd.getPipelineStatus();
		} catch {
			// silently fail
		}
	}
</script>

{#if status && (status.onStillCount > 0 || status.distilledCount > 0)}
	<div class="text-[10px] text-base-content/30 px-1 py-0.5 flex items-center gap-1.5" title={t('settings.pipelineStats')}>
		<span class="inline-block w-1.5 h-1.5 rounded-full bg-primary/60 animate-pulse"></span>
		<span>{t('settings.onStill')}: {status.onStillCount}</span>
		<span class="text-base-content/15">|</span>
		<span>{t('settings.jarred')}: {status.jarredCount}</span>
	</div>
{/if}
