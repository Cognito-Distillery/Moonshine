<script lang="ts">
	import { t } from '$lib/i18n/index.svelte';
	import type { MessageKey } from '$lib/i18n/index.svelte';

	const glossary: { titleKey: MessageKey; descKey: MessageKey }[] = [
		{ titleKey: 'help.g.mash', descKey: 'help.g.mash.d' },
		{ titleKey: 'help.g.mashing', descKey: 'help.g.mashing.d' },
		{ titleKey: 'help.g.mashTun', descKey: 'help.g.mashTun.d' },
		{ titleKey: 'help.g.putOnStill', descKey: 'help.g.putOnStill.d' },
		{ titleKey: 'help.g.takeOffStill', descKey: 'help.g.takeOffStill.d' },
		{ titleKey: 'help.g.still', descKey: 'help.g.still.d' },
		{ titleKey: 'help.g.distill', descKey: 'help.g.distill.d' },
		{ titleKey: 'help.g.pipeline', descKey: 'help.g.pipeline.d' },
		{ titleKey: 'help.g.masonJar', descKey: 'help.g.masonJar.d' },
		{ titleKey: 'help.g.jarShelf', descKey: 'help.g.jarShelf.d' }
	];

	const mashTypes: { badgeClass: string; typeKey: MessageKey; descKey: MessageKey }[] = [
		{ badgeClass: 'badge-warning', typeKey: 'type.결정', descKey: 'type.결정.desc' },
		{ badgeClass: 'badge-error', typeKey: 'type.문제', descKey: 'type.문제.desc' },
		{ badgeClass: 'badge-success', typeKey: 'type.인사이트', descKey: 'type.인사이트.desc' },
		{ badgeClass: 'badge-info', typeKey: 'type.질문', descKey: 'type.질문.desc' }
	];

	const settingsItems: { labelKey: MessageKey; descKey: MessageKey }[] = [
		{ labelKey: 'help.settings.pipeline.label', descKey: 'help.settings.pipeline.desc' },
		{ labelKey: 'help.settings.api.label', descKey: 'help.settings.api.desc' },
		{ labelKey: 'help.settings.sidebar.label', descKey: 'help.settings.sidebar.desc' },
		{ labelKey: 'help.settings.lang.label', descKey: 'help.settings.lang.desc' },
		{ labelKey: 'help.settings.datetime.label', descKey: 'help.settings.datetime.desc' },
		{ labelKey: 'help.settings.password.label', descKey: 'help.settings.password.desc' }
	];

	const searchModes: { labelKey: MessageKey; descKey: MessageKey }[] = [
		{ labelKey: 'help.search.local.label', descKey: 'help.search.local.desc' },
		{ labelKey: 'help.search.keyword.label', descKey: 'help.search.keyword.desc' },
		{ labelKey: 'help.search.natural.label', descKey: 'help.search.natural.desc' }
	];

	const graphControls: { labelKey: MessageKey; descKey: MessageKey }[] = [
		{ labelKey: 'help.graph.click.label', descKey: 'help.graph.click.desc' },
		{ labelKey: 'help.graph.dblclick.label', descKey: 'help.graph.dblclick.desc' },
		{ labelKey: 'help.graph.rightClick.label', descKey: 'help.graph.rightClick.desc' },
		{ labelKey: 'help.graph.filter.label', descKey: 'help.graph.filter.desc' },
		{ labelKey: 'help.graph.layout.label', descKey: 'help.graph.layout.desc' },
		{ labelKey: 'help.graph.searchSettings.label', descKey: 'help.graph.searchSettings.desc' },
		{ labelKey: 'help.graph.history.label', descKey: 'help.graph.history.desc' },
		{ labelKey: 'help.graph.highlight.label', descKey: 'help.graph.highlight.desc' }
	];

	const flowSteps: { labelKey: MessageKey; descKey: MessageKey }[] = [
		{ labelKey: 'help.flow.mashing', descKey: 'help.flow.mashing.d' },
		{ labelKey: 'help.flow.mashTun', descKey: 'help.flow.mashTun.d' },
		{ labelKey: 'help.flow.putOnStill', descKey: 'help.flow.putOnStill.d' },
		{ labelKey: 'help.flow.distill', descKey: 'help.flow.distill.d' },
		{ labelKey: 'help.flow.jar', descKey: 'help.flow.jar.d' },
		{ labelKey: 'help.flow.jarShelf', descKey: 'help.flow.jarShelf.d' }
	];
