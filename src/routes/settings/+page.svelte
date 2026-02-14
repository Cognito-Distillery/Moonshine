<script lang="ts">
	import { onMount } from 'svelte';
	import { check } from '@tauri-apps/plugin-updater';
	import { relaunch } from '@tauri-apps/plugin-process';
	import { getSidebarPosition, setSidebarPosition, getDateFormat, setDateFormat, getTimeFormat, setTimeFormat } from '$lib/stores/settings.svelte';
	import type { SidebarPosition } from '$lib/stores/settings.svelte';
	import { formatDateTime } from '$lib/utils/datetime';
	import type { DateFormatId, TimeFormatId } from '$lib/utils/datetime';
	import { authStore } from '$lib/stores/auth.svelte';
	import * as authCmd from '$lib/commands/auth';
	import * as settingsCmd from '$lib/commands/settings';
	import * as pipelineCmd from '$lib/commands/pipeline';
	import type { PipelineStatus } from '$lib/commands/pipeline';
	import { showToast } from '$lib/stores/toast.svelte';
	import { t } from '$lib/i18n/index.svelte';
	import { getLocale, setLocale } from '$lib/i18n/index.svelte';
	import type { Locale, MessageKey } from '$lib/i18n/index.svelte';

	const positions: { value: SidebarPosition; titleKey: MessageKey }[] = [
		{ value: 'top', titleKey: 'settings.pos.top' },
		{ value: 'left', titleKey: 'settings.pos.left' },
		{ value: 'right', titleKey: 'settings.pos.right' },
		{ value: 'bottom', titleKey: 'settings.pos.bottom' }
	];

	const languages: { value: Locale; label: string }[] = [
		{ value: 'ko', label: '한국어' },
		{ value: 'en', label: 'English' }
	];

	const providers: { value: string; labelKey: MessageKey }[] = [
		{ value: 'openai', labelKey: 'settings.providerOpenAI' },
		{ value: 'gemini', labelKey: 'settings.providerGemini' }
	];

	const embeddingModelPresets: Record<string, string[]> = {
		openai: ['text-embedding-3-small', 'text-embedding-3-large'],
		gemini: ['gemini-embedding-001']
	};

	const chatModelPresets: Record<string, string[]> = {
		openai: ['gpt-4o-mini', 'gpt-4o', 'gpt-4.1-mini', 'gpt-4.1-nano'],
		gemini: ['gemini-2.0-flash', 'gemini-2.5-flash']
	};

	const defaultEmbeddingModels: Record<string, string> = {
		openai: 'text-embedding-3-small',
		gemini: 'gemini-embedding-001'
	};

	const defaultChatModels: Record<string, string> = {
		openai: 'gpt-4o-mini',
		gemini: 'gemini-2.0-flash'
	};

	// Password
	let currentPassword = $state('');
	let newPassword = $state('');
	let passwordSaving = $state(false);

	// AI Provider
	let embeddingProvider = $state('openai');
	let providerSwitching = $state(false);
	let providerWarningOpen = $state(false);

	// Models
	let embeddingModel = $state('text-embedding-3-small');
	let chatModel = $state('gpt-4o-mini');
	let customEmbeddingModel = $state('');
	let customChatModel = $state('');
	let embeddingModelCustom = $state(false);
	let chatModelCustom = $state(false);
	let modelSwitching = $state(false);

	// Confirmation dialogs for re-embed/re-extract
	let reembedConfirmOpen = $state(false);
	let reembedPendingAction = $state<(() => Promise<void>) | null>(null);
	let reextractConfirmOpen = $state(false);
	let reextractRunning = $state(false);



	// API Key
	let apiKey = $state('');
	let apiKeySaving = $state(false);
	let apiKeyEditing = $state(true);
	let apiKeySavedValue = $state('');

	function maskApiKey(key: string): string {
		if (key.length <= 8) return '••••••••';
		return key.slice(0, 4) + '••••••••' + key.slice(-4);
	}

	// Update
	let updateStatus: 'idle' | 'checking' | 'available' | 'downloading' | 'ready' | 'upToDate' | 'error' = $state('idle');
	let updateVersion = $state('');

	// Pipeline
	let pipelineInterval = $state(30);
	let pipelineStatus = $state<PipelineStatus | null>(null);
	let progressPhase = $derived(pipelineStatus?.progress?.phase ?? null);
	let progressStep = $derived(pipelineStatus?.progress?.step ?? null);
	let progressCurrent = $derived(pipelineStatus?.progress?.current ?? 0);
	let progressTotal = $derived(pipelineStatus?.progress?.total ?? 0);
	let progressPercent = $derived(progressTotal > 0 ? Math.round((progressCurrent / progressTotal) * 100) : 0);
	let pipelineRunning = $state(false);
	let pipelineWarningOpen = $state(false);

	// Similarity (pipeline only; search settings are in FilterPanel)
	let pipelineThreshold = $state(0.3);
	let pipelineTopK = $state(5);
	let similaritySaving = $state(false);

	let apiKeySettingName = $derived(
		embeddingProvider === 'gemini' ? 'gemini_api_key' : 'openai_api_key'
	);

	let apiKeyPlaceholder = $derived(
		embeddingProvider === 'gemini' ? t('settings.geminiApiKeyPlaceholder') : t('settings.apiKeyPlaceholder')
	);

	let intervalPercent = $derived(((pipelineInterval - 5) / (60 - 5)) * 100);

	// Poll pipeline status while running (every 3s)
	let pollTimer: ReturnType<typeof setInterval> | null = null;
	function startPolling() {
		if (pollTimer) return;
		pollTimer = setInterval(async () => {
			try {
				const status = await pipelineCmd.getPipelineStatus();
				pipelineStatus = status;
				if (!status.running) stopPolling();
			} catch { /* ignore */ }
		}, 500);
	}
	function stopPolling() {
		if (pollTimer) { clearInterval(pollTimer); pollTimer = null; }
	}

	onMount(() => {
		const previewTimer = setInterval(() => { previewNow = Date.now(); }, 1000);

		(async () => { try {
			const [providerVal, statusVal, embModelVal, chatModelVal, pThresholdVal, pTopKVal] = await Promise.all([
				settingsCmd.getSetting('embedding_provider'),
				pipelineCmd.getPipelineStatus(),
				settingsCmd.getSetting('embedding_model'),
				settingsCmd.getSetting('chat_model'),
				settingsCmd.getSetting('pipeline_threshold'),
				settingsCmd.getSetting('pipeline_top_k')
			]);
			embeddingProvider = providerVal || 'openai';
			pipelineStatus = statusVal;
			pipelineInterval = statusVal.intervalMin;

			// Start polling if pipeline is already running
			if (statusVal.running) startPolling();

			// Load similarity settings
			if (pThresholdVal) pipelineThreshold = parseFloat(pThresholdVal);
			if (pTopKVal) pipelineTopK = parseInt(pTopKVal);

			// Load models (default if not set)
			const provider = embeddingProvider;
			embeddingModel = embModelVal || defaultEmbeddingModels[provider];
			chatModel = chatModelVal || defaultChatModels[provider];

			// Check if current values are custom (not in presets)
			if (!embeddingModelPresets[provider]?.includes(embeddingModel)) {
				customEmbeddingModel = embeddingModel;
				embeddingModelCustom = true;
			}
			if (!chatModelPresets[provider]?.includes(chatModel)) {
				customChatModel = chatModel;
				chatModelCustom = true;
			}

			// Load the API key for the current provider
			const keyName = embeddingProvider === 'gemini' ? 'gemini_api_key' : 'openai_api_key';
			const keyVal = await settingsCmd.getSetting(keyName);
			if (keyVal) {
				apiKey = keyVal;
				apiKeySavedValue = keyVal;
				apiKeyEditing = false;
			}
		} catch {
			// ignore
		}
			checkForUpdates();
		})();

		return () => {
			clearInterval(previewTimer);
			stopPolling();
		};
	});

	function handleSwitchProvider(provider: string) {
		if (provider === embeddingProvider) return;
		reembedPendingAction = () => doSwitchProvider(provider);
		reembedConfirmOpen = true;
	}

	async function doSwitchProvider(provider: string) {
		providerSwitching = true;
		startPolling();
		try {
			const resetCount = await settingsCmd.switchEmbeddingProvider(provider);
			embeddingProvider = provider;

			embeddingModel = defaultEmbeddingModels[provider];
			chatModel = defaultChatModels[provider];
			embeddingModelCustom = false;
			chatModelCustom = false;
			customEmbeddingModel = '';
			customChatModel = '';

			const keyName = provider === 'gemini' ? 'gemini_api_key' : 'openai_api_key';
			const keyVal = await settingsCmd.getSetting(keyName);
			apiKey = keyVal || '';
			apiKeySavedValue = keyVal || '';
			apiKeyEditing = !keyVal;

			if (resetCount > 0) {
				showToast(t('settings.providerSwitched').replace('{count}', String(resetCount)), 'success');
			} else {
				showToast(t('common.saved'), 'success');
			}
			pipelineStatus = await pipelineCmd.getPipelineStatus();
		} catch (e) {
			showToast(String(e));
		} finally {
			providerSwitching = false;
		}
	}

	function handleSwitchEmbeddingModel(model: string) {
		if (model === embeddingModel) return;
		reembedPendingAction = () => doSwitchEmbeddingModel(model);
		reembedConfirmOpen = true;
	}

	async function doSwitchEmbeddingModel(model: string) {
		modelSwitching = true;
		startPolling();
		try {
			const resetCount = await settingsCmd.switchEmbeddingModel(model);
			embeddingModel = model;
			if (resetCount > 0) {
				showToast(t('settings.embeddingModelChanged').replace('{count}', String(resetCount)), 'success');
			} else {
				showToast(t('common.saved'), 'success');
			}
			pipelineStatus = await pipelineCmd.getPipelineStatus();
		} catch (e) {
			showToast(String(e));
		} finally {
			modelSwitching = false;
		}
	}

	async function handleConfirmReembed() {
		reembedConfirmOpen = false;
		if (reembedPendingAction) {
			await reembedPendingAction();
			reembedPendingAction = null;
		}
	}

	async function handleReextract() {
		reextractConfirmOpen = true;
	}

	async function handleConfirmReextract() {
		reextractConfirmOpen = false;
		reextractRunning = true;
		startPolling();
		try {
			const count = await settingsCmd.reextractRelationships();
			pipelineStatus = await pipelineCmd.getPipelineStatus();
			showToast(t('settings.reextractDone').replace('{count}', String(count)), 'success');
		} catch (e) {
			showToast(String(e));
		} finally {
			reextractRunning = false;
		}
	}

	async function handleSwitchChatModel(model: string) {
		if (model === chatModel) return;
		modelSwitching = true;
		try {
			await settingsCmd.switchChatModel(model);
			chatModel = model;
			showToast(t('settings.chatModelChanged'), 'success');
		} catch (e) {
			showToast(String(e));
		} finally {
			modelSwitching = false;
		}
	}

	async function handleSetPassword() {
		passwordSaving = true;
		try {
			await authCmd.setPassword(
				authStore.passwordSet ? currentPassword : null,
				newPassword
			);
			await authStore.checkAuth();
			currentPassword = '';
			newPassword = '';
			showToast(t('common.saved'), 'success');
		} catch (e) {
			showToast(String(e));
		} finally {
			passwordSaving = false;
		}
	}

	async function handleRemovePassword() {
		if (!currentPassword.trim()) return;
		passwordSaving = true;
		try {
			await authCmd.removePassword(currentPassword);
			await authStore.checkAuth();
			currentPassword = '';
			showToast(t('common.saved'), 'success');
		} catch (e) {
			showToast(String(e));
		} finally {
			passwordSaving = false;
		}
	}

	async function handleSaveApiKey() {
		apiKeySaving = true;
		try {
			await settingsCmd.setSetting(apiKeySettingName, apiKey);
			apiKeySavedValue = apiKey;
			apiKeyEditing = false;
			showToast(t('common.saved'), 'success');
		} catch (e) {
			showToast(String(e));
		} finally {
			apiKeySaving = false;
		}
	}

	async function handleSetInterval() {
		try {
			await pipelineCmd.setPipelineInterval(pipelineInterval);
			pipelineStatus = await pipelineCmd.getPipelineStatus();
			showToast(t('common.saved'), 'success');
		} catch (e) {
			showToast(String(e));
		}
	}

	async function handleTriggerPipeline() {
		pipelineRunning = true;
		startPolling();
		try {
			await pipelineCmd.triggerPipeline();
			pipelineStatus = await pipelineCmd.getPipelineStatus();
			showToast(t('settings.pipelineDone'), 'success');
		} catch (e) {
			showToast(String(e));
		} finally {
			pipelineRunning = false;
		}
	}

	async function checkForUpdates() {
		updateStatus = 'checking';
		try {
			const update = await check();
			if (update) {
				updateVersion = update.version;
				updateStatus = 'available';
			} else {
				updateStatus = 'upToDate';
			}
		} catch {
			updateStatus = 'error';
		}
	}

	async function installUpdate() {
		updateStatus = 'downloading';
		try {
			const update = await check();
			if (update) {
				await update.downloadAndInstall();
				updateStatus = 'ready';
				await relaunch();
			}
		} catch {
			updateStatus = 'error';
		}
	}

	async function handleSaveSimilarity(key: string, value: string) {
		similaritySaving = true;
		try {
			await settingsCmd.setSetting(key, value);
			showToast(t('common.saved'), 'success');
		} catch (e) {
			showToast(String(e));
		} finally {
			similaritySaving = false;
		}
	}

	async function handleResetSimilarityDefaults() {
		similaritySaving = true;
		try {
			await Promise.all([
				settingsCmd.setSetting('pipeline_threshold', '0.3'),
				settingsCmd.setSetting('pipeline_top_k', '5')
			]);
			pipelineThreshold = 0.3;
			pipelineTopK = 5;
			showToast(t('settings.defaultsRestored'), 'success');
		} catch (e) {
			showToast(String(e));
		} finally {
			similaritySaving = false;
		}
	}

	const dateFormats: { value: DateFormatId; labelKey: MessageKey }[] = [
		{ value: 'short', labelKey: 'settings.dateFormat.short' },
		{ value: 'medium', labelKey: 'settings.dateFormat.medium' },
		{ value: 'iso', labelKey: 'settings.dateFormat.iso' },
		{ value: 'slash', labelKey: 'settings.dateFormat.slash' },
		{ value: 'dot', labelKey: 'settings.dateFormat.dot' }
	];

	const timeFormats: { value: TimeFormatId; labelKey: MessageKey }[] = [
		{ value: '24h', labelKey: 'settings.timeFormat.24h' },
		{ value: '12h', labelKey: 'settings.timeFormat.12h' },
		{ value: '24h-sec', labelKey: 'settings.timeFormat.24h-sec' }
	];

	let previewNow = $state(Date.now());

	function formatPipelineTime(ts: number | null): string {
		if (!ts) return t('settings.pipelineNever');
		return formatDateTime(ts, getDateFormat(), getTimeFormat());
	}
