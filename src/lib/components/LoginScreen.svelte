<script lang="ts">
	import { authStore } from '$lib/stores/auth.svelte';
	import { t } from '$lib/i18n/index.svelte';

	let password = $state('');

	async function handleLogin(e: Event) {
		e.preventDefault();
		if (!password.trim()) return;
		await authStore.login(password.trim());
	}
</script>

<div class="flex h-screen items-center justify-center">
	<div class="w-full max-w-sm flex flex-col gap-6 px-6">
		<div class="text-center flex flex-col items-center">
			<svg width="96" height="96" viewBox="0 0 200 200" fill="none" class="mb-4">
				<defs>
					<linearGradient id="liqLogin" x1="100" y1="180" x2="100" y2="80" gradientUnits="userSpaceOnUse">
						<stop offset="0%" stop-color="#C4D4E0" stop-opacity="0.3"/>
						<stop offset="100%" stop-color="#C4D4E0" stop-opacity="0.05"/>
					</linearGradient>
				</defs>
				<path d="M68 40 H132 V52 H68 Z" stroke="#C4D4E0" stroke-width="3" stroke-linecap="round" fill="none"/>
				<path d="M72 52 V60 C72 65, 65 70, 65 80 V160 A 20 20 0 0 0 85 180 H115 A 20 20 0 0 0 135 160 V80 C135 70, 128 65, 128 60 V52" stroke="#C4D4E0" stroke-width="3" stroke-linecap="round" fill="none"/>
				<path d="M72 60 H128" stroke="#C4D4E0" stroke-width="2" stroke-opacity="0.5"/>
				<path d="M72 68 H128" stroke="#C4D4E0" stroke-width="2" stroke-opacity="0.5"/>
				<path d="M68 115 Q 100 120, 132 115 V 160 A 17 17 0 0 1 115 177 H 85 A 17 17 0 0 1 68 160 Z" fill="url(#liqLogin)" stroke="none"/>
				<path d="M100 70 A 22 22 0 1 1 88 110 A 18 18 0 1 0 100 70 Z" fill="#E2E8F0"/>
				<path d="M130 85 V 105" stroke="#C4D4E0" stroke-width="2" stroke-opacity="0.3" stroke-linecap="round"/>
				<path d="M70 160 V 170" stroke="#C4D4E0" stroke-width="2" stroke-opacity="0.3" stroke-linecap="round"/>
			</svg>
			<h1 class="text-2xl font-bold tracking-tight">Moonshine</h1>
			<p class="text-sm text-base-content/40 mt-1">{t('login.title')}</p>
		</div>

		<form class="flex flex-col gap-3" onsubmit={handleLogin}>
			<input
				type="password"
				class="input w-full bg-white/[0.12] border-white/[0.18] focus:border-primary placeholder:text-base-content/30"
				placeholder={t('login.password')}
				bind:value={password}
				required
			/>
			<button
				type="submit"
				class="btn btn-primary w-full"
				disabled={!password.trim()}
			>
				{t('login.enter')}
			</button>
		</form>

		{#if authStore.error}
			<p class="text-sm text-error text-center">
				{authStore.error.startsWith('login.') ? t(authStore.error as 'login.wrongPassword') : authStore.error}
			</p>
		{/if}
	</div>
</div>
