export type SidebarPosition = 'left' | 'right' | 'top' | 'bottom';
export type ViewMode = 'card' | 'list';

const STORAGE_KEY = 'moonshine-settings';

interface Settings {
	sidebarPosition: SidebarPosition;
	viewMode: ViewMode;
}

function loadSettings(): Settings {
	if (typeof localStorage === 'undefined') return { sidebarPosition: 'left', viewMode: 'list' };
	try {
		const raw = localStorage.getItem(STORAGE_KEY);
		if (raw) return JSON.parse(raw);
	} catch {
		// ignore
	}
	return { sidebarPosition: 'left', viewMode: 'list' };
}

function saveSettings(settings: Settings) {
	if (typeof localStorage === 'undefined') return;
	localStorage.setItem(STORAGE_KEY, JSON.stringify(settings));
}

const initial = loadSettings();

let sidebarPosition = $state<SidebarPosition>(initial.sidebarPosition);
let viewMode = $state<ViewMode>(initial.viewMode);

export function getSidebarPosition() {
	return sidebarPosition;
}

export function setSidebarPosition(pos: SidebarPosition) {
	sidebarPosition = pos;
	saveSettings({ sidebarPosition, viewMode });
}

export function isHorizontal() {
	return sidebarPosition === 'top' || sidebarPosition === 'bottom';
}

export function getViewMode() {
	return viewMode;
}

export function setViewMode(mode: ViewMode) {
	viewMode = mode;
	saveSettings({ sidebarPosition, viewMode });
}
