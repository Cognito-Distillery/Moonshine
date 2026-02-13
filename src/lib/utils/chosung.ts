const CHOSUNG = [
	'ㄱ', 'ㄲ', 'ㄴ', 'ㄷ', 'ㄸ', 'ㄹ', 'ㅁ', 'ㅂ', 'ㅃ', 'ㅅ',
	'ㅆ', 'ㅇ', 'ㅈ', 'ㅉ', 'ㅊ', 'ㅋ', 'ㅌ', 'ㅍ', 'ㅎ'
];

const CHOSUNG_SET = new Set(CHOSUNG);

/** 완성형 한글에서 초성 추출 (비한글은 그대로 반환) */
function getChosung(char: string): string {
	const code = char.charCodeAt(0);
	if (code >= 0xac00 && code <= 0xd7a3) {
		return CHOSUNG[Math.floor((code - 0xac00) / 588)];
	}
	return char;
}

/** 문자열에서 초성만 추출 */
function extractChosung(str: string): string {
	return [...str].map(getChosung).join('');
}

/** 검색어가 모두 초성인지 판별 */
function isChosungOnly(query: string): boolean {
	return [...query].every((ch) => CHOSUNG_SET.has(ch));
}

/** 초성 검색을 포함한 한글 매칭 */
export function matchKorean(text: string, query: string): boolean {
	const lower = query.toLowerCase();
	if (text.toLowerCase().includes(lower)) return true;
	if (isChosungOnly(query)) {
		return extractChosung(text).includes(query);
	}
	return false;
}
