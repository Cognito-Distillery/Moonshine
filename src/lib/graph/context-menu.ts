import { t } from '$lib/i18n/index.svelte';
import type { MessageKey } from '$lib/i18n/ko';

export interface ContextMenuItem {
	labelKey: MessageKey;
	action: string;
}

export const nodeMenuItems: ContextMenuItem[] = [
	{ labelKey: 'ctx.viewDetail', action: 'view-detail' },
	{ labelKey: 'ctx.editNode', action: 'edit-node' },
	{ labelKey: 'ctx.addEdge', action: 'add-edge' },
	{ labelKey: 'ctx.expand', action: 'expand' }
];

export const edgeMenuItems: ContextMenuItem[] = [
	{ labelKey: 'ctx.editEdge', action: 'edit-edge' },
	{ labelKey: 'ctx.deleteEdge', action: 'delete-edge' }
];

export const canvasMenuItems: ContextMenuItem[] = [
	{ labelKey: 'ctx.resetView', action: 'fit' }
];

export function getMenuLabel(item: ContextMenuItem): string {
	return t(item.labelKey);
}
