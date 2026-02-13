export type SidePanelMode = 'filter' | 'detail' | 'edit-node' | 'edit-edge' | 'add-edge';

function createUIStore() {
	let sidePanelOpen = $state(true);
	let sidePanelMode = $state<SidePanelMode>('filter');
	let isLoading = $state(false);
	let confirmDialog = $state<{ message: string; onConfirm: () => void } | null>(null);
	let addEdgeSourceId = $state<string | null>(null);

	return {
		get sidePanelOpen() { return sidePanelOpen; },
		get sidePanelMode() { return sidePanelMode; },
		get isLoading() { return isLoading; },
		get confirmDialog() { return confirmDialog; },
		get addEdgeSourceId() { return addEdgeSourceId; },

		setSidePanelMode(mode: SidePanelMode) {
			sidePanelMode = mode;
			sidePanelOpen = true;
		},
		toggleSidePanel() {
			sidePanelOpen = !sidePanelOpen;
		},
		setLoading(val: boolean) {
			isLoading = val;
		},
		showConfirm(message: string, onConfirm: () => void) {
			confirmDialog = { message, onConfirm };
		},
		dismissConfirm() {
			confirmDialog = null;
		},
		startAddEdge(sourceId: string) {
			addEdgeSourceId = sourceId;
			sidePanelMode = 'add-edge';
			sidePanelOpen = true;
		},
		cancelAddEdge() {
			addEdgeSourceId = null;
			sidePanelMode = 'filter';
		}
	};
}

export const uiStore = createUIStore();