</script>

<div class="max-w-lg mx-auto flex flex-col gap-8">
	<div>
		<h1 class="text-2xl font-semibold tracking-tight">{t('settings.title')}</h1>
	</div>

	<!-- App Update -->
	<section class="border border-base-300 rounded-lg p-5 flex flex-col gap-5">
		<h2 class="text-xs font-medium text-base-content/40 uppercase tracking-wider">{t('update.app')}</h2>
		<div class="flex items-center justify-between">
			<span class="text-sm">{t('update.version')}: <strong>v{__APP_VERSION__}</strong></span>

			{#if updateStatus === 'idle' || updateStatus === 'checking'}
				<span class="text-xs text-base-content/40 flex items-center gap-1">
					<span class="loading loading-spinner loading-xs"></span>
					{t('update.checking')}
				</span>
			{:else if updateStatus === 'available'}
				<button class="btn btn-sm btn-primary" onclick={installUpdate}>
					{t('update.install')}
				</button>
			{:else if updateStatus === 'downloading'}
				<button class="btn btn-sm btn-primary" disabled>
					<span class="loading loading-spinner loading-xs"></span>
					{t('update.downloading')}
				</button>
			{:else if updateStatus === 'ready'}
				<span class="text-xs text-success">{t('update.readyToInstall')}</span>
			{:else if updateStatus === 'upToDate'}
				<span class="text-xs text-success">{t('update.upToDate')}</span>
			{:else if updateStatus === 'error'}
				<div class="flex items-center gap-2">
					<span class="text-xs text-error">{t('update.failed')}</span>
					<button class="btn btn-sm btn-ghost btn-xs" onclick={checkForUpdates} aria-label={t('update.check')}>
						<svg xmlns="http://www.w3.org/2000/svg" class="size-3.5" viewBox="0 0 20 20" fill="currentColor">
							<path fill-rule="evenodd" d="M4 2a1 1 0 011 1v2.101a7.002 7.002 0 0111.601 2.566 1 1 0 11-1.885.666A5.002 5.002 0 005.999 7H9a1 1 0 010 2H4a1 1 0 01-1-1V3a1 1 0 011-1zm.008 9.057a1 1 0 011.276.61A5.002 5.002 0 0014.001 13H11a1 1 0 110-2h5a1 1 0 011 1v5a1 1 0 11-2 0v-2.101a7.002 7.002 0 01-11.601-2.566 1 1 0 01.61-1.276z" clip-rule="evenodd" />
						</svg>
					</button>
				</div>
			{/if}
		</div>

		{#if updateStatus === 'available'}
			<p class="text-xs text-info">{t('update.available').replace('{version}', updateVersion)}</p>
		{/if}
	</section>

	<!-- Layout -->
	<section class="border border-base-300 rounded-lg p-5 flex flex-col gap-5">
		<h2 class="text-xs font-medium text-base-content/40 uppercase tracking-wider">{t('settings.layout')}</h2>

		<div class="flex items-center justify-between">
			<div class="flex flex-col gap-0.5">
				<span class="text-sm font-medium">{t('settings.sidebarPosition')}</span>
				<span class="text-xs text-base-content/35">{t('settings.sidebarDesc')}</span>
			</div>
			<div class="join">
				{#each positions as pos}
					<button
						class="btn btn-sm join-item"
						class:btn-primary={getSidebarPosition() === pos.value}
						onclick={() => { setSidebarPosition(pos.value); showToast(t('common.saved'), 'success'); }}
					>
						{t(pos.titleKey)}
					</button>
				{/each}
			</div>
		</div>

		<div class="flex items-center justify-between">
			<div class="flex flex-col gap-0.5">
				<span class="text-sm font-medium">{t('settings.language')}</span>
				<span class="text-xs text-base-content/35">{t('settings.languageDesc')}</span>
			</div>
			<div class="join">
				{#each languages as lang}
					<button
						class="btn btn-sm join-item"
						class:btn-primary={getLocale() === lang.value}
						onclick={() => { setLocale(lang.value); showToast(t('common.saved'), 'success'); }}
					>
						{lang.label}
					</button>
				{/each}
			</div>
		</div>
	</section>

	<!-- Date & Time Format -->
	<section class="border border-base-300 rounded-lg p-5 flex flex-col gap-5">
		<h2 class="text-xs font-medium text-base-content/40 uppercase tracking-wider">{t('settings.dateTimeFormat')}</h2>

		<div class="flex flex-col gap-2">
			<span class="text-sm font-medium">{t('settings.dateFormat')}</span>
			<div class="flex flex-col gap-1">
				{#each dateFormats as df}
					<label class="flex items-center gap-2 cursor-pointer px-2 py-1 rounded hover:bg-white/[0.06]">
						<input
							type="radio"
							name="dateFormat"
							class="radio radio-sm radio-primary"
							value={df.value}
							checked={getDateFormat() === df.value}
							onchange={() => { setDateFormat(df.value); showToast(t('common.saved'), 'success'); }}
						/>
						<span class="text-sm">{t(df.labelKey)}</span>
					</label>
				{/each}
			</div>
		</div>

		<div class="flex flex-col gap-2">
			<span class="text-sm font-medium">{t('settings.timeFormat')}</span>
			<div class="flex flex-col gap-1">
				{#each timeFormats as tf}
					<label class="flex items-center gap-2 cursor-pointer px-2 py-1 rounded hover:bg-white/[0.06]">
						<input
							type="radio"
							name="timeFormat"
							class="radio radio-sm radio-primary"
							value={tf.value}
							checked={getTimeFormat() === tf.value}
							onchange={() => { setTimeFormat(tf.value); showToast(t('common.saved'), 'success'); }}
						/>
						<span class="text-sm">{t(tf.labelKey)}</span>
					</label>
				{/each}
			</div>
		</div>

		<div class="flex items-center gap-2 px-2 py-2 bg-white/[0.04] rounded">
			<span class="text-xs text-base-content/40">{t('settings.preview')}:</span>
			<span class="text-sm font-medium tabular-nums">{formatDateTime(previewNow, getDateFormat(), getTimeFormat())}</span>
		</div>
	</section>

	<!-- Password -->
	<section class="border border-base-300 rounded-lg p-5 flex flex-col gap-5">
		<h2 class="text-xs font-medium text-base-content/40 uppercase tracking-wider">{t('settings.password')}</h2>
		<p class="text-xs text-base-content/35">{t('settings.passwordDesc')}</p>
		<p class="text-xs">
			{authStore.passwordSet ? t('settings.passwordSet') : t('settings.passwordNotSet')}
		</p>

		{#if authStore.passwordSet}
			<input
				type="password"
				class="input input-sm w-full bg-white/[0.12] border-white/[0.18]"
				placeholder={t('settings.currentPassword')}
				bind:value={currentPassword}
			/>
		{/if}
		<input
			type="password"
			class="input input-sm w-full bg-white/[0.12] border-white/[0.18]"
			placeholder={t('settings.newPassword')}
			bind:value={newPassword}
		/>

		<div class="flex gap-2">
			<button
				class="btn btn-sm btn-primary flex-1"
				onclick={handleSetPassword}
				disabled={passwordSaving || !newPassword.trim()}
			>
				{authStore.passwordSet ? t('settings.changePassword') : t('settings.setPassword')}
			</button>
			{#if authStore.passwordSet}
				<button
					class="btn btn-sm btn-error flex-1"
					onclick={handleRemovePassword}
					disabled={passwordSaving || !currentPassword.trim()}
				>
					{t('settings.removePassword')}
				</button>
			{/if}
		</div>
	</section>

	<!-- Pipeline -->
	<section class="border border-base-300 rounded-lg p-5 flex flex-col gap-5">
		<div class="flex items-center justify-between">
			<h2 class="text-xs font-medium text-base-content/40 uppercase tracking-wider">{t('settings.pipeline')}</h2>
			<button
				class="btn btn-ghost btn-xs btn-circle text-warning"
				aria-label={t('settings.pipelineWarningTitle')}
				onclick={() => (pipelineWarningOpen = true)}
			>
				<svg xmlns="http://www.w3.org/2000/svg" class="size-4" viewBox="0 0 20 20" fill="currentColor">
					<path fill-rule="evenodd" d="M8.485 2.495c.673-1.167 2.357-1.167 3.03 0l6.28 10.875c.673 1.167-.17 2.625-1.516 2.625H3.72c-1.347 0-2.189-1.458-1.515-2.625L8.485 2.495zM10 6a.75.75 0 01.75.75v3.5a.75.75 0 01-1.5 0v-3.5A.75.75 0 0110 6zm0 9a1 1 0 100-2 1 1 0 000 2z" clip-rule="evenodd" />
				</svg>
			</button>
		</div>
		<p class="text-xs text-base-content/35">{t('settings.pipelineDesc')}</p>

		<div class="flex items-center gap-3">
			<span class="text-sm">{t('settings.pipelineInterval')}</span>
			<input
				type="range"
				min="5"
				max="60"
				step="5"
				class="range range-sm range-primary flex-1"
				style="--range-fill: {intervalPercent}%;"
				bind:value={pipelineInterval}
				onchange={handleSetInterval}
			/>
			<span class="text-sm font-mono w-8 text-right">{pipelineInterval}</span>
		</div>

		<button
			class="btn btn-sm btn-outline"
			onclick={handleTriggerPipeline}
			disabled={pipelineRunning || pipelineStatus?.running || reextractRunning || providerSwitching || modelSwitching}
		>
			{#if pipelineRunning}
				<span class="loading loading-spinner loading-xs"></span>
				{t('settings.pipelineRunning')}
			{:else}
				{t('settings.pipelineTrigger')}
			{/if}
		</button>

		{#if pipelineStatus}
			<div class="text-xs text-base-content/50 space-y-1">
				<div>{t('settings.pipelineLastRun')}: {formatPipelineTime(pipelineStatus.lastRun)}</div>
				<div class="flex gap-4 flex-wrap">
					<span>{t('settings.onStill')}: {pipelineStatus.onStillCount}</span>
					<span>{t('settings.distilled')}: {pipelineStatus.distilledCount}</span>
					<span>{t('settings.jarred')}: {pipelineStatus.jarredCount}</span>
					</div>
			</div>
		{/if}

		<div class="flex items-center gap-3">
			<div class="flex flex-col gap-0.5 flex-1">
				<span class="text-sm font-medium">{t('settings.pipelineThreshold')}</span>
				<span class="text-xs text-base-content/35">{t('settings.pipelineThresholdDesc')}</span>
			</div>
			<input
				type="number"
				class="input input-sm w-24 bg-white/[0.12] border-white/[0.18] text-right"
				step="0.05"
				min="0.1"
				max="0.9"
				bind:value={pipelineThreshold}
			/>
			<button
				class="btn btn-sm btn-primary"
				onclick={() => handleSaveSimilarity('pipeline_threshold', String(pipelineThreshold))}
				disabled={similaritySaving}
			>
				{t('common.save')}
			</button>
		</div>

		<div class="flex items-center gap-3">
			<div class="flex flex-col gap-0.5 flex-1">
				<span class="text-sm font-medium">{t('settings.pipelineTopK')}</span>
				<span class="text-xs text-base-content/35">{t('settings.pipelineTopKDesc')}</span>
			</div>
			<input
				type="number"
				class="input input-sm w-24 bg-white/[0.12] border-white/[0.18] text-right"
				step="1"
				min="1"
				max="20"
				bind:value={pipelineTopK}
			/>
			<button
				class="btn btn-sm btn-primary"
				onclick={() => handleSaveSimilarity('pipeline_top_k', String(pipelineTopK))}
				disabled={similaritySaving}
			>
				{t('common.save')}
			</button>
		</div>

		<button
			class="btn btn-sm btn-outline btn-ghost"
			onclick={handleResetSimilarityDefaults}
			disabled={similaritySaving}
		>
			{t('settings.resetDefaults')}
		</button>
	</section>

	<!-- AI Provider -->
	<section class="border border-base-300 rounded-lg p-5 flex flex-col gap-5 relative">
		<div class="flex items-center justify-between">
			<h2 class="text-xs font-medium text-base-content/40 uppercase tracking-wider">{t('settings.aiProvider')}</h2>
			<button
				class="btn btn-ghost btn-xs btn-circle text-warning"
				aria-label={t('settings.providerWarningTitle')}
				onclick={() => (providerWarningOpen = true)}
			>
				<svg xmlns="http://www.w3.org/2000/svg" class="size-4" viewBox="0 0 20 20" fill="currentColor">
					<path fill-rule="evenodd" d="M8.485 2.495c.673-1.167 2.357-1.167 3.03 0l6.28 10.875c.673 1.167-.17 2.625-1.516 2.625H3.72c-1.347 0-2.189-1.458-1.515-2.625L8.485 2.495zM10 6a.75.75 0 01.75.75v3.5a.75.75 0 01-1.5 0v-3.5A.75.75 0 0110 6zm0 9a1 1 0 100-2 1 1 0 000 2z" clip-rule="evenodd" />
				</svg>
			</button>
		</div>
		<p class="text-xs text-base-content/35">{t('settings.aiProviderDesc')}</p>

		<div class="flex items-center justify-between">
			<span class="text-sm font-medium">{t('settings.provider')}</span>
			<div class="join">
				{#each providers as prov}
					<button
						class="btn btn-sm join-item"
						class:btn-primary={embeddingProvider === prov.value}
						onclick={() => handleSwitchProvider(prov.value)}
						disabled={providerSwitching}
					>
						{#if providerSwitching && embeddingProvider !== prov.value}
							<span class="loading loading-spinner loading-xs"></span>
						{/if}
						{t(prov.labelKey)}
					</button>
				{/each}
			</div>
		</div>

		<!-- API Key -->
		<div class="flex flex-col gap-2">
			<span class="text-sm font-medium">{t('settings.apiKey')}</span>
			{#if apiKeyEditing}
				<div class="flex gap-2">
					<input
						type="password"
						class="input input-sm flex-1 bg-white/[0.12] border-white/[0.18]"
						placeholder={apiKeyPlaceholder}
						bind:value={apiKey}
					/>
					<button
						class="btn btn-sm btn-primary"
						onclick={handleSaveApiKey}
						disabled={apiKeySaving || !apiKey.trim()}
					>
						{t('common.save')}
					</button>
					{#if apiKeySavedValue}
						<button
							class="btn btn-sm btn-ghost"
							onclick={() => { apiKey = apiKeySavedValue; apiKeyEditing = false; }}
						>
							{t('common.cancel')}
						</button>
					{/if}
				</div>
			{:else}
				<div class="flex items-center gap-2">
					<span class="text-sm text-base-content/50 font-mono flex-1">{maskApiKey(apiKey)}</span>
					<button
						class="btn btn-sm btn-ghost"
						onclick={() => { apiKey = ''; apiKeyEditing = true; }}
					>
						{t('common.edit')}
					</button>
				</div>
			{/if}
		</div>

		<!-- Embedding Model -->
		<div class="flex flex-col gap-2">
			<span class="text-sm font-medium">{t('settings.embeddingModel')}</span>
			<div class="flex gap-2">
				<select
					class="select select-sm flex-1 bg-white/[0.12] border-white/[0.18]"
					value={embeddingModelCustom ? '__custom__' : embeddingModel}
					onchange={(e) => {
						const val = (e.target as HTMLSelectElement).value;
						if (val === '__custom__') {
							embeddingModelCustom = true;
							customEmbeddingModel = '';
						} else {
							embeddingModelCustom = false;
							handleSwitchEmbeddingModel(val);
						}
					}}
					disabled={modelSwitching || providerSwitching}
				>
					{#each embeddingModelPresets[embeddingProvider] || [] as model}
						<option value={model}>{model}</option>
					{/each}
					<option value="__custom__">{t('settings.modelCustom')}</option>
				</select>
			</div>
			{#if embeddingModelCustom}
				<div class="flex gap-2">
					<input
						type="text"
						class="input input-sm flex-1 bg-white/[0.12] border-white/[0.18]"
						placeholder={t('settings.modelCustom')}
						bind:value={customEmbeddingModel}
					/>
					<button
						class="btn btn-sm btn-primary"
						onclick={() => {
							if (customEmbeddingModel.trim()) handleSwitchEmbeddingModel(customEmbeddingModel.trim());
						}}
						disabled={modelSwitching || !customEmbeddingModel.trim()}
					>
						{t('common.save')}
					</button>
				</div>
			{/if}

			{#if progressPhase === 're_embed'}
				{@const msgKey = progressStep === 'saving' ? 'settings.progressReEmbedSaving' : 'settings.progressReEmbedApi'}
				<div class="flex flex-col gap-1.5">
					<div class="flex items-center gap-2 text-xs text-info">
						<span class="loading loading-spinner loading-xs"></span>
						<span>{t(msgKey)}</span>
						<span class="ml-auto tabular-nums">{progressPercent}%</span>
					</div>
					{#if progressTotal > 0}
						<progress class="progress progress-info w-full" value={progressCurrent} max={progressTotal}></progress>
					{/if}
				</div>
			{/if}
		</div>

		<!-- Chat Model -->
		<div class="flex flex-col gap-2">
			<span class="text-sm font-medium">{t('settings.chatModel')}</span>
			<div class="flex gap-2">
				<select
					class="select select-sm flex-1 bg-white/[0.12] border-white/[0.18]"
					value={chatModelCustom ? '__custom__' : chatModel}
					onchange={(e) => {
						const val = (e.target as HTMLSelectElement).value;
						if (val === '__custom__') {
							chatModelCustom = true;
							customChatModel = '';
						} else {
							chatModelCustom = false;
							handleSwitchChatModel(val);
						}
					}}
					disabled={modelSwitching || providerSwitching}
				>
					{#each chatModelPresets[embeddingProvider] || [] as model}
						<option value={model}>{model}</option>
					{/each}
					<option value="__custom__">{t('settings.modelCustom')}</option>
				</select>
			</div>
			{#if chatModelCustom}
				<div class="flex gap-2">
					<input
						type="text"
						class="input input-sm flex-1 bg-white/[0.12] border-white/[0.18]"
						placeholder={t('settings.modelCustom')}
						bind:value={customChatModel}
					/>
					<button
						class="btn btn-sm btn-primary"
						onclick={() => {
							if (customChatModel.trim()) handleSwitchChatModel(customChatModel.trim());
						}}
						disabled={modelSwitching || !customChatModel.trim()}
					>
						{t('common.save')}
					</button>
				</div>
			{/if}
		</div>

		<button
			class="btn btn-sm btn-outline btn-warning"
			onclick={handleReextract}
			disabled={reextractRunning || pipelineRunning || pipelineStatus?.running}
		>
			{#if reextractRunning}
				<span class="loading loading-spinner loading-xs"></span>
				{t('settings.reextractRunning')}
			{:else}
				{t('settings.reextractBtn')}
			{/if}
		</button>

		{#if progressPhase === 're_extract'}
			{@const msgKey = progressStep === 'similarity' ? 'settings.progressReExtractSimilarity'
				: progressStep === 'api' ? 'settings.progressReExtractApi'
				: 'settings.progressReExtractSaving'}
			<div class="flex flex-col gap-1.5">
				<div class="flex items-center gap-2 text-xs text-info">
					<span class="loading loading-spinner loading-xs"></span>
					<span>{t(msgKey)}</span>
					<span class="ml-auto tabular-nums">{progressPercent}%</span>
				</div>
				{#if progressTotal > 0}
					<progress class="progress progress-info w-full" value={progressCurrent} max={progressTotal}></progress>
				{/if}
			</div>
		{/if}
	</section>
</div>

<!-- Provider Warning Modal -->
{#if providerWarningOpen}
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<div class="modal modal-open" onkeydown={(e) => e.key === 'Escape' && (providerWarningOpen = false)}>
		<div class="modal-box border border-base-300">
			<h3 class="text-lg font-bold flex items-center gap-2">
				<svg xmlns="http://www.w3.org/2000/svg" class="size-5 text-warning" viewBox="0 0 20 20" fill="currentColor">
					<path fill-rule="evenodd" d="M8.485 2.495c.673-1.167 2.357-1.167 3.03 0l6.28 10.875c.673 1.167-.17 2.625-1.516 2.625H3.72c-1.347 0-2.189-1.458-1.515-2.625L8.485 2.495zM10 6a.75.75 0 01.75.75v3.5a.75.75 0 01-1.5 0v-3.5A.75.75 0 0110 6zm0 9a1 1 0 100-2 1 1 0 000 2z" clip-rule="evenodd" />
				</svg>
				{t('settings.providerWarningTitle')}
			</h3>
			<ul class="py-4 space-y-2 text-sm">
				<li class="flex gap-2">
					<span class="text-warning font-bold">&#8226;</span>
					{t('settings.providerWarning1')}
				</li>
				<li class="flex gap-2">
					<span class="text-warning font-bold">&#8226;</span>
					{t('settings.providerWarning2')}
				</li>
				<li class="flex gap-2">
					<span class="text-success font-bold">&#8226;</span>
					{t('settings.providerWarning3')}
				</li>
			</ul>
			<div class="modal-action">
				<button class="btn btn-sm btn-primary" onclick={() => (providerWarningOpen = false)}>
					{t('settings.providerWarningClose')}
				</button>
			</div>
		</div>
		<!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
		<div class="modal-backdrop" onclick={() => (providerWarningOpen = false)}></div>
	</div>
{/if}

<!-- Pipeline Warning Modal -->
{#if pipelineWarningOpen}
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<div class="modal modal-open" onkeydown={(e) => e.key === 'Escape' && (pipelineWarningOpen = false)}>
		<div class="modal-box border border-base-300">
			<h3 class="text-lg font-bold flex items-center gap-2">
				<svg xmlns="http://www.w3.org/2000/svg" class="size-5 text-warning" viewBox="0 0 20 20" fill="currentColor">
					<path fill-rule="evenodd" d="M8.485 2.495c.673-1.167 2.357-1.167 3.03 0l6.28 10.875c.673 1.167-.17 2.625-1.516 2.625H3.72c-1.347 0-2.189-1.458-1.515-2.625L8.485 2.495zM10 6a.75.75 0 01.75.75v3.5a.75.75 0 01-1.5 0v-3.5A.75.75 0 0110 6zm0 9a1 1 0 100-2 1 1 0 000 2z" clip-rule="evenodd" />
				</svg>
				{t('settings.pipelineWarningTitle')}
			</h3>
			<ul class="py-4 space-y-2 text-sm">
				<li class="flex gap-2">
					<span class="text-info font-bold">&#8226;</span>
					{t('settings.pipelineWarning1')}
				</li>
				<li class="flex gap-2">
					<span class="text-info font-bold">&#8226;</span>
					{t('settings.pipelineWarning2')}
				</li>
				<li class="flex gap-2">
					<span class="text-info font-bold">&#8226;</span>
					{t('settings.pipelineWarning3')}
				</li>
				<li class="flex gap-2">
					<span class="text-warning font-bold">&#8226;</span>
					{t('settings.pipelineWarning4')}
				</li>
				<li class="flex gap-2">
					<span class="text-warning font-bold">&#8226;</span>
					{t('settings.similarityWarning1')}
				</li>
				<li class="flex gap-2">
					<span class="text-warning font-bold">&#8226;</span>
					{t('settings.similarityWarning2')}
				</li>
				<li class="flex gap-2">
					<span class="text-info font-bold">&#8226;</span>
					{t('settings.similarityWarning4')}
				</li>
				<li class="flex gap-2">
					<span class="text-success font-bold">&#8226;</span>
					{t('settings.similarityWarning5')}
				</li>
			</ul>
			<div class="modal-action">
				<button class="btn btn-sm btn-primary" onclick={() => (pipelineWarningOpen = false)}>
					{t('settings.providerWarningClose')}
				</button>
			</div>
		</div>
		<!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
		<div class="modal-backdrop" onclick={() => (pipelineWarningOpen = false)}></div>
	</div>
{/if}

<!-- Re-embed Confirmation Modal -->
{#if reembedConfirmOpen}
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<div class="modal modal-open" onkeydown={(e) => e.key === 'Escape' && (reembedConfirmOpen = false)}>
		<div class="modal-box border border-base-300">
			<h3 class="text-lg font-bold flex items-center gap-2">
				<svg xmlns="http://www.w3.org/2000/svg" class="size-5 text-warning" viewBox="0 0 20 20" fill="currentColor">
					<path fill-rule="evenodd" d="M8.485 2.495c.673-1.167 2.357-1.167 3.03 0l6.28 10.875c.673 1.167-.17 2.625-1.516 2.625H3.72c-1.347 0-2.189-1.458-1.515-2.625L8.485 2.495zM10 6a.75.75 0 01.75.75v3.5a.75.75 0 01-1.5 0v-3.5A.75.75 0 0110 6zm0 9a1 1 0 100-2 1 1 0 000 2z" clip-rule="evenodd" />
				</svg>
				{t('settings.confirmReembed')}
			</h3>
			<ul class="py-4 space-y-2 text-sm">
				<li class="flex gap-2">
					<span class="text-warning font-bold">&#8226;</span>
					{t('settings.confirmReembedMsg1')}
				</li>
				<li class="flex gap-2">
					<span class="text-warning font-bold">&#8226;</span>
					{t('settings.confirmReembedMsg2')}
				</li>
				<li class="flex gap-2">
					<span class="text-success font-bold">&#8226;</span>
					{t('settings.confirmReembedMsg3')}
				</li>
			</ul>
			<div class="modal-action">
				<button class="btn btn-sm btn-ghost" onclick={() => { reembedConfirmOpen = false; reembedPendingAction = null; }}>
					{t('common.cancel')}
				</button>
				<button class="btn btn-sm btn-warning" onclick={handleConfirmReembed}>
					{t('settings.confirmReembedProceed')}
				</button>
			</div>
		</div>
		<!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
		<div class="modal-backdrop" onclick={() => { reembedConfirmOpen = false; reembedPendingAction = null; }}></div>
	</div>
{/if}

<!-- Re-extract Confirmation Modal -->
{#if reextractConfirmOpen}
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<div class="modal modal-open" onkeydown={(e) => e.key === 'Escape' && (reextractConfirmOpen = false)}>
		<div class="modal-box border border-base-300">
			<h3 class="text-lg font-bold flex items-center gap-2">
				<svg xmlns="http://www.w3.org/2000/svg" class="size-5 text-warning" viewBox="0 0 20 20" fill="currentColor">
					<path fill-rule="evenodd" d="M8.485 2.495c.673-1.167 2.357-1.167 3.03 0l6.28 10.875c.673 1.167-.17 2.625-1.516 2.625H3.72c-1.347 0-2.189-1.458-1.515-2.625L8.485 2.495zM10 6a.75.75 0 01.75.75v3.5a.75.75 0 01-1.5 0v-3.5A.75.75 0 0110 6zm0 9a1 1 0 100-2 1 1 0 000 2z" clip-rule="evenodd" />
				</svg>
				{t('settings.confirmReextract')}
			</h3>
			<ul class="py-4 space-y-2 text-sm">
				<li class="flex gap-2">
					<span class="text-warning font-bold">&#8226;</span>
					{t('settings.confirmReextractMsg1')}
				</li>
				<li class="flex gap-2">
					<span class="text-success font-bold">&#8226;</span>
					{t('settings.confirmReextractMsg2')}
				</li>
				<li class="flex gap-2">
					<span class="text-warning font-bold">&#8226;</span>
					{t('settings.confirmReextractMsg3')}
				</li>
				<li class="flex gap-2">
					<span class="text-warning font-bold">&#8226;</span>
					{t('settings.confirmReextractMsg4')}
				</li>
			</ul>
			<div class="modal-action">
				<button class="btn btn-sm btn-ghost" onclick={() => (reextractConfirmOpen = false)}>
					{t('common.cancel')}
				</button>
				<button class="btn btn-sm btn-warning" onclick={handleConfirmReextract}>
					{t('settings.confirmReextractProceed')}
				</button>
			</div>
		</div>
		<!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
		<div class="modal-backdrop" onclick={() => (reextractConfirmOpen = false)}></div>
	</div>
{/if}
