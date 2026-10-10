export function pickRandom<T>(items: readonly T[], exclude?: T): T | undefined {
	const pool = exclude === undefined ? items : items.filter((item) => item !== exclude);
	if (pool.length === 0) return undefined;
	return pool[Math.floor(Math.random() * pool.length)];
}
