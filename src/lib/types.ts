export type MashType = '결정' | '문제' | '인사이트' | '질문';

export enum MashStatus {
	MASH_TUN = 'MASH_TUN',
	ON_STILL = 'ON_STILL',
	DISTILLED = 'DISTILLED',
	JARRED = 'JARRED',
	RE_EMBED = 'RE_EMBED',
	RE_EXTRACT = 'RE_EXTRACT'
}

export interface Mash {
	id: string;
	type: MashType;
	summary: string;
	context: string;
	memo: string;
	status: MashStatus;
	createdAt: number;
	updatedAt: number;
}
