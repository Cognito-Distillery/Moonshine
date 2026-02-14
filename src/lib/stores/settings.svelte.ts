import type { DateFormatId, TimeFormatId } from '$lib/utils/datetime';

export type SidebarPosition = 'left' | 'right' | 'top' | 'bottom';
export type ViewMode = 'card' | 'list';

const STORAGE_KEY = 'moonshine-settings';

interface Settings {
	sidebarPosition: SidebarPosition;
	viewMode: ViewMode;
	dateFormat: DateFormatId;
	timeFormat: TimeFormatId;
}

const DEFAULTS: Settings = {
	sidebarPosition: 'left',
	viewMode: 'list',
	dateFormat: 'medium',
	timeFormat: '24h'
};

function loadSettings(): Settings {
	if (typeof localStorage === 'undefined') return { ...DEFAULTS };
	try {
		const raw = localStorage.getItem(STORAGE_KEY);
		if (raw) return { ...DEFAULTS, ...JSON.parse(raw) };
	} catch {
		// ignore
	}
	return { ...DEFAULTS };
}

function saveSettings(settings: Settings) {
	if (typeof localStorage === 'undefined') return;
	localStorage.setItem(STORAGE_KEY, JSON.stringify(settings));
}

const initial = loadSettings();

let sidebarPosition = $state<SidebarPosition>(initial.sidebarPosition);
let viewMode = $state<ViewMode>(initial.viewMode);
let dateFormat = $state<DateFormatId>(initial.dateFormat);
let timeFormat = $state<TimeFormatId>(initial.timeFormat);

function save() {
	saveSettings({ sidebarPosition, viewMode, dateFormat, timeFormat });
}

export function getSidebarPosition() {
	return sidebarPosition;
}

export function setSidebarPosition(pos: SidebarPosition) {
	sidebarPosition = pos;
	save();
}

export function isHorizontal() {
	return sidebarPosition === 'top' || sidebarPosition === 'bottom';
}

export function getViewMode() {
	return viewMode;
}

export function setViewMode(mode: ViewMode) {
	viewMode = mode;
	save();
}

export function getDateFormat(): DateFormatId {
	return dateFormat;
}

export function setDateFormat(fmt: DateFormatId) {
	dateFormat = fmt;
	save();
}

export function getTimeFormat(): TimeFormatId {
	return timeFormat;
}

export function setTimeFormat(fmt: TimeFormatId) {
	timeFormat = fmt;
	save();
}
