<script lang="ts">
	import { uiStore } from '$lib/stores/ui.svelte';
	import { t } from '$lib/i18n/index.svelte';
</script>

{#if uiStore.confirmDialog}
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<div class="modal modal-open" onkeydown={(e) => e.key === 'Escape' && uiStore.dismissConfirm()}>
		<div class="modal-box">
			<h3 class="text-lg font-bold">{t('confirm.title')}</h3>
			<p class="py-4">{uiStore.confirmDialog.message}</p>
			<div class="modal-action">
				<button class="btn" onclick={() => uiStore.dismissConfirm()}>{t('confirm.cancel')}</button>
				<button class="btn btn-error" onclick={() => uiStore.confirmDialog?.onConfirm()}>
					{t('confirm.delete')}
				</button>
			</div>
		</div>
		<!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
		<div class="modal-backdrop" onclick={() => uiStore.dismissConfirm()}></div>
	</div>
{/if}
