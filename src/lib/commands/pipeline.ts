import { invoke } from '@tauri-apps/api/core';

export interface PipelineStatus {
	lastRun: number | null;
	nextRun: number | null;
	intervalMin: number;
	onStillCount: number;
	distilledCount: number;
	jarredCount: number;
	running: boolean;
}

export function triggerPipeline(): Promise<void> {
	return invoke('trigger_pipeline');
}

export function setPipelineInterval(minutes: number): Promise<void> {
	return invoke('set_pipeline_interval', { minutes });
}

export function getPipelineStatus(): Promise<PipelineStatus> {
	return invoke<PipelineStatus>('get_pipeline_status');
}
