export type ToastType = 'error' | 'success' | 'info';

let message = $state('');
let type = $state<ToastType>('error');
let visible = $state(false);
let timer: ReturnType<typeof setTimeout> | null = null;

export function getToast() {
	return { message, type, visible };
}

export function showToast(msg: string, toastType: ToastType = 'error', duration = 4000) {
	if (timer) clearTimeout(timer);
	message = msg;
	type = toastType;
	visible = true;
	timer = setTimeout(() => {
		visible = false;
		timer = null;
	}, duration);
}

export function dismissToast() {
	if (timer) clearTimeout(timer);
	visible = false;
	timer = null;
}
