<script lang="ts">
	import { t } from '$lib/i18n/index.svelte';
	import type { MessageKey } from '$lib/i18n/index.svelte';

	const glossary: { titleKey: MessageKey; descKey: MessageKey }[] = [
		{ titleKey: 'help.g.mashing', descKey: 'help.g.mashing.d' },
		{ titleKey: 'help.g.mash', descKey: 'help.g.mash.d' },
		{ titleKey: 'help.g.mashTun', descKey: 'help.g.mashTun.d' },
		{ titleKey: 'help.g.putOnStill', descKey: 'help.g.putOnStill.d' },
		{ titleKey: 'help.g.takeOffStill', descKey: 'help.g.takeOffStill.d' },
		{ titleKey: 'help.g.still', descKey: 'help.g.still.d' },
		{ titleKey: 'help.g.distill', descKey: 'help.g.distill.d' },
		{ titleKey: 'help.g.masonJar', descKey: 'help.g.masonJar.d' }
	];

	const mashTypes: { badgeClass: string; typeKey: MessageKey; descKey: MessageKey }[] = [
		{ badgeClass: 'badge-warning', typeKey: 'type.결정', descKey: 'type.결정.desc' },
		{ badgeClass: 'badge-error', typeKey: 'type.문제', descKey: 'type.문제.desc' },
		{ badgeClass: 'badge-success', typeKey: 'type.인사이트', descKey: 'type.인사이트.desc' },
		{ badgeClass: 'badge-info', typeKey: 'type.질문', descKey: 'type.질문.desc' }
	];

	const flowSteps: MessageKey[] = [
		'help.flow.mashing',
		'help.flow.mashTun',
		'help.flow.still',
		'help.flow.distill',
		'help.flow.masonJar'
	];
</script>

<div class="max-w-2xl mx-auto flex flex-col gap-8">
	<div>
		<h1 class="text-2xl font-semibold tracking-tight">{t('help.title')}</h1>
		<p class="text-sm text-base-content/40 mt-1">{t('help.subtitle')}</p>
	</div>

	<section class="border border-base-300 rounded-lg p-5 flex flex-col gap-4">
		<h2 class="text-xs font-medium text-base-content/40 uppercase tracking-wider">{t('help.concept')}</h2>
		<p class="text-sm text-base-content/60 leading-relaxed">
			{t('help.conceptDesc')}
		</p>
	</section>

	<section class="flex flex-col gap-6">
		<div class="border border-base-300 rounded-lg p-5 flex flex-col gap-2">
			<a href="/" class="text-sm font-semibold flex items-center gap-2 text-base-content hover:!text-primary transition-colors">
				<span class="w-6 h-6 flex items-center justify-center rounded bg-primary/15 text-primary text-xs font-bold">1</span>
				{t('help.step1.title')}
			</a>
			<p class="text-sm text-base-content/60 leading-relaxed">
				{@html t('help.step1.desc')}
			</p>
		</div>

		<div class="border border-base-300 rounded-lg p-5 flex flex-col gap-2">
			<a href="/mashes" class="text-sm font-semibold flex items-center gap-2 text-base-content hover:!text-primary transition-colors">
				<span class="w-6 h-6 flex items-center justify-center rounded bg-primary/15 text-primary text-xs font-bold">2</span>
				{t('help.step2.title')}
			</a>
			<p class="text-sm text-base-content/60 leading-relaxed">
				{@html t('help.step2.desc')}
			</p>
		</div>

		<div class="border border-base-300 rounded-lg p-5 flex flex-col gap-2">
			<a href="/still" class="text-sm font-semibold flex items-center gap-2 text-base-content hover:!text-primary transition-colors">
				<span class="w-6 h-6 flex items-center justify-center rounded bg-primary/15 text-primary text-xs font-bold">3</span>
				{t('help.step3.title')}
			</a>
			<p class="text-sm text-base-content/60 leading-relaxed">
				{@html t('help.step3.desc')}
			</p>
		</div>

		<div class="border border-base-300 rounded-lg p-5 flex flex-col gap-2">
			<a href="/jar-shelf" class="text-sm font-semibold flex items-center gap-2 text-base-content hover:!text-primary transition-colors">
				<span class="w-6 h-6 flex items-center justify-center rounded bg-primary/15 text-primary text-xs font-bold">4</span>
				{t('help.step4.title')}
			</a>
			<p class="text-sm text-base-content/60 leading-relaxed">
				{@html t('help.step4.desc')}
			</p>
		</div>
	</section>

	<section class="border border-base-300 rounded-lg p-5 flex flex-col gap-3">
		<h2 class="text-xs font-medium text-base-content/40 uppercase tracking-wider">{t('help.glossary')}</h2>
		<div class="flex flex-col gap-2.5">
			{#each glossary as item}
				<div class="flex items-baseline gap-3">
					<span class="text-sm font-semibold text-primary min-w-28 shrink-0">{t(item.titleKey)}</span>
					<span class="text-sm text-base-content/50">{t(item.descKey)}</span>
				</div>
			{/each}
		</div>
	</section>

	<section class="border border-base-300 rounded-lg p-5 flex flex-col gap-3">
		<h2 class="text-xs font-medium text-base-content/40 uppercase tracking-wider">{t('help.mashTypes')}</h2>
		<div class="grid grid-cols-2 gap-3">
			{#each mashTypes as mt}
				<div class="flex items-center gap-2">
					<span class="badge badge-sm badge-outline {mt.badgeClass}">{t(mt.typeKey)}</span>
					<span class="text-sm text-base-content/50">{t(mt.descKey)}</span>
				</div>
			{/each}
		</div>
	</section>

	<section class="border border-base-300 rounded-lg p-5 flex flex-col gap-3">
		<h2 class="text-xs font-medium text-base-content/40 uppercase tracking-wider">{t('help.flow')}</h2>
		<div class="flex items-center gap-3 text-sm text-base-content/50">
			{#each flowSteps as step, i}
				{#if i > 0}
					<span class="text-base-content/20">&rarr;</span>
				{/if}
				<span class="badge badge-sm badge-ghost">{t(step)}</span>
			{/each}
		</div>
	</section>
</div>