</script>

<div class="max-w-2xl mx-auto flex flex-col gap-8">
	<div>
		<h1 class="text-2xl font-semibold tracking-tight">{t('help.title')}</h1>
		<p class="text-sm text-base-content/60 mt-1">{t('help.subtitle')}</p>
	</div>

	<h2 class="text-lg font-semibold text-base-content/70">{t('help.catOverview')}</h2>

	<section class="border border-base-300 rounded-lg p-5 flex flex-col gap-4">
		<h2 class="text-xs font-medium text-base-content/60 uppercase tracking-wider">{t('help.concept')}</h2>
		<p class="text-sm text-base-content/60 leading-relaxed">
			{t('help.conceptDesc')}
		</p>
	</section>

	<section class="border border-base-300 rounded-lg p-5 flex flex-col gap-3">
		<h2 class="text-xs font-medium text-base-content/60 uppercase tracking-wider">{t('help.glossary')}</h2>
		<table class="table table-sm">
			<tbody>
				{#each glossary as item}
					<tr>
						<td class="font-semibold text-primary whitespace-nowrap pl-0">{t(item.titleKey)}</td>
						<td class="text-base-content/65">{t(item.descKey)}</td>
					</tr>
				{/each}
			</tbody>
		</table>
	</section>

	<section class="border border-base-300 rounded-lg p-5 flex flex-col gap-3">
		<h2 class="text-xs font-medium text-base-content/60 uppercase tracking-wider">{t('help.flow')}</h2>
		<table class="table table-sm">
			<tbody>
				{#each flowSteps as step, i}
					<tr>
						<td class="pl-0 w-8 text-center text-base-content/35">{#if i > 0}&darr;{/if}</td>
						<td class="font-semibold text-primary whitespace-nowrap">{t(step.labelKey)}</td>
						<td class="text-base-content/65">{t(step.descKey)}</td>
					</tr>
				{/each}
			</tbody>
		</table>
	</section>

	<h2 class="text-lg font-semibold text-base-content/70">{t('help.catPages')}</h2>

	<section class="flex flex-col gap-6">
		<div class="border border-base-300 rounded-lg p-5 flex flex-col gap-4">
			<a href="/" class="text-sm font-semibold flex items-center gap-2 text-base-content hover:!text-primary transition-colors">
				<span class="w-6 h-6 flex items-center justify-center rounded bg-primary/15 text-primary text-xs font-bold">1</span>
				{t('help.step1.title')}
				<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="shrink-0 text-base-content/45">
					<line x1="12" y1="5" x2="12" y2="19"/><line x1="5" y1="12" x2="19" y2="12"/>
				</svg>
			</a>
			<p class="text-sm text-base-content/60 leading-relaxed">
				{@html t('help.step1.desc')}
			</p>
			<h3 class="text-xs font-medium text-base-content/60 uppercase tracking-wider">{t('help.mashTypes')}</h3>
			<p class="text-sm text-base-content/60">{t('help.mashTypes.desc')}</p>
			<div class="grid grid-cols-2 gap-3">
				{#each mashTypes as mt}
					<div class="flex items-center gap-2">
						<span class="badge badge-sm badge-outline {mt.badgeClass}">{t(mt.typeKey)}</span>
						<span class="text-sm text-base-content/65">{t(mt.descKey)}</span>
					</div>
				{/each}
			</div>
		</div>

		<div class="border border-base-300 rounded-lg p-5 flex flex-col gap-2">
			<a href="/mashes" class="text-sm font-semibold flex items-center gap-2 text-base-content hover:!text-primary transition-colors">
				<span class="w-6 h-6 flex items-center justify-center rounded bg-primary/15 text-primary text-xs font-bold">2</span>
				{t('help.step2.title')}
				<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="shrink-0 text-base-content/45">
					<path d="M14 2H6a2 2 0 00-2 2v16a2 2 0 002 2h12a2 2 0 002-2V8z"/><polyline points="14 2 14 8 20 8"/><line x1="16" y1="13" x2="8" y2="13"/><line x1="16" y1="17" x2="8" y2="17"/>
				</svg>
			</a>
			<p class="text-sm text-base-content/60 leading-relaxed">
				{@html t('help.step2.desc')}
			</p>
		</div>

		<div class="border border-base-300 rounded-lg p-5 flex flex-col gap-2">
			<a href="/still" class="text-sm font-semibold flex items-center gap-2 text-base-content hover:!text-primary transition-colors">
				<span class="w-6 h-6 flex items-center justify-center rounded bg-primary/15 text-primary text-xs font-bold">3</span>
				{t('help.step3.title')}
				<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="shrink-0 text-base-content/45">
					<path d="M10 2v7.527a2 2 0 01-.211.896L4.72 20.55a1 1 0 00.9 1.45h12.76a1 1 0 00.9-1.45l-5.069-10.127A2 2 0 0114 9.527V2"/><path d="M8.5 2h7"/><path d="M7 16.5h10"/>
				</svg>
			</a>
			<p class="text-sm text-base-content/60 leading-relaxed">
				{@html t('help.step3.desc')}
			</p>
		</div>

		<div class="border border-base-300 rounded-lg p-5 flex flex-col gap-4">
			<a href="/jar-shelf" class="text-sm font-semibold flex items-center gap-2 text-base-content hover:!text-primary transition-colors">
				<span class="w-6 h-6 flex items-center justify-center rounded bg-primary/15 text-primary text-xs font-bold">4</span>
				{t('help.step4.title')}
				<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="shrink-0 text-base-content/45">
					<circle cx="6" cy="6" r="3"/><circle cx="18" cy="6" r="3"/><circle cx="12" cy="18" r="3"/><line x1="8.5" y1="7.5" x2="10.5" y2="16"/><line x1="15.5" y1="7.5" x2="13.5" y2="16"/>
				</svg>
			</a>
			<p class="text-sm text-base-content/60 leading-relaxed">
				{@html t('help.step4.desc')}
			</p>
			<h3 class="text-xs font-medium text-base-content/60 uppercase tracking-wider">{t('help.search')}</h3>
			<p class="text-sm text-base-content/60 leading-relaxed">{t('help.search.desc')}</p>
			<table class="table table-sm">
				<tbody>
					{#each searchModes as item}
						<tr>
							<td class="font-semibold text-primary w-24 pl-0">{t(item.labelKey)}</td>
							<td class="text-base-content/65">{t(item.descKey)}</td>
						</tr>
					{/each}
				</tbody>
			</table>
			<h3 class="text-xs font-medium text-base-content/60 uppercase tracking-wider">{t('help.graph')}</h3>
			<p class="text-sm text-base-content/60 leading-relaxed">{t('help.graph.desc')}</p>
			<table class="table table-sm">
				<tbody>
					{#each graphControls as item}
						<tr>
							<td class="font-semibold text-primary w-28 pl-0">{t(item.labelKey)}</td>
							<td class="text-base-content/65">{t(item.descKey)}</td>
						</tr>
					{/each}
				</tbody>
			</table>
		</div>

		<div class="border border-base-300 rounded-lg p-5 flex flex-col gap-4">
			<a href="/settings" class="text-sm font-semibold flex items-center gap-2 text-base-content hover:!text-primary transition-colors">
				<span class="w-6 h-6 flex items-center justify-center rounded bg-primary/15 text-primary text-xs font-bold">5</span>
				{t('help.settings')}
				<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" class="shrink-0 text-base-content/45">
					<circle cx="12" cy="12" r="3"/>
					<path d="M19.4 15a1.65 1.65 0 00.33 1.82l.06.06a2 2 0 010 2.83 2 2 0 01-2.83 0l-.06-.06a1.65 1.65 0 00-1.82-.33 1.65 1.65 0 00-1 1.51V21a2 2 0 01-4 0v-.09A1.65 1.65 0 009 19.4a1.65 1.65 0 00-1.82.33l-.06.06a2 2 0 01-2.83 0 2 2 0 010-2.83l.06-.06A1.65 1.65 0 004.68 15a1.65 1.65 0 00-1.51-1H3a2 2 0 010-4h.09A1.65 1.65 0 004.6 9a1.65 1.65 0 00-.33-1.82l-.06-.06a2 2 0 010-2.83 2 2 0 012.83 0l.06.06A1.65 1.65 0 009 4.68a1.65 1.65 0 001-1.51V3a2 2 0 014 0v.09a1.65 1.65 0 001 1.51 1.65 1.65 0 001.82-.33l.06-.06a2 2 0 012.83 0 2 2 0 010 2.83l-.06.06A1.65 1.65 0 0019.4 9a1.65 1.65 0 001.51 1H21a2 2 0 010 4h-.09a1.65 1.65 0 00-1.51 1z"/>
				</svg>
			</a>
			<p class="text-sm text-base-content/60 leading-relaxed">{t('help.settings.desc')}</p>
			<table class="table table-sm">
				<tbody>
					{#each settingsItems as item}
						<tr>
							<td class="font-semibold text-primary w-32 pl-0">{t(item.labelKey)}</td>
							<td class="text-base-content/65">{t(item.descKey)}</td>
						</tr>
					{/each}
				</tbody>
			</table>
		</div>
	</section>

	<h2 class="text-lg font-semibold text-base-content/70">{t('help.catExtra')}</h2>

	<section class="border border-base-300 rounded-lg p-5 flex flex-col gap-4">
		<h2 class="text-xs font-medium text-base-content/60 uppercase tracking-wider">{t('help.floatingMemo')}</h2>
		<p class="text-sm text-base-content/60 leading-relaxed">{t('help.floatingMemo.desc')}</p>
		<h3 class="text-xs font-medium text-base-content/60 uppercase tracking-wider">{t('help.shortcuts')}</h3>
		<p class="text-sm text-base-content/60 leading-relaxed">{t('help.shortcuts.desc')}</p>
		<table class="table table-sm">
			<thead>
				<tr>
					<th class="text-base-content/60 pl-0">{t('help.shortcuts.action')}</th>
					<th class="text-base-content/60">{t('help.shortcuts.windows')}</th>
					<th class="text-base-content/60">{t('help.shortcuts.macos')}</th>
					<th class="text-base-content/60">{t('help.shortcuts.linuxX11')}</th>
				</tr>
			</thead>
			<tbody>
				<tr>
					<td class="text-base-content/65 pl-0">{t('help.shortcuts.floatingMemo')}</td>
					<td><kbd class="kbd kbd-sm">{t('help.shortcuts.windows.shortcut')}</kbd></td>
					<td><kbd class="kbd kbd-sm">{t('help.shortcuts.macos.shortcut')}</kbd></td>
					<td><kbd class="kbd kbd-sm">{t('help.shortcuts.linuxX11.shortcut')}</kbd></td>
				</tr>
			</tbody>
		</table>
		<p class="text-xs text-base-content/50">{t('help.shortcuts.waylandHint')}</p>
		<code class="text-xs bg-base-content/[0.08] rounded px-2 py-1 font-mono">dbus-send --session --dest=com.moonshine.App --type=method_call /com/moonshine/App com.moonshine.App.ToggleFloatingMemo</code>
	</section>

	<h2 class="text-lg font-semibold text-base-content/70">{t('help.catTechnical')}</h2>

	<section class="border border-base-300 rounded-lg p-5 flex flex-col gap-4">
		<h2 class="text-xs font-medium text-base-content/60 uppercase tracking-wider">{t('help.data')}</h2>
		<p class="text-sm text-base-content/60 leading-relaxed">{t('help.data.desc')}</p>
		<ul class="flex flex-col gap-2 text-sm text-base-content/60 leading-relaxed pl-1 font-mono text-xs">
			<li>{@html t('help.data.linux')}</li>
			<li>{@html t('help.data.mac')}</li>
			<li>{@html t('help.data.win')}</li>
		</ul>
	</section>

</div>
